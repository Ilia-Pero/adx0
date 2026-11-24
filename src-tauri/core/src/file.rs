//! The hosts file on the disk.

use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Write};
use std::path::{Path, PathBuf};

use crate::{Failure, Hosts};

/// The hosts file. The first time it is written, a copy of what was there before stays next to it.
pub struct HostsFile {
    file: PathBuf,
}

impl HostsFile {
    #[must_use]
    pub fn new(file: PathBuf) -> Self {
        Self { file }
    }

    /// Contents of the file. They are empty if there is no file.
    ///
    /// # Errors
    ///
    /// There is a file, but it cannot be read.
    pub fn read(&self) -> Result<Hosts, Failure> {
        match fs::read(&self.file) {
            Ok(bytes) => Ok(Hosts::new(bytes)),
            Err(failure) if failure.kind() == ErrorKind::NotFound => Ok(Hosts::new(Vec::new())),
            Err(failure) => Err(Failure::Closed(failure.to_string())),
        }
    }

    /// Writes the contents and checks what has been written. If the disk holds something else,
    /// puts the old contents back. The file is not replaced by a new one, so that its access
    /// rights stay the same.
    ///
    /// # Errors
    ///
    /// The file cannot be written, or was not written completely.
    pub fn write(&self, fresh: &Hosts) -> Result<(), Failure> {
        let before = self.read()?;
        if before == *fresh {
            return Ok(());
        }
        self.keep(&before)
            .and_then(|()| fill(&self.file, fresh.bytes()))
            .map_err(|failure| Failure::Closed(failure.to_string()))?;
        if self.read()? == *fresh {
            return Ok(());
        }
        fill(&self.file, before.bytes()).map_err(|failure| Failure::Closed(failure.to_string()))?;
        Err(Failure::Unverified)
    }

    /// Leaves a copy of what was there before the program. Does not touch an existing copy.
    fn keep(&self, before: &Hosts) -> io::Result<()> {
        match File::create_new(self.file.with_extension("before-adx0")) {
            Ok(mut copy) => {
                copy.write_all(before.bytes())?;
                copy.sync_all()
            }
            Err(failure) if failure.kind() == ErrorKind::AlreadyExists => Ok(()),
            Err(failure) => Err(failure),
        }
    }
}

/// Replaces contents of the file.
fn fill(file: &Path, bytes: &[u8]) -> io::Result<()> {
    let mut open = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file)?;
    open.write_all(bytes)?;
    open.sync_all()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::HostsFile;
    use crate::Hosts;

    fn hosts(text: &str) -> Hosts {
        Hosts::new(text.as_bytes().to_vec())
    }

    #[test]
    fn treats_missing_file_as_empty() {
        let root = TempDir::new().unwrap();
        assert_eq!(
            HostsFile::new(root.path().join("hosts")).read().unwrap(),
            hosts("")
        );
    }

    #[test]
    fn returns_what_was_written() {
        let root = TempDir::new().unwrap();
        let file = HostsFile::new(root.path().join("hosts"));
        file.write(&hosts("new\n")).unwrap();
        assert_eq!(file.read().unwrap(), hosts("new\n"));
    }

    #[test]
    fn leaves_copy_of_what_was_before() {
        let root = TempDir::new().unwrap();
        fs::write(root.path().join("hosts"), "mine\n").unwrap();
        HostsFile::new(root.path().join("hosts"))
            .write(&hosts("new\n"))
            .unwrap();
        assert_eq!(
            fs::read_to_string(root.path().join("hosts.before-adx0")).unwrap(),
            "mine\n"
        );
    }

    #[test]
    fn keeps_first_copy_on_later_writes() {
        let root = TempDir::new().unwrap();
        fs::write(root.path().join("hosts"), "mine\n").unwrap();
        let file = HostsFile::new(root.path().join("hosts"));
        file.write(&hosts("new\n")).unwrap();
        file.write(&hosts("newer\n")).unwrap();
        assert_eq!(
            fs::read_to_string(root.path().join("hosts.before-adx0")).unwrap(),
            "mine\n"
        );
    }

    #[test]
    fn leaves_no_copy_when_nothing_changes() {
        let root = TempDir::new().unwrap();
        fs::write(root.path().join("hosts"), "mine\n").unwrap();
        HostsFile::new(root.path().join("hosts"))
            .write(&hosts("mine\n"))
            .unwrap();
        assert!(!root.path().join("hosts.before-adx0").exists());
    }
}
