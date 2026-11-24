//! Lists of ad addresses and the section with them in the hosts file.
//!
//! The crate does not depend on Tauri and needs no administrator's permission: it is told where
//! the hosts file is, so everything is tested on temporary files.

mod domain;
mod failure;
mod file;
mod hosts;
mod list;
mod settings;
mod shelf;
mod source;
mod verdict;

pub use domain::{Address, Domain};
pub use failure::Failure;
pub use file::HostsFile;
pub use hosts::Hosts;
pub use list::{Blocklist, Format};
pub use settings::{Settings, SettingsFile};
pub use shelf::Shelf;
pub use source::{Source, Web, SOURCES};
pub use verdict::Verdict;
