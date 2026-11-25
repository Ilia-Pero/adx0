//! Lists of ad addresses.

use std::collections::BTreeSet;

use crate::Domain;

/// Where lists in the hosts format send ads. Lines with other addresses are skipped: otherwise
/// somebody else's list could replace the address of a real site.
const NOWHERE: [&str; 4] = ["0.0.0.0", "127.0.0.1", "::", "::1"];

/// How a list is written.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// Lines such as `0.0.0.0 ads.example.com` or just `ads.example.com`.
    Hosts,
    /// Rules of Adblock Plus. Only `||ads.example.com^` is read, because a name at the beginning
    /// of any other rule means "on this site" and not "block this site".
    Adblock,
}

impl Format {
    /// The line that says to block the address.
    #[must_use]
    pub fn line(self, domain: &Domain) -> String {
        match self {
            Format::Hosts => format!("0.0.0.0 {domain}\n"),
            Format::Adblock => format!("||{domain}^\n"),
        }
    }
}

/// The text of a list.
pub struct Blocklist<'a> {
    text: &'a str,
    format: Format,
}

impl<'a> Blocklist<'a> {
    #[must_use]
    pub fn new(text: &'a str, format: Format) -> Self {
        Self { text, format }
    }

    /// Addresses from the list, each of them once.
    #[must_use]
    pub fn domains(&self) -> BTreeSet<Domain> {
        match self.format {
            Format::Hosts => self.text.lines().flat_map(row).collect(),
            Format::Adblock => self.text.lines().filter_map(rule).collect(),
        }
    }
}

/// The address from the rule `||ads.example.com^`.
fn rule(line: &str) -> Option<Domain> {
    line.trim()
        .strip_prefix("||")?
        .strip_suffix('^')
        .and_then(Domain::parsed)
}

/// Addresses from a line of the hosts format. A comment starts with `#` after a space. In
/// `site.com#?#.banner` there is no comment, so the whole line is skipped: otherwise the site
/// itself would be blocked.
fn row(line: &str) -> Vec<Domain> {
    let words: Vec<&str> = line
        .split_whitespace()
        .take_while(|word| !word.starts_with(['#', '!', '[']))
        .collect();
    match words.as_slice() {
        [name] => Domain::parsed(name).into_iter().collect(),
        [target, names @ ..] if NOWHERE.contains(target) => names
            .iter()
            .filter_map(|name| Domain::parsed(name))
            .collect(),
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{Blocklist, Format};
    use crate::Domain;

    const NONE: [&str; 0] = [];

    fn names(text: &str, format: Format) -> Vec<String> {
        Blocklist::new(text, format)
            .domains()
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    fn hosts(text: &str) -> Vec<String> {
        names(text, Format::Hosts)
    }

    fn adblock(text: &str) -> Vec<String> {
        names(text, Format::Adblock)
    }

    #[test]
    fn reads_hosts_line() {
        assert_eq!(hosts("0.0.0.0 ads.example.com"), ["ads.example.com"]);
    }

    #[test]
    fn reads_several_names_in_hosts_line() {
        assert_eq!(hosts("127.0.0.1 a.com b.com # note"), ["a.com", "b.com"]);
    }

    #[test]
    fn reads_plain_name() {
        assert_eq!(hosts("ads.example.com"), ["ads.example.com"]);
    }

    #[test]
    fn skips_line_leading_to_real_address() {
        assert_eq!(hosts("203.0.113.7 bank.example.com"), NONE);
    }

    #[test]
    fn skips_comments_of_hosts() {
        assert_eq!(hosts("# a.com\n\n  # b.com"), NONE);
    }

    #[test]
    fn skips_name_of_this_computer() {
        assert_eq!(hosts("127.0.0.1 localhost"), NONE);
    }

    #[test]
    fn does_not_close_site_named_in_stray_rule() {
        assert_eq!(hosts("youtube.com#?#.banner"), NONE);
    }

    #[test]
    fn drops_repeats() {
        assert_eq!(
            Blocklist::new("a.com\n0.0.0.0 A.com\r\n", Format::Hosts).domains(),
            BTreeSet::from([Domain::parsed("a.com").unwrap()])
        );
    }

    /// The address written in the format and read from it.
    fn returned(format: Format) -> Vec<String> {
        names(&format.line(&Domain::parsed("a.com").unwrap()), format)
    }

    #[test]
    fn reads_its_own_line_of_hosts() {
        assert_eq!(returned(Format::Hosts), ["a.com"]);
    }

    #[test]
    fn reads_its_own_rule() {
        assert_eq!(returned(Format::Adblock), ["a.com"]);
    }

    #[test]
    fn reads_rule_for_whole_site() {
        assert_eq!(adblock("||ads.example.com^"), ["ads.example.com"]);
    }

    #[test]
    fn skips_rule_with_conditions() {
        assert_eq!(adblock("||example.com^$third-party"), NONE);
    }

    #[test]
    fn skips_rule_for_part_of_site() {
        assert_eq!(adblock("||example.com/banner^"), NONE);
    }

    #[test]
    fn skips_rule_hiding_elements() {
        assert_eq!(adblock("example.com##.banner"), NONE);
    }

    #[test]
    fn does_not_close_site_named_in_extended_rule() {
        assert_eq!(adblock("youtube.com#?#.banner"), NONE);
    }

    #[test]
    fn does_not_close_site_named_in_style_rule() {
        assert_eq!(adblock("ok.ru#$#body { top: 0 }"), NONE);
    }

    #[test]
    fn does_not_close_site_named_alone_among_rules() {
        assert_eq!(
            adblock("[Adblock Plus 2.0]\nyandex.ru\n||ads.example.com^"),
            ["ads.example.com"]
        );
    }
}
