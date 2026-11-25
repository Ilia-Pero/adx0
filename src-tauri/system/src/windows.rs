//! Windows.

#![allow(unsafe_code)]

use std::ffi::{OsStr, OsString};
use std::fs;
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
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

    /// A program of the system that works without a window.
    fn silent(name: &str) -> Result<Command, Failure> {
        let mut program = Command::new(Self::folder()?.join(name));
        program
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        Ok(program)
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

    fn settle(&self, argument: &str) -> Result<(), Failure> {
        let vault = self
            .vault()
            .ok_or_else(|| Failure::Disk("Program Files".to_owned()))?;
        let file = vault.join("task.xml");
        let text = task(&std::env::current_exe()?, argument);
        fs::create_dir_all(&vault)?;
        fs::write(&file, encoded(&text))?;
        let status = Self::silent("schtasks.exe")?
            .args(["/Create", "/F", "/TN", TASK, "/RU", "SYSTEM", "/XML"])
            .arg(&file)
            .status();
        // The scheduler has read it already.
        let _ = fs::remove_file(&file);
        if status?.success() {
            Ok(())
        } else {
            Err(Failure::Disk("schtasks".to_owned()))
        }
    }

    fn unsettle(&self) {
        if let Ok(mut scheduler) = Self::silent("schtasks.exe") {
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
        if let Ok(mut configuration) = Self::silent("ipconfig.exe") {
            let _ = configuration.arg("/flushdns").status();
        }
    }
}

/// What the scheduler has to do. The task belongs to the system itself, so it has no window and asks
/// for nothing. It waits for two minutes: there is enough to do when the system starts. A task
/// works on batteries only if it is told so, and most laptops are on batteries.
fn task(program: &Path, argument: &str) -> String {
    let program = program.display().to_string().replace('&', "&amp;");
    format!(
        r#"<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.2" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
  <RegistrationInfo>
    <Description>Fresh lists for adx0</Description>
  </RegistrationInfo>
  <Triggers>
    <LogonTrigger>
      <Delay>PT2M</Delay>
    </LogonTrigger>
  </Triggers>
  <Principals>
    <Principal id="System">
      <UserId>S-1-5-18</UserId>
      <RunLevel>HighestAvailable</RunLevel>
    </Principal>
  </Principals>
  <Settings>
    <DisallowStartIfOnBatteries>false</DisallowStartIfOnBatteries>
    <StopIfGoingOnBatteries>false</StopIfGoingOnBatteries>
    <ExecutionTimeLimit>PT15M</ExecutionTimeLimit>
  </Settings>
  <Actions Context="System">
    <Exec>
      <Command>"{program}"</Command>
      <Arguments>{argument}</Arguments>
    </Exec>
  </Actions>
</Task>
"#
    )
}

/// The text as the scheduler reads it: UTF-16 with a mark at the beginning.
fn encoded(text: &str) -> Vec<u8> {
    ['\u{feff}']
        .into_iter()
        .chain(text.chars())
        .flat_map(|sign| sign.encode_utf16(&mut [0; 2]).to_vec())
        .flat_map(u16::to_le_bytes)
        .collect()
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
    use std::path::Path;

    use super::{encoded, joined, task, Windows};
    use crate::System;

    #[test]
    fn keeps_hosts_in_system_folder() {
        assert!(Windows.hosts().unwrap().ends_with("drivers/etc/hosts"));
    }

    #[test]
    fn starts_the_program_by_itself() {
        assert!(task(Path::new("C:\\My dir\\adx0.exe"), "--refresh")
            .contains("<Command>\"C:\\My dir\\adx0.exe\"</Command>"));
    }

    #[test]
    fn keeps_a_sign_of_xml_in_the_path() {
        assert!(task(Path::new("C:\\A&B\\adx0.exe"), "--refresh").contains("C:\\A&amp;B\\adx0.exe"));
    }

    #[test]
    fn begins_the_task_with_a_mark() {
        assert_eq!(encoded("<"), [0xff, 0xfe, b'<', 0]);
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
