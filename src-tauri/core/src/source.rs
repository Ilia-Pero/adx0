//! Where lists come from.

use crate::{Failure, Format};

/// A list of addresses on the internet.
#[derive(Debug)]
pub struct Source {
    /// The name of the file where the list is kept.
    pub id: &'static str,
    pub title: &'static str,
    /// The key of the note for a person: what is in the list and under which license.
    pub note: &'static str,
    pub url: &'static str,
    pub format: Format,
    /// For whom the list is on from the start: codes of languages, or `*` for everyone.
    pub initial: &'static [&'static str],
}

impl Source {
    /// Whether the list is on until a person who speaks the language chooses otherwise.
    #[must_use]
    pub fn suits(&self, language: &str) -> bool {
        self.initial.contains(&"*") || self.initial.contains(&language)
    }
}

/// Lists the program can download. They are asked for by the program of a person and never
/// handed on. Numbers of addresses were checked on 2026-11-24.
pub const SOURCES: &[Source] = &[
    Source {
        id: "yoyo",
        title: "Peter Lowe",
        note: "source.yoyo",
        url: "https://pgl.yoyo.org/adservers/serverlist.php?hostformat=hosts&showintro=0&mimetype=plaintext",
        format: Format::Hosts,
        initial: &["*"],
    },
    Source {
        id: "adaway",
        title: "AdAway",
        note: "source.adaway",
        url: "https://adaway.org/hosts.txt",
        format: Format::Hosts,
        initial: &["*"],
    },
    Source {
        id: "ruadlist",
        title: "RU AdList",
        note: "source.ruadlist",
        url: "https://easylist-downloads.adblockplus.org/advblock.txt",
        format: Format::Adblock,
        initial: &["ru"],
    },
    Source {
        id: "listefr",
        title: "Liste FR",
        note: "source.listefr",
        url: "https://easylist-downloads.adblockplus.org/liste_fr.txt",
        format: Format::Adblock,
        initial: &["fr"],
    },
    Source {
        id: "easylistchina",
        title: "EasyList China",
        note: "source.easylistchina",
        url: "https://easylist-downloads.adblockplus.org/easylistchina.txt",
        format: Format::Adblock,
        initial: &["zh"],
    },
    Source {
        id: "stevenblack",
        title: "StevenBlack",
        note: "source.stevenblack",
        url: "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts",
        format: Format::Hosts,
        initial: &[],
    },
];

/// The internet lists are downloaded from.
pub trait Web {
    /// The text at the address.
    ///
    /// # Errors
    ///
    /// The address does not answer, or the answer is not a text.
    fn text(&self, url: &str) -> Result<String, Failure>;
}

#[cfg(test)]
mod tests {
    use super::SOURCES;
    use crate::Address;

    #[test]
    fn names_site_of_every_source() {
        assert!(SOURCES
            .iter()
            .all(|source| Address::new(source.url).domain().is_some()));
    }

    #[test]
    fn is_on_for_everyone_when_it_is_for_everyone() {
        assert!(SOURCES[0].suits("ja"));
    }

    #[test]
    fn is_off_for_those_who_speak_another_language() {
        assert!(!SOURCES
            .iter()
            .any(|source| source.id == "ruadlist" && source.suits("de")));
    }

    #[test]
    fn has_note_in_every_language() {
        assert!(adx0_phrase::LANGUAGES.iter().all(|language| SOURCES
            .iter()
            .all(|source| !language.text(source.note).is_empty())));
    }
}
