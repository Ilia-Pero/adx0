//! Windows.

#![allow(unsafe_code)]

use std::ffi::{OsStr, OsString};
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::ptr::null_mut;

use adx0_core::Failure;
use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, ERROR_CANCELLED};
use windows_sys::Win32::System::Com::CoTaskMemFree;
use windows_sys::Win32::System::SystemInformation::GetSystemDirectoryW;
use windows_sys::Win32::System::Threading::{
    GetExitCodeProcess, WaitForSingleObject, CREATE_NO_WINDOW, INFINITE,
};
use windows_sys::Win32::UI::Shell::{
    FOLDERID_ProgramFiles, SHGetKnownFolderPath, ShellExecuteExW, SEE_MASK_NOCLOSEPROCESS,
    SHELLEXECUTEINFOW,
};
use windows_sys::Win32::UI::WindowsAndMessaging::SW_HIDE;

use crate::System;

/// No path of Windows is longer.
const LONGEST: usize = 32_767;

/// The name of the task in the scheduler.
const TASK: &str = "adx0";

/// Windows.
pub struct Windows;

impl Windows {
    /// The folder `System32`. The system is asked for it, and not an environment variable: any
    /// program can change the variable, and a part of the work needs the administrator's
    /// permission.
    fn folder() -> Result<PathBuf, Failure> {
        let mut name = [0_u16; 260];
        // SAFETY: the buffer holds as many characters as its length says.
        let length = unsafe { GetSystemDirectoryW(name.as_mut_ptr(), 260) } as usize;
        if length == 0 || length >= name.len() {
            return Err(Failure::Disk("System32".to_owned()));
        }
        Ok(PathBuf::from(OsString::from_wide(&name[..length])))
    }

    /// The folder `Program Files`. The system is asked for it, see above.
    fn programs() -> Option<PathBuf> {
        let mut name = null_mut();
        // SAFETY: the system allocates the name, and it is freed once, whatever the answer is.
        unsafe {
            let answer = SHGetKnownFolderPath(&FOLDERID_ProgramFiles, 0, null_mut(), &raw mut name);
            let found = (answer == 0 && !name.is_null()).then(|| {
                let length = (0..LONGEST)
                    .take_while(|index| *name.add(*index) != 0)
                    .count();
                PathBuf::from(OsString::from_wide(std::slice::from_raw_parts(
                    name, length,
                )))
            });
            CoTaskMemFree(name.cast());
            found
        }
    }

    /// The scheduler.
    fn scheduler() -> Result<Command, Failure> {
        let mut scheduler = Command::new(Self::folder()?.join("schtasks.exe"));
        scheduler
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        Ok(scheduler)
    }
}

impl System for Windows {
    fn hosts(&self) -> Result<PathBuf, Failure> {
        Ok(Self::folder()?.join("drivers").join("etc").join("hosts"))
    }

    fn elevate(&self, arguments: &[OsString]) -> Result<(), Failure> {
        let program = wide(std::env::current_exe()?.as_os_str());
        let verb = wide(OsStr::new("runas"));
        let line = wide(&joined(arguments));
        // SAFETY: the strings end with zero and outlive the call. The handle of the process,
        // if there is one, is closed once.
        let (started, reason, code) = unsafe {
            let mut info: SHELLEXECUTEINFOW = zeroed();
            info.cbSize = u32::try_from(size_of::<SHELLEXECUTEINFOW>()).unwrap_or_default();
            info.fMask = SEE_MASK_NOCLOSEPROCESS;
            info.lpVerb = verb.as_ptr();
            info.lpFile = program.as_ptr();
            info.lpParameters = line.as_ptr();
            info.nShow = SW_HIDE;
            let started = ShellExecuteExW(&raw mut info) != 0;
            let reason = GetLastError();
            let mut code = 0;
            if started && !info.hProcess.is_null() {
                WaitForSingleObject(info.hProcess, INFINITE);
                GetExitCodeProcess(info.hProcess, &raw mut code);
                CloseHandle(info.hProcess);
            }
            (started, reason, code)
        };
        if !started {
            return Err(if reason == ERROR_CANCELLED {
                Failure::Refused
            } else {
                Failure::Closed(reason.to_string())
            });
        }
        match i32::try_from(code) {
            Ok(0) => Ok(()),
            Ok(code) => Err(Failure::from_code(code)),
            Err(_) => Err(Failure::Closed(code.to_string())),
        }
    }

    fn vault(&self) -> Option<PathBuf> {
        let program = std::env::current_exe().ok()?;
        program
            .starts_with(Self::programs()?)
            .then(|| program.with_file_name("kept"))
    }

    /// The task belongs to the system itself, so it has no window and asks for nothing. It waits
    /// for two minutes: there is enough to do when the system starts.
    fn settle(&self, argument: &str) -> Result<(), Failure> {
        let program = std::env::current_exe()?;
        let status = Self::scheduler()?
            .args([
                "/Create", "/F", "/TN", TASK, "/RU", "SYSTEM", "/RL", "HIGHEST",
            ])
            .args(["/SC", "ONLOGON", "/DELAY", "0002:00", "/TR"])
            .arg(format!("\"{}\" {argument}", program.display()))
            .status()?;
        if status.success() {
            Ok(())
        } else {
            Err(Failure::Disk("schtasks".to_owned()))
        }
    }

    fn unsettle(&self) {
        if let Ok(mut scheduler) = Self::scheduler() {
            let _ = scheduler.args(["/Delete", "/F", "/TN", TASK]).status();
        }
    }

    fn browse(&self, url: &str) -> Result<(), Failure> {
        Ok(Command::new(Self::folder()?.join("rundll32.exe"))
            .args(["url.dll,FileProtocolHandler", url])
            .status()
            .map(|_| ())?)
    }

    fn forget(&self) {
        if let Ok(folder) = Self::folder() {
            let _ = Command::new(folder.join("ipconfig.exe"))
                .arg("/flushdns")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
    }
}

/// The text the way Windows takes it: UTF-16 with zero at the end.
fn wide(text: &OsStr) -> Vec<u16> {
    text.encode_wide().chain([0]).collect()
}

/// Arguments as one line, each in quotes. Windows does not allow quotes inside paths, and
/// a backslash before the closing quote is doubled, as the rules of parsing require.
fn joined(arguments: &[OsString]) -> OsString {
    let mut line = OsString::new();
    for argument in arguments {
        let text = argument.to_string_lossy();
        let slashes = text.len() - text.trim_end_matches('\\').len();
        line.push("\"");
        line.push(argument);
        line.push("\\".repeat(slashes));
        line.push("\" ");
    }
    line
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use super::{joined, Windows};
    use crate::System;

    #[test]
    fn keeps_hosts_in_system_folder() {
        assert!(Windows.hosts().unwrap().ends_with("drivers/etc/hosts"));
    }

    #[test]
    fn quotes_path_with_space() {
        assert_eq!(
            joined(&[OsString::from("--apply"), OsString::from("C:\\My dir")]),
            OsString::from("\"--apply\" \"C:\\My dir\" ")
        );
    }

    #[test]
    fn doubles_slash_before_closing_quote() {
        assert_eq!(
            joined(&[OsString::from("C:\\")]),
            OsString::from("\"C:\\\\\" ")
        );
    }
}
