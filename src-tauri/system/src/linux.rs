//! Linux.

use std::ffi::OsString;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use adx0_core::Failure;

use crate::System;

/// Linux.
///
/// @todo #1 Nobody has run this on Linux yet. Check on Ubuntu 26.04 that pkexec asks for the
///  password, that a refusal is understood and that /etc/hosts is written.
pub struct Linux;

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

    fn forget(&self) {
        let _ = Command::new("/usr/bin/resolvectl")
            .arg("flush-caches")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}
