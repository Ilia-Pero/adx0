//! The folder of the program: settings and downloaded lists.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use adx0_core::{Domain, Failure, Settings, SettingsFile, Shelf, Verdict, SOURCES};
use adx0_phrase::Language;

/// The folder of the program on the disk.
pub struct Home {
    root: PathBuf,
}

impl Home {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn path(&self) -> &Path {
        &self.root
    }

    pub fn settings(&self) -> SettingsFile {
        SettingsFile::new(
            self.root.join("settings.json"),
            Settings::initial(Language::current().code),
        )
    }

    pub fn shelf(&self) -> Shelf {
        Shelf::new(self.root.join("lists"))
    }

    /// Addresses to block. An empty set is a mistake of a person and not a wish to turn off.
    pub fn domains(&self) -> Result<BTreeSet<Domain>, Failure> {
        let domains = Verdict::new(&self.settings().read()?, &self.shelf(), SOURCES).domains()?;
        if domains.is_empty() {
            Err(Failure::Empty)
        } else {
            Ok(domains)
        }
    }
}
