//! Turning blocking on and off.
//!
//! The hosts file is changed by this very program, started with the administrator's permission:
//! `adx0 --apply <folder of the program>` blocks the addresses, `adx0 --clear` opens them, and
//! `adx0 --refresh`, which the system starts by itself, downloads the lists again.

use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use adx0_core::{Failure, HostsFile, Source, SOURCES};
use adx0_net::Http;
use adx0_system::{Native, System};

use crate::home::Home;

/// The argument the system starts the program with.
const REFRESH: &str = "--refresh";

/// The value of an environment variable. It works in debug builds only: in the real program it
/// would let anybody make the administrator change somebody else's file.
pub fn rehearsal(name: &str) -> Option<PathBuf> {
    if cfg!(debug_assertions) {
        env::var_os(name).map(PathBuf::from)
    } else {
        None
    }
}

/// The hosts file. In debug builds `ADX0_HOSTS` replaces it.
pub fn hosts() -> Result<HostsFile, Failure> {
    match rehearsal("ADX0_HOSTS") {
        Some(file) => Ok(HostsFile::new(file)),
        None => Ok(HostsFile::new(Native.hosts()?)),
    }
}

/// The folder of the part that starts by itself, if there is one.
pub fn kept() -> Option<Home> {
    Native.vault().map(Home::new)
}

/// Turns blocking on or off.
pub fn switch(home: &Home, on: bool) -> Result<(), Failure> {
    if rehearsal("ADX0_HOSTS").is_some() {
        return if on { apply(home) } else { clear() };
    }
    if on {
        // There is no need to ask for a password if there is nothing to block.
        home.domains()?;
        // The administrator may speak another language, for which other lists are on at first.
        home.settings().write(&home.settings().read()?)?;
        Native.elevate(&[
            OsString::from("--apply"),
            home.path().as_os_str().to_owned(),
        ])
    } else {
        Native.elevate(&[OsString::from("--clear")])
    }
}

/// Does what the arguments say, if they are meant for the part of the program that works with
/// the administrator's permission.
pub fn called(arguments: &[OsString]) -> Option<ExitCode> {
    match arguments {
        [flag, root] if flag == "--apply" => {
            let home = Home::new(root.into());
            Some(code(apply(&home).and_then(|()| stay(&home))))
        }
        [flag] if flag == "--clear" => Some(code(clear())),
        [flag] if flag == REFRESH => Some(code(refresh())),
        _ => None,
    }
}

/// Blocks the addresses in the hosts file.
fn apply(home: &Home) -> Result<(), Failure> {
    let file = hosts()?;
    file.write(&file.read()?.guarded(&home.domains()?)?)?;
    forget();
    Ok(())
}

/// Opens the addresses the program has blocked.
fn clear() -> Result<(), Failure> {
    leave();
    let file = hosts()?;
    file.write(&file.read()?.cleared()?)?;
    forget();
    Ok(())
}

/// Makes the system start the program by itself, if a person wants so. What starts by itself has
/// a copy of the choice in a folder where only the administrator can write, so no other program
/// can make it block something else.
fn stay(home: &Home) -> Result<(), Failure> {
    let Some(vault) = Native.vault() else {
        return Ok(());
    };
    leave();
    if home.settings().read()?.fresh {
        home.copy(&Home::new(vault))?;
        Native.settle(REFRESH)
    } else {
        Ok(())
    }
}

/// Makes the system start the program no more.
fn leave() {
    if let Some(vault) = Native.vault() {
        Native.unsettle();
        // There may be nothing to remove.
        let _ = fs::remove_dir_all(vault);
    }
}

/// Downloads the lists that are a day old, and blocks what they say. Does nothing if blocking is
/// off. A list that cannot be downloaded stays as it was.
fn refresh() -> Result<(), Failure> {
    let Some(vault) = Native.vault() else {
        return Ok(());
    };
    let home = Home::new(vault);
    let settings = home.settings().read()?;
    if !settings.fresh || hosts()?.read()?.count() == 0 {
        return Ok(());
    }
    let shelf = home.shelf();
    let stale = |source: &&Source| {
        settings.sources.iter().any(|id| id == source.id) && !shelf.fresh(source.id)
    };
    let renewed = SOURCES
        .iter()
        .filter(stale)
        .filter(|source| shelf.refresh(source, &Http).is_ok())
        .count();
    if renewed > 0 {
        apply(&home)
    } else {
        Ok(())
    }
}

fn forget() {
    if rehearsal("ADX0_HOSTS").is_none() {
        Native.forget();
    }
}

fn code(done: Result<(), Failure>) -> ExitCode {
    match done {
        Ok(()) => ExitCode::SUCCESS,
        Err(failure) => {
            eprintln!("{failure}");
            ExitCode::from(failure.code())
        }
    }
}
