//! The folder of the program: settings and downloaded lists.

use std::collections::BTreeSet;
use std::env;
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

    /// The settings. At first they follow the language chosen in the installer.
    pub fn settings(&self) -> SettingsFile {
        let installed = env::current_exe()
            .ok()
            .and_then(|program| program.parent().map(Language::installed))
            .unwrap_or_default();
        let spoken = if installed.is_empty() {
            Language::current().code
        } else {
            &installed
        };
        SettingsFile::new(
            self.root.join("settings.json"),
            Settings {
                language: installed.clone(),
                ..Settings::initial(spoken)
            },
        )
    }

    pub fn shelf(&self) -> Shelf {
        Shelf::new(self.root.join("lists"))
    }

    /// Copies the settings and addresses of the chosen lists to the other folder.
    pub fn copy(&self, vault: &Home) -> Result<(), Failure> {
        let settings = self.settings().read()?;
        vault.settings().write(&settings)?;
        SOURCES
            .iter()
            .filter(|source| settings.sources.iter().any(|id| id == source.id))
            .try_for_each(|source| self.shelf().copy(source, &vault.shelf()))
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
