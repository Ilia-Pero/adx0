//! Linux.

use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use adx0_core::Failure;

use crate::System;

/// Where the package puts the program. Only the administrator can write there.
const PROGRAM: &str = "/usr/bin/adx0";

/// What systemd starts, and when.
const SERVICE: &str = "/etc/systemd/system/adx0.service";
const TIMER: &str = "/etc/systemd/system/adx0.timer";

/// Linux.
///
/// @todo #1 Nobody has run this on Linux yet. Check on Ubuntu 26.04 that pkexec asks for the
///  password, that a refusal is understood and that /etc/hosts is written.
pub struct Linux;

impl Linux {
    /// Tells systemd to do something. False if it has not done it.
    fn told(words: &[&str]) -> bool {
        Command::new("/usr/bin/systemctl")
            .args(words)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|status| status.success())
    }
}

impl System for Linux {
    fn hosts(&self) -> Result<PathBuf, Failure> {
        Ok(PathBuf::from("/etc/hosts"))
    }

    fn elevate(&self, arguments: &[OsString]) -> Result<(), Failure> {
        let status = Command::new("/usr/bin/pkexec")
            .arg(std::env::current_exe()?)
            .args(arguments)
            .status()?;
        match status.code() {
            Some(0) => Ok(()),
            // pkexec exits with 126 if the request was closed and with 127 if it was declined.
            Some(126 | 127) => Err(Failure::Refused),
            Some(code) => Err(Failure::from_code(code)),
            None => Err(Failure::Closed("pkexec".to_owned())),
        }
    }

    fn vault(&self) -> Option<PathBuf> {
        (std::env::current_exe().ok()?.as_os_str() == PROGRAM)
            .then(|| PathBuf::from("/var/lib/adx0"))
    }

    fn settle(&self, argument: &str) -> Result<(), Failure> {
        fs::write(
            SERVICE,
            format!(
                "[Unit]\nDescription=Fresh lists for adx0\nAfter=network-online.target\n\n\
                 [Service]\nType=oneshot\nNice=19\nExecStart={PROGRAM} {argument}\n"
            ),
        )?;
        fs::write(
            TIMER,
            "[Unit]\nDescription=Fresh lists for adx0\n\n\
             [Timer]\nOnBootSec=2min\nOnUnitActiveSec=1d\n\n\
             [Install]\nWantedBy=timers.target\n",
        )?;
        if Self::told(&["daemon-reload"]) && Self::told(&["enable", "--now", "adx0.timer"]) {
            Ok(())
        } else {
            Err(Failure::Disk("systemctl".to_owned()))
        }
    }

    fn unsettle(&self) {
        // There is nothing to stop if there is no such timer.
        Self::told(&["disable", "--now", "adx0.timer"]);
        for file in [SERVICE, TIMER] {
            let _ = fs::remove_file(file);
        }
        Self::told(&["daemon-reload"]);
    }

    fn forget(&self) {
        let _ = Command::new("/usr/bin/resolvectl")
            .arg("flush-caches")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}
