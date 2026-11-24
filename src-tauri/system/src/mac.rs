//! macOS.

use std::ffi::OsString;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use adx0_core::Failure;

use crate::System;

/// A script that puts a command together from the arguments and runs it with the administrator's
/// permission. `AppleScript` quotes the arguments by itself, so spaces and quotes are safe.
const SCRIPT: [&str; 7] = [
    "on run argv",
    "set wholeCommand to \"\"",
    "repeat with onePart in argv",
    "set wholeCommand to wholeCommand & quoted form of (onePart as text) & \" \"",
    "end repeat",
    "do shell script wholeCommand with administrator privileges",
    "end run",
];

/// macOS.
///
/// @todo #1 Nobody has run this on a Mac yet. Check on High Sierra and on the current macOS that
///  the password is asked for, that a refusal is understood, that /etc/hosts is written and that
///  the exit code of the program reaches its window.
pub struct Mac;

impl System for Mac {
    fn hosts(&self) -> Result<PathBuf, Failure> {
        Ok(PathBuf::from("/etc/hosts"))
    }

    fn elevate(&self, arguments: &[OsString]) -> Result<(), Failure> {
        let output = Command::new("/usr/bin/osascript")
            .args(SCRIPT.iter().flat_map(|line| ["-e", line]))
            .arg(std::env::current_exe()?)
            .args(arguments)
            .output()?;
        if output.status.success() {
            return Ok(());
        }
        Err(match code(&String::from_utf8_lossy(&output.stderr)) {
            Some(-128) => Failure::Refused,
            Some(code) => Failure::from_code(code),
            None => Failure::Closed("osascript".to_owned()),
        })
    }

    fn forget(&self) {
        for (program, arguments) in [
            ("/usr/bin/dscacheutil", ["-flushcache"].as_slice()),
            ("/usr/bin/killall", ["-HUP", "mDNSResponder"].as_slice()),
        ] {
            let _ = Command::new(program)
                .args(arguments)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
    }
}

/// The exit code in the message of osascript: it is in brackets at the very end.
fn code(message: &str) -> Option<i32> {
    message
        .trim_end()
        .strip_suffix(')')?
        .rsplit_once('(')?
        .1
        .parse()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::code;

    #[test]
    fn finds_code_at_the_end_of_message() {
        assert_eq!(code("0:73: execution error: refused (10)\n"), Some(10));
    }

    #[test]
    fn finds_code_of_cancelled_request() {
        assert_eq!(
            code("0:73: execution error: User canceled. (-128)\n"),
            Some(-128)
        );
    }

    #[test]
    fn finds_nothing_in_other_message() {
        assert_eq!(code("no such program"), None);
    }
}
