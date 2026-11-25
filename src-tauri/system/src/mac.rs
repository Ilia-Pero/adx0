//! macOS.

use std::ffi::OsString;
use std::fs::{self, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
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

/// Where only the administrator can write.
const VAULT: &str = "/Library/Application Support/adx0";

/// What the system starts by itself: its name and its description.
const LABEL: &str = "io.github.ilia-pero.adx0";
const DAEMON: &str = "/Library/LaunchDaemons/io.github.ilia-pero.adx0.plist";

/// How often the system starts it, in seconds: once a day.
const DAY: u32 = 86_400;

/// macOS.
///
/// @todo #1 Nobody has run this on a Mac yet. Check on High Sierra and on the current macOS that
///  the password is asked for, that a refusal is understood, that /etc/hosts is written and that
///  the exit code of the program reaches its window.
pub struct Mac;

impl Mac {
    /// Tells the system to do something with what it starts. False if it has not done it.
    fn told(words: &[&str]) -> bool {
        Command::new("/bin/launchctl")
            .args(words)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|status| status.success())
    }
}

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

    fn vault(&self) -> Option<PathBuf> {
        Some(PathBuf::from(VAULT))
    }

    /// The system starts a copy of the program: anybody can replace what lies in `/Applications`.
    fn settle(&self, argument: &str) -> Result<(), Failure> {
        let program = Path::new(VAULT).join("adx0");
        let own = std::env::current_exe()?;
        if own != program {
            fs::create_dir_all(VAULT)?;
            // The old copy may be at work: it is replaced, and not written over.
            let _ = fs::remove_file(&program);
            fs::copy(own, &program)?;
            fs::set_permissions(&program, Permissions::from_mode(0o755))?;
        }
        fs::write(
            DAEMON,
            format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
                 <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \
                 \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
                 <plist version=\"1.0\">\n<dict>\n\
                 <key>Label</key><string>{LABEL}</string>\n\
                 <key>ProgramArguments</key><array><string>{}</string>\
                 <string>{argument}</string></array>\n\
                 <key>RunAtLoad</key><true/>\n\
                 <key>StartInterval</key><integer>{DAY}</integer>\n\
                 <key>ProcessType</key><string>Background</string>\n\
                 </dict>\n</plist>\n",
                program.display()
            ),
        )?;
        fs::set_permissions(DAEMON, Permissions::from_mode(0o644))?;
        // There is nothing to end if it has not been started before.
        Self::told(&["bootout", &format!("system/{LABEL}")]);
        if Self::told(&["bootstrap", "system", DAEMON]) {
            Ok(())
        } else {
            Err(Failure::Disk("launchctl".to_owned()))
        }
    }

    fn unsettle(&self) {
        Self::told(&["bootout", &format!("system/{LABEL}")]);
        let _ = fs::remove_file(DAEMON);
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
