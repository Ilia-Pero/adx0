//! Turning blocking on and off.
//!
//! The hosts file is changed by this very program, started with the administrator's permission:
//! `adx0 --apply <folder of the program>` blocks the addresses, `adx0 --clear` opens them.

use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;

use adx0_core::{Failure, HostsFile};
use adx0_system::{Native, System};

use crate::home::Home;

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
        [flag, root] if flag == "--apply" => Some(code(apply(&Home::new(root.into())))),
        [flag] if flag == "--clear" => Some(code(clear())),
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
    let file = hosts()?;
    file.write(&file.read()?.cleared()?)?;
    forget();
    Ok(())
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
