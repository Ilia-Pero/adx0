//! Names of sites.

use std::fmt::{self, Display, Formatter};
use std::net::IpAddr;

/// The name of a site: lowercase Latin letters, digits, hyphens and underscores, with at least
/// one dot.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Domain {
    name: String,
}

impl Domain {
    /// The name of a site from the text. Nothing if the text is not a name.
    #[must_use]
    pub fn parsed(text: &str) -> Option<Domain> {
        let name = text.strip_suffix('.').unwrap_or(text).to_ascii_lowercase();
        let sound = name.len() <= 253
            && name.contains('.')
            && name.parse::<IpAddr>().is_err()
            && name.split('.').all(label)
            && name != "localhost.localdomain";
        sound.then_some(Domain { name })
    }

    /// The same name without `www` at the beginning.
    #[must_use]
    pub fn plain(&self) -> Domain {
        let name = self.name.strip_prefix("www.").unwrap_or(&self.name);
        Domain {
            name: name.to_owned(),
        }
    }

    /// True if this is the same site or its subdomain.
    #[must_use]
    pub fn within(&self, other: &Domain) -> bool {
        self.name
            .strip_suffix(&other.name)
            .is_some_and(|head| head.is_empty() || head.ends_with('.'))
    }
}

impl Display for Domain {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        out.write_str(&self.name)
    }
}

/// A part of the name between dots.
fn label(text: &str) -> bool {
    (1..=63).contains(&text.len())
        && !text.starts_with('-')
        && !text.ends_with('-')
        && text
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
}

/// The address of a site as a person types it: with or without `https://`, a path and a port.
pub struct Address<'a> {
    text: &'a str,
}

impl<'a> Address<'a> {
    #[must_use]
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    /// The name of the site in the address. Nothing if there is no name.
    #[must_use]
    pub fn domain(&self) -> Option<Domain> {
        let text = self.text.trim();
        let rest = text.split_once("://").map_or(text, |(_, rest)| rest);
        let host = rest.split(['/', '?', '#', ':']).next().unwrap_or_default();
        Domain::parsed(host)
    }
}

#[cfg(test)]
mod tests {
    use super::{Address, Domain};

    fn domain(text: &str) -> Domain {
        Domain::parsed(text).unwrap()
    }

    #[test]
    fn ignores_letter_case() {
        assert_eq!(domain("Ads.Example.COM"), domain("ads.example.com"));
    }

    #[test]
    fn ignores_trailing_dot() {
        assert_eq!(domain("ads.example.com."), domain("ads.example.com"));
    }

    #[test]
    fn refuses_name_without_dot() {
        assert_eq!(Domain::parsed("localhost"), None);
    }

    #[test]
    fn refuses_ip_address() {
        assert_eq!(Domain::parsed("127.0.0.1"), None);
    }

    #[test]
    fn refuses_spaces_and_line_breaks() {
        assert_eq!(Domain::parsed("a.com\n1.2.3.4 bank.com"), None);
    }

    #[test]
    fn refuses_empty_label() {
        assert_eq!(Domain::parsed("a..com"), None);
    }

    #[test]
    fn refuses_mask() {
        assert_eq!(Domain::parsed("*.example.com"), None);
    }

    #[test]
    fn drops_www_from_plain_name() {
        assert_eq!(domain("www.example.com").plain(), domain("example.com"));
    }

    #[test]
    fn counts_subdomain_as_part_of_site() {
        assert!(domain("ads.example.com").within(&domain("example.com")));
    }

    #[test]
    fn does_not_count_lookalike_as_part_of_site() {
        assert!(!domain("badexample.com").within(&domain("example.com")));
    }

    #[test]
    fn finds_site_in_full_address() {
        assert_eq!(
            Address::new(" https://shop.example.com:8080/cart?x=1 ").domain(),
            Some(domain("shop.example.com"))
        );
    }

    #[test]
    fn finds_nothing_in_plain_word() {
        assert_eq!(Address::new("hello").domain(), None);
    }
}
