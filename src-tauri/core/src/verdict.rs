//! What to block.

use std::collections::BTreeSet;

use crate::{Address, Blocklist, Domain, Failure, Settings, Shelf, Source};

/// Sites that lists cannot block: one of them in a list is a mistake of the list. Ad subdomains
/// of these sites are blocked as usual, and a person may block any of the sites.
const ESSENTIAL: &str = include_str!("../../../data/essential.json");

/// Ad servers and counters of Yandex and Mail.ru that lists lack: lists are made for browsers
/// and block them by parts of the address, which the hosts file cannot do.
const OWN: &str = include_str!("../../../data/own.json");

/// The decision what to block: the lists and addresses of a person, except the allowed ones.
pub struct Verdict<'a> {
    settings: &'a Settings,
    shelf: &'a Shelf,
    sources: &'a [Source],
}

impl<'a> Verdict<'a> {
    #[must_use]
    pub fn new(settings: &'a Settings, shelf: &'a Shelf, sources: &'a [Source]) -> Self {
        Self {
            settings,
            shelf,
            sources,
        }
    }

    /// Addresses to block.
    ///
    /// # Errors
    ///
    /// A downloaded list cannot be read.
    pub fn domains(&self) -> Result<BTreeSet<Domain>, Failure> {
        let mut domains = BTreeSet::new();
        for source in self.sources {
            if self.settings.sources.iter().any(|id| id == source.id) {
                let text = self.shelf.text(source.id)?;
                domains.extend(Blocklist::new(&text, source.format).domains());
            }
        }
        let essential = built(ESSENTIAL)?;
        domains.retain(|domain| !essential.contains(&domain.plain()));
        if !domains.is_empty() {
            domains.extend(built(OWN)?);
        }
        domains.extend(sites(&self.settings.blocked));
        let allowed = self.allowed();
        domains.retain(|domain| !allowed.iter().any(|open| domain.within(open)));
        Ok(domains)
    }

    /// Sites that must stay open: those a person has chosen and those lists are downloaded from.
    /// Otherwise one mistake in a list would keep the program from ever updating it.
    fn allowed(&self) -> BTreeSet<Domain> {
        self.sources
            .iter()
            .filter_map(|source| Address::new(source.url).domain())
            .chain(sites(&self.settings.allowed))
            .collect()
    }
}

/// Addresses built into the program.
fn built(json: &str) -> Result<BTreeSet<Domain>, Failure> {
    let names: Vec<String> = serde_json::from_str(json)?;
    Ok(sites(&names))
}

fn sites(texts: &[String]) -> BTreeSet<Domain> {
    texts
        .iter()
        .filter_map(|text| Address::new(text).domain())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fs;

    use tempfile::TempDir;

    use super::{built, Verdict, ESSENTIAL, OWN};
    use crate::{Domain, Settings, Shelf, SOURCES};

    /// The verdict with one list of the given text, without addresses of the program itself.
    fn names(list: &str, blocked: &[&str], allowed: &[&str]) -> Vec<String> {
        all(list, blocked, allowed)
            .difference(&built(OWN).unwrap())
            .map(ToString::to_string)
            .collect()
    }

    /// The verdict with one list of the given text.
    fn all(list: &str, blocked: &[&str], allowed: &[&str]) -> BTreeSet<Domain> {
        let root = TempDir::new().unwrap();
        fs::write(root.path().join("yoyo.txt"), list).unwrap();
        Verdict::new(
            &Settings {
                sources: vec!["yoyo".to_owned(), "missing".to_owned()],
                blocked: blocked.iter().map(ToString::to_string).collect(),
                allowed: allowed.iter().map(ToString::to_string).collect(),
                language: String::new(),
            },
            &Shelf::new(root.path().to_owned()),
            SOURCES,
        )
        .domains()
        .unwrap()
    }

    fn domain(name: &str) -> Domain {
        Domain::parsed(name).unwrap()
    }

    #[test]
    fn joins_list_with_addresses_of_person() {
        assert_eq!(
            names("a.com", &["https://b.com/page"], &[]),
            ["a.com", "b.com"]
        );
    }

    #[test]
    fn opens_allowed_site_with_subdomains() {
        assert_eq!(
            names("shop.com\nads.shop.com\nother.com", &[], &["shop.com"]),
            ["other.com"]
        );
    }

    #[test]
    fn prefers_allowed_over_blocked() {
        assert_eq!(names("", &["a.com"], &["a.com"]), Vec::<String>::new());
    }

    #[test]
    fn keeps_essential_site_open_when_list_is_wrong() {
        assert_eq!(
            names("youtube.com\nwww.yandex.ru\nads.youtube.com", &[], &[]),
            ["ads.youtube.com"]
        );
    }

    #[test]
    fn lets_person_block_essential_site() {
        assert_eq!(names("", &["youtube.com"], &[]), ["youtube.com"]);
    }

    #[test]
    fn adds_addresses_of_its_own() {
        assert!(all("a.com", &[], &[]).contains(&domain("an.yandex.ru")));
    }

    #[test]
    fn adds_nothing_of_its_own_without_lists() {
        assert_eq!(all("", &[], &[]), BTreeSet::new());
    }

    #[test]
    fn opens_address_of_its_own_when_person_allows_it() {
        assert!(!all("a.com", &[], &["mc.yandex.ru"]).contains(&domain("mc.yandex.ru")));
    }

    #[test]
    fn never_closes_sites_of_lists() {
        assert_eq!(names("adaway.org\na.com", &[], &[]), ["a.com"]);
    }

    #[test]
    fn blocks_nothing_essential_of_its_own() {
        assert!(built(OWN).unwrap().is_disjoint(&built(ESSENTIAL).unwrap()));
    }
}
