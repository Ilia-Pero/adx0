//! What can go wrong.

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;

use adx0_phrase::Phrase;

/// A failure of the ad blocker. Each one has a number: the part of the program that works with
/// the administrator's permission reports a failure by its exit code only.
#[derive(Debug)]
pub enum Failure {
    /// The hosts file is out of reach: another program holds it.
    Closed(String),
    /// The administrator's permission was not given.
    Refused,
    /// The section in the hosts file is damaged.
    Broken,
    /// The hosts file is in an encoding the system cannot read.
    Encoded,
    /// The hosts file was not what was written to it. Its previous contents are back.
    Unverified,
    /// The site of a list does not answer.
    Silent { url: String, reason: String },
    /// There is almost nothing in a downloaded list.
    Short { title: String, count: usize },
    /// Settings or lists on the disk cannot be read or written.
    Disk(String),
    /// There is nothing to block: no list is chosen, or none is downloaded.
    Empty,
    /// It is not known whether there is a newer version of the program.
    Unknown(String),
}

impl Failure {
    /// The number by which the failure is passed as an exit code.
    #[must_use]
    pub fn code(&self) -> u8 {
        match self {
            Failure::Closed(_) => 10,
            Failure::Broken => 11,
            Failure::Encoded => 12,
            Failure::Unverified => 13,
            Failure::Refused => 14,
            Failure::Empty => 16,
            Failure::Silent { .. }
            | Failure::Short { .. }
            | Failure::Disk(_)
            | Failure::Unknown(_) => 15,
        }
    }

    /// The failure by the number with which the other part of the program has exited.
    #[must_use]
    pub fn from_code(code: i32) -> Failure {
        match code {
            10 => Failure::Closed("10".to_owned()),
            11 => Failure::Broken,
            12 => Failure::Encoded,
            13 => Failure::Unverified,
            14 => Failure::Refused,
            16 => Failure::Empty,
            other => Failure::Disk(other.to_string()),
        }
    }

    fn phrase(&self) -> Phrase {
        match self {
            Failure::Closed(reason) => Phrase::new("error.closed").with("reason", reason),
            Failure::Refused => Phrase::new("error.refused"),
            Failure::Broken => Phrase::new("error.broken"),
            Failure::Encoded => Phrase::new("error.encoded"),
            Failure::Unverified => Phrase::new("error.unverified"),
            Failure::Silent { url, reason } => Phrase::new("error.silent")
                .with("url", url)
                .with("reason", reason),
            Failure::Short { title, count } => Phrase::new("error.short")
                .with("title", title)
                .with("count", count),
            Failure::Disk(reason) => Phrase::new("error.disk").with("reason", reason),
            Failure::Empty => Phrase::new("error.empty"),
            Failure::Unknown(reason) => Phrase::new("error.update").with("reason", reason),
        }
    }
}

impl Display for Failure {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        self.phrase().fmt(out)
    }
}

impl Error for Failure {}

impl From<io::Error> for Failure {
    fn from(failure: io::Error) -> Self {
        Failure::Disk(failure.to_string())
    }
}

impl From<serde_json::Error> for Failure {
    fn from(failure: serde_json::Error) -> Self {
        Failure::Disk(failure.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Failure;

    #[test]
    fn survives_transfer_by_exit_code() {
        assert!(matches!(
            Failure::from_code(i32::from(Failure::Broken.code())),
            Failure::Broken
        ));
    }

    #[test]
    fn explains_itself() {
        assert_ne!(Failure::Empty.to_string(), "");
    }
}
