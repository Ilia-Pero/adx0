//! Contents of the hosts file.

use std::collections::BTreeSet;
use std::fmt::Write;

use crate::{Domain, Failure};

const BEGIN: &[u8] = b"# BEGIN adx0. Do not edit this section";
const END: &[u8] = b"# END adx0";

/// How many names there are in a line. A short file is read faster, and Windows accepts
/// no more than nine names in a line.
const NAMES: usize = 9;

/// Contents of the hosts file: lines of a person and the section of the program. They are kept
/// as bytes, because comments of other people come in any encoding and must not change.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hosts {
    bytes: Vec<u8>,
}

impl Hosts {
    #[must_use]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The same contents without the section of the program.
    ///
    /// # Errors
    ///
    /// The section is damaged, or the file is in UTF-16.
    pub fn cleared(&self) -> Result<Hosts, Failure> {
        if self.bytes.starts_with(&[0xff, 0xfe]) || self.bytes.starts_with(&[0xfe, 0xff]) {
            return Err(Failure::Encoded);
        }
        let mut kept = Vec::with_capacity(self.bytes.len());
        let mut inside = false;
        for line in self.bytes.split_inclusive(|byte| *byte == b'\n') {
            match line.trim_ascii() {
                BEGIN if inside => return Err(Failure::Broken),
                BEGIN => inside = true,
                END if inside => inside = false,
                END => return Err(Failure::Broken),
                _ if inside => {}
                _ => kept.extend(line),
            }
        }
        if inside {
            return Err(Failure::Broken);
        }
        Ok(Hosts { bytes: kept })
    }

    /// The same contents with a section that blocks the addresses. The section comes last:
    /// the system takes the first line that fits, so lines of a person stay in charge.
    ///
    /// # Errors
    ///
    /// The old section is damaged, or the file is in UTF-16.
    pub fn guarded(&self, domains: &BTreeSet<Domain>) -> Result<Hosts, Failure> {
        let mut bytes = self.cleared()?.bytes;
        if domains.is_empty() {
            return Ok(Hosts { bytes });
        }
        let end = if bytes.windows(2).any(|pair| pair == b"\r\n") {
            "\r\n"
        } else {
            "\n"
        };
        if !bytes.is_empty() && !bytes.ends_with(b"\n") {
            bytes.extend(end.as_bytes());
        }
        bytes.extend(BEGIN);
        bytes.extend(end.as_bytes());
        let names: Vec<&Domain> = domains.iter().collect();
        for group in names.chunks(NAMES) {
            let mut line = "0.0.0.0".to_owned();
            for name in group {
                let _ = write!(line, " {name}");
            }
            bytes.extend(line.as_bytes());
            bytes.extend(end.as_bytes());
        }
        bytes.extend(END);
        bytes.extend(end.as_bytes());
        Ok(Hosts { bytes })
    }

    /// How many addresses the section blocks.
    #[must_use]
    pub fn count(&self) -> usize {
        self.bytes
            .split(|byte| *byte == b'\n')
            .map(<[u8]>::trim_ascii)
            .skip_while(|line| *line != BEGIN)
            .take_while(|line| *line != END)
            .filter(|line| line.starts_with(b"0.0.0.0 "))
            .map(|line| line.split(|byte| *byte == b' ').count() - 1)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::Hosts;
    use crate::{Domain, Failure};

    fn hosts(text: &str) -> Hosts {
        Hosts::new(text.as_bytes().to_vec())
    }

    fn domains(count: usize) -> BTreeSet<Domain> {
        (0..count)
            .map(|index| Domain::parsed(&format!("ad{index:03}.com")).unwrap())
            .collect()
    }

    #[test]
    fn returns_to_original_after_clearing() {
        let original = hosts("127.0.0.1 localhost\r\n# my line\r\n10.0.0.5 nas.local\r\n");
        assert_eq!(
            original.guarded(&domains(20)).unwrap().cleared().unwrap(),
            original
        );
    }

    #[test]
    fn keeps_bytes_of_foreign_encoding() {
        let original = Hosts::new(b"# \xcf\xf0\xe8\xe2\xe5\xf2\n".to_vec());
        assert_eq!(
            original.guarded(&domains(1)).unwrap().cleared().unwrap(),
            original
        );
    }

    #[test]
    fn puts_section_after_own_lines() {
        assert!(hosts("127.0.0.1 localhost\n")
            .guarded(&domains(1))
            .unwrap()
            .bytes()
            .starts_with(b"127.0.0.1 localhost\n# BEGIN"));
    }

    #[test]
    fn starts_section_on_new_line() {
        assert!(hosts("127.0.0.1 localhost")
            .guarded(&domains(1))
            .unwrap()
            .bytes()
            .starts_with(b"127.0.0.1 localhost\n# BEGIN"));
    }

    #[test]
    fn puts_at_most_nine_names_in_line() {
        let guarded = hosts("").guarded(&domains(10)).unwrap();
        assert_eq!(
            String::from_utf8(guarded.bytes().to_vec())
                .unwrap()
                .lines()
                .filter(|line| line.starts_with("0.0.0.0 "))
                .count(),
            2
        );
    }

    #[test]
    fn replaces_earlier_section() {
        assert_eq!(
            hosts("")
                .guarded(&domains(50))
                .unwrap()
                .guarded(&domains(3))
                .unwrap()
                .count(),
            3
        );
    }

    #[test]
    fn removes_section_when_nothing_to_block() {
        assert_eq!(
            hosts("a\n")
                .guarded(&domains(5))
                .unwrap()
                .guarded(&domains(0))
                .unwrap(),
            hosts("a\n")
        );
    }

    #[test]
    fn counts_every_name() {
        assert_eq!(hosts("").guarded(&domains(25)).unwrap().count(), 25);
    }

    #[test]
    fn counts_nothing_outside_section() {
        assert_eq!(hosts("0.0.0.0 a.com b.com\n").count(), 0);
    }

    #[test]
    fn refuses_section_without_end() {
        assert!(matches!(
            hosts("# BEGIN adx0. Do not edit this section\n0.0.0.0 a.com\n").cleared(),
            Err(Failure::Broken)
        ));
    }

    #[test]
    fn refuses_end_without_section() {
        assert!(matches!(
            hosts("# END adx0\n").cleared(),
            Err(Failure::Broken)
        ));
    }

    #[test]
    fn refuses_utf16() {
        assert!(matches!(
            Hosts::new(vec![0xff, 0xfe, b'a', 0]).cleared(),
            Err(Failure::Encoded)
        ));
    }
}
