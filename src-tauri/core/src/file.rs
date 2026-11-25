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
        self.change(fresh, fill)
    }

    /// Changes the contents with the function that writes bytes into the file. It may fail halfway,
    /// for example when the disk is full.
    fn change(
        &self,
        fresh: &Hosts,
        fill: impl Fn(&Path, &[u8]) -> io::Result<()>,
    ) -> Result<(), Failure> {
        let closed = |failure: io::Error| Failure::Closed(failure.to_string());
        let before = self.read()?;
        if before == *fresh {
            return Ok(());
        }
        let filled = self
            .keep(&before)
            .and_then(|()| fill(&self.file, fresh.bytes()));
        let holds = |wanted: &Hosts| self.read().is_ok_and(|now| now == *wanted);
        if filled.is_ok() && holds(fresh) {
            return Ok(());
        }
        if !holds(&before) {
            fill(&self.file, before.bytes()).map_err(closed)?;
        }
        Err(filled.map_or_else(closed, |()| Failure::Unverified))
    }

    /// Leaves a copy of what was there before the program. Does not touch an existing copy.
    fn keep(&self, before: &Hosts) -> io::Result<()> {
        let place = self.file.with_extension("before-adx0");
        match File::create_new(&place) {
            Ok(mut copy) => copy
                .write_all(before.bytes())
                .and_then(|()| copy.sync_all())
                .inspect_err(|_| {
                    // The next time a part of a copy would be taken for a whole one.
                    let _ = fs::remove_file(&place);
                }),
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
    use std::cell::Cell;
    use std::fs;
    use std::io;
    use std::path::Path;

    use tempfile::TempDir;

    use super::HostsFile;
    use crate::{Failure, Hosts};

    fn hosts(text: &str) -> Hosts {
        Hosts::new(text.as_bytes().to_vec())
    }

    /// The outcome of a change during which the disk is full for a while, and the file after it.
    fn full() -> (Result<(), Failure>, String) {
        let root = TempDir::new().unwrap();
        let path = root.path().join("hosts");
        fs::write(&path, "mine\n").unwrap();
        let first = Cell::new(true);
        let outcome =
            HostsFile::new(path.clone()).change(&hosts("new\n"), |file: &Path, bytes: &[u8]| {
                if first.replace(false) {
                    fs::write(file, &bytes[..1])?;
                    return Err(io::Error::other("the disk is full"));
                }
                fs::write(file, bytes)
            });
        (outcome, fs::read_to_string(path).unwrap())
    }

    #[test]
    fn returns_to_what_was_before_when_the_disk_is_full() {
        assert_eq!(full().1, "mine\n");
    }

    #[test]
    fn reports_a_full_disk() {
        assert!(matches!(full().0, Err(Failure::Closed(reason)) if reason.contains("full")));
    }

    #[test]
    fn returns_to_what_was_before_when_something_else_is_written() {
        let root = TempDir::new().unwrap();
        let path = root.path().join("hosts");
        fs::write(&path, "mine\n").unwrap();
        let first = Cell::new(true);
        let outcome =
            HostsFile::new(path.clone()).change(&hosts("new\n"), |file: &Path, bytes: &[u8]| {
                fs::write(file, if first.replace(false) { b"" } else { bytes })
            });
        assert!(
            matches!(outcome, Err(Failure::Unverified)) && fs::read(path).unwrap() == b"mine\n"
        );
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
