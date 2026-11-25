//! What the program needs from the operating system: the hosts file, the administrator's
//! permission and forgetting remembered addresses.
//!
//! The crate does not depend on Tauri, so the compiler checks its macOS and Linux code on any
//! machine: `cargo clippy -p adx0-system --target x86_64-unknown-linux-gnu`.

use std::ffi::OsString;
use std::path::PathBuf;

use adx0_core::Failure;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod mac;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::Linux as Native;
#[cfg(target_os = "macos")]
pub use mac::Mac as Native;
#[cfg(target_os = "windows")]
pub use windows::Windows as Native;

/// An operating system.
pub trait System {
    /// Where the hosts file is.
    ///
    /// # Errors
    ///
    /// The system has not said where its folder is.
    fn hosts(&self) -> Result<PathBuf, Failure>;

    /// Starts this very program with the administrator's permission and waits until it is done.
    ///
    /// # Errors
    ///
    /// The permission was not given, or the program has failed.
    fn elevate(&self, arguments: &[OsString]) -> Result<(), Failure>;

    /// The folder where only the administrator can write. Nothing if the program itself lies where
    /// anybody can: then nothing may be started by itself with the administrator's permission.
    fn vault(&self) -> Option<PathBuf>;

    /// Makes the system start the program with the argument, with the administrator's permission,
    /// when the system starts. Needs the permission.
    ///
    /// # Errors
    ///
    /// The system has refused.
    fn settle(&self, argument: &str) -> Result<(), Failure>;

    /// Makes the system start the program no more. Needs the administrator's permission.
    fn unsettle(&self);

    /// Asks the system to forget remembered addresses of sites. A failure does no harm:
    /// the system forgets them by itself in a few minutes.
    fn forget(&self);
}
