//! Settings of the program.

use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{Failure, SOURCES};

/// What a person has chosen.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    /// Names of the lists that are on.
    pub sources: Vec<String>,
    /// Sites blocked in addition to the lists.
    pub blocked: Vec<String>,
    /// Sites that must not be blocked.
    pub allowed: Vec<String>,
    /// The code of the language. Empty means the language of the system.
    #[serde(default)]
    pub language: String,
}

impl Settings {
    /// Settings until a person who speaks the language changes them.
    #[must_use]
    pub fn initial(language: &str) -> Settings {
        Settings {
            sources: SOURCES
                .iter()
                .filter(|source| source.suits(language))
                .map(|source| source.id.to_owned())
                .collect(),
            blocked: Vec::new(),
            allowed: Vec::new(),
            language: String::new(),
        }
    }
}

/// The file with the settings. Initial settings are in force while there is no file.
pub struct SettingsFile {
    file: PathBuf,
    initial: Settings,
}

impl SettingsFile {
    #[must_use]
    pub fn new(file: PathBuf, initial: Settings) -> Self {
        Self { file, initial }
    }

    /// Settings from the file.
    ///
    /// # Errors
    ///
    /// There is a file, but it cannot be read or understood.
    pub fn read(&self) -> Result<Settings, Failure> {
        match fs::read(&self.file) {
            Ok(json) => serde_json::from_slice(&json)
                .map_err(|failure| Failure::Disk(format!("{}: {failure}", self.file.display()))),
            Err(failure) if failure.kind() == ErrorKind::NotFound => Ok(self.initial.clone()),
            Err(failure) => Err(failure.into()),
        }
    }

    /// Writes the settings. The old file is replaced as a whole, so a crash in the middle
    /// does not spoil it.
    ///
    /// # Errors
    ///
    /// The file cannot be written.
    pub fn write(&self, settings: &Settings) -> Result<(), Failure> {
        if let Some(folder) = self.file.parent() {
            fs::create_dir_all(folder)?;
        }
        let draft = self.file.with_extension("new");
        fs::write(&draft, serde_json::to_vec_pretty(settings)?)?;
        fs::rename(&draft, &self.file)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::{Settings, SettingsFile};

    #[test]
    fn leaves_large_list_off_at_first() {
        assert!(!Settings::initial("ru")
            .sources
            .contains(&"stevenblack".to_owned()));
    }

    #[test]
    fn turns_list_of_own_language_on() {
        assert!(Settings::initial("fr")
            .sources
            .contains(&"listefr".to_owned()));
    }

    #[test]
    fn returns_what_was_written() {
        let root = TempDir::new().unwrap();
        let file = SettingsFile::new(
            root.path().join("deep").join("settings.json"),
            Settings::initial("en"),
        );
        let chosen = Settings {
            sources: vec!["yoyo".to_owned()],
            blocked: vec!["a.com".to_owned()],
            allowed: vec!["b.com".to_owned()],
            language: "el".to_owned(),
        };
        file.write(&chosen).unwrap();
        assert_eq!(file.read().unwrap(), chosen);
    }

    #[test]
    fn names_the_damaged_file() {
        let root = TempDir::new().unwrap();
        let file = root.path().join("settings.json");
        fs::write(&file, "{").unwrap();
        assert!(SettingsFile::new(file, Settings::initial("en"))
            .read()
            .unwrap_err()
            .to_string()
            .contains("settings.json"));
    }

    #[test]
    fn understands_settings_without_language() {
        let root = TempDir::new().unwrap();
        let file = root.path().join("settings.json");
        fs::write(&file, r#"{"sources":[],"blocked":[],"allowed":[]}"#).unwrap();
        assert_eq!(
            SettingsFile::new(file, Settings::initial("en"))
                .read()
                .unwrap()
                .language,
            ""
        );
    }

    #[test]
    fn starts_with_initial_settings() {
        let root = TempDir::new().unwrap();
        assert_eq!(
            SettingsFile::new(root.path().join("settings.json"), Settings::initial("en"))
                .read()
                .unwrap(),
            Settings::initial("en")
        );
    }
}
