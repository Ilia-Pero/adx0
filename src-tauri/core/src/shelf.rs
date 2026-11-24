//! Downloaded lists on the disk.

use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::{Blocklist, Failure, Source, Web};

/// With fewer addresses than this, something else came instead of a list, such as an error page.
const FEWEST: usize = 100;

/// A shelf with downloaded lists. The last good list stays on the disk, so blocking does not
/// depend on whether its site is up.
pub struct Shelf {
    root: PathBuf,
}

impl Shelf {
    #[must_use]
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Downloads the list and puts it on the shelf. A bad list does not replace the old one.
    ///
    /// # Errors
    ///
    /// The list has not been downloaded, has almost no addresses, or cannot be written.
    pub fn refresh(&self, source: &Source, web: &impl Web) -> Result<(), Failure> {
        let text = web.text(source.url)?;
        let count = Blocklist::new(&text, source.format).domains().len();
        if count < FEWEST {
            return Err(Failure::Short {
                title: source.title.to_owned(),
                count,
            });
        }
        fs::create_dir_all(&self.root)?;
        let file = self.file(source.id);
        let draft = file.with_extension("new");
        fs::write(&draft, text)?;
        fs::rename(&draft, &file)?;
        Ok(())
    }

    /// The text of the list. Empty if the list is not downloaded yet.
    ///
    /// # Errors
    ///
    /// There is a list, but it cannot be read.
    pub fn text(&self, id: &str) -> Result<String, Failure> {
        match fs::read_to_string(self.file(id)) {
            Ok(text) => Ok(text),
            Err(failure) if failure.kind() == ErrorKind::NotFound => Ok(String::new()),
            Err(failure) => Err(failure.into()),
        }
    }

    /// When the list was downloaded. Nothing if it is not downloaded yet.
    #[must_use]
    pub fn moment(&self, id: &str) -> Option<SystemTime> {
        fs::metadata(self.file(id))
            .and_then(|meta| meta.modified())
            .ok()
    }

    /// Only letters and digits of the name get into the path: anybody can edit the settings,
    /// and a name such as `../../secret` must not lead off the shelf.
    fn file(&self, id: &str) -> PathBuf {
        let name: String = id.chars().filter(char::is_ascii_alphanumeric).collect();
        self.root.join(format!("{name}.txt"))
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;
    use std::fs;

    use tempfile::TempDir;

    use super::Shelf;
    use crate::{Failure, Source, Web, SOURCES};

    /// The internet that answers the same to any address.
    struct Page(String);

    impl Web for Page {
        fn text(&self, _url: &str) -> Result<String, Failure> {
            Ok(self.0.clone())
        }
    }

    fn source() -> &'static Source {
        &SOURCES[0]
    }

    fn page(count: usize) -> Page {
        let mut text = String::new();
        for index in 0..count {
            writeln!(text, "0.0.0.0 ad{index}.com").unwrap();
        }
        Page(text)
    }

    #[test]
    fn keeps_downloaded_list() {
        let root = TempDir::new().unwrap();
        let shelf = Shelf::new(root.path().join("lists"));
        shelf.refresh(source(), &page(100)).unwrap();
        assert_eq!(shelf.text(source().id).unwrap(), page(100).0);
    }

    #[test]
    fn keeps_earlier_list_when_site_sends_garbage() {
        let root = TempDir::new().unwrap();
        let shelf = Shelf::new(root.path().to_owned());
        shelf.refresh(source(), &page(100)).unwrap();
        let _ = shelf.refresh(source(), &Page("<html>Maintenance</html>".to_owned()));
        assert_eq!(shelf.text(source().id).unwrap(), page(100).0);
    }

    #[test]
    fn reports_list_with_too_few_addresses() {
        let root = TempDir::new().unwrap();
        assert!(matches!(
            Shelf::new(root.path().to_owned()).refresh(source(), &page(99)),
            Err(Failure::Short { count: 99, .. })
        ));
    }

    #[test]
    fn has_empty_text_before_download() {
        let root = TempDir::new().unwrap();
        assert_eq!(Shelf::new(root.path().to_owned()).text("yoyo").unwrap(), "");
    }

    #[test]
    fn does_not_leave_its_folder() {
        let root = TempDir::new().unwrap();
        fs::write(root.path().join("secret.txt"), "secret").unwrap();
        assert_eq!(
            Shelf::new(root.path().join("lists"))
                .text("../secret")
                .unwrap(),
            ""
        );
    }
}
