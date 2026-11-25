//! An ad blocker that does not run: it writes ad addresses into the hosts file and exits.
//! While its window is closed, there is no process and no load.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod guard;
mod home;

use std::env;
use std::ffi::OsString;
use std::process::ExitCode;

fn main() -> ExitCode {
    let arguments: Vec<OsString> = env::args_os().skip(1).collect();
    if let Some(code) = guard::called(&arguments) {
        return code;
    }
    let ran = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::state,
            commands::save,
            commands::refresh,
            commands::turn,
            commands::update,
            commands::download,
            commands::news
        ])
        .run(tauri::generate_context!());
    match ran {
        Ok(()) => ExitCode::SUCCESS,
        Err(failure) => {
            eprintln!("{failure}");
            ExitCode::FAILURE
        }
    }
}
