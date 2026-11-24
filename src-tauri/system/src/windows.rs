//! Windows.

#![allow(unsafe_code)]

use std::ffi::{OsStr, OsString};
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use adx0_core::Failure;
use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, ERROR_CANCELLED};
use windows_sys::Win32::System::SystemInformation::GetSystemDirectoryW;
use windows_sys::Win32::System::Threading::{GetExitCodeProcess, WaitForSingleObject, INFINITE};
use windows_sys::Win32::UI::Shell::{ShellExecuteExW, SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW};
use windows_sys::Win32::UI::WindowsAndMessaging::SW_HIDE;

use crate::System;

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
