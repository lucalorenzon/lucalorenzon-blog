//! The `robots.txt` the build writes into the site's root (card docs/wip/add-robots-txt.md).
//!
//! It points search engines to the sitemap and asks the robots that collect text to train AI
//! models to stay away. No `User-agent: *` group: whatever is not named stays allowed, so
//! ordinary search engines and assistants answering a user are welcome. The blocked names come
//! from a file made by `scripts/robots-list/robots_list.py`.

use crate::site_address::SiteAddress;
use crate::sitemap::sitemap_address;
use std::fmt;

/// Where the build writes the file, relative to the site's root.
pub const ROBOTS_FILE: &str = "robots.txt";

/// The shared signal against AI training (draft-kelley-aipref-training-preference-signal).
const TRAINING_SIGNAL: &str = "__usage_ai-training";

/// A robot's name as `robots.txt` can hold it: not empty, trimmed, without control characters,
/// `#` (it starts a comment) or `:` (it ends the field name).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RobotName(String);

/// Why a text is not a robot's name.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RobotNameError {
    #[error("it is empty")]
    Empty,
    #[error("it has spaces at its start or end")]
    Spaces,
    #[error("it has the character {0:?}")]
    Character(char),
}

impl RobotName {
    pub fn parse(text: &str) -> Result<Self, RobotNameError> {
        if text.is_empty() {
            return Err(RobotNameError::Empty);
        }
        if text != text.trim() {
            return Err(RobotNameError::Spaces);
        }
        if let Some(bad) = text
            .chars()
            .find(|c| c.is_control() || *c == '#' || *c == ':')
        {
            return Err(RobotNameError::Character(bad));
        }
        Ok(Self(text.to_owned()))
    }
}

impl fmt::Display for RobotName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A line of the blocked robots file that is not a robot's name.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("malformed line {line} in the blocked robots file: `{text}` ({reason})")]
pub struct MalformedRobotLine {
    /// Counted from 1.
    pub line: usize,
    pub text: String,
    pub reason: RobotNameError,
}

/// Reads the list: one robot name per line.
pub fn parse_blocked_robots(text: &str) -> Result<Vec<RobotName>, MalformedRobotLine> {
    text.lines()
        .enumerate()
        .map(|(index, line)| {
            RobotName::parse(line).map_err(|reason| MalformedRobotLine {
                line: index + 1,
                text: line.to_owned(),
                reason,
            })
        })
        .collect()
}

/// The text of the `robots.txt`: the training signal group, the group of the blocked robots
/// when there are any, and the sitemap's address.
pub fn robots_txt(site: &SiteAddress, blocked: &[RobotName]) -> String {
    let mut text =
        String::from("# Search engines and assistants are welcome; AI training is not.\n\n");
    text.push_str(&format!("User-agent: {TRAINING_SIGNAL}\nDisallow: /\n\n"));
    if !blocked.is_empty() {
        for name in blocked {
            text.push_str(&format!("User-agent: {name}\n"));
        }
        text.push_str("Disallow: /\n\n");
    }
    text.push_str(&format!("Sitemap: {}\n", sitemap_address(site)));
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sitemap;
    use std::fs;
    use std::path::Path;

    fn site() -> SiteAddress {
        SiteAddress::parse("https://example.com/blog/").unwrap()
    }

    const LIST: &str = "AI2Bot\nBrightbot 1.0\nSome/Bot-1.0_x\n";

    fn list(text: &str) -> Vec<RobotName> {
        parse_blocked_robots(text).unwrap()
    }

    #[test]
    fn writes_the_exact_text() {
        let text = robots_txt(&site(), &list(LIST));
        assert_eq!(
            text,
            "# Search engines and assistants are welcome; AI training is not.\n\
             \n\
             User-agent: __usage_ai-training\n\
             Disallow: /\n\
             \n\
             User-agent: AI2Bot\n\
             User-agent: Brightbot 1.0\n\
             User-agent: Some/Bot-1.0_x\n\
             Disallow: /\n\
             \n\
             Sitemap: https://example.com/blog/sitemap.xml\n"
        );
    }

    #[test]
    fn points_to_the_address_the_sitemap_has() {
        let site = site();
        let text = robots_txt(&site, &list(LIST));
        let last = text.lines().last().unwrap();
        assert_eq!(
            last,
            format!("Sitemap: {}", sitemap::sitemap_address(&site))
        );
        let pages = sitemap::pages(&site, &crate::article_page::real_articles());
        let dates: String = pages
            .iter()
            .filter_map(|p| p.source)
            .map(|s| format!("{s}\t2026-11-06\n"))
            .collect();
        let dates = sitemap::ContentDates::parse(&dates).unwrap();
        let xml = sitemap::build_sitemap(&pages, &site, Some(&dates)).xml;
        assert!(xml.contains("<loc>https://example.com/blog/about.html</loc>"));
        assert!(sitemap::sitemap_address(&site).starts_with("https://example.com/blog/"));
        assert!(sitemap::sitemap_address(&site).ends_with(sitemap::SITEMAP_FILE));
    }

    #[test]
    fn blocks_only_the_names_it_is_given() {
        let text = robots_txt(&site(), &list(LIST));
        for engine in [
            "Googlebot",
            "Bingbot",
            "DuckDuckBot",
            "Applebot",
            "YandexBot",
        ] {
            assert!(!text.contains(engine), "{engine}");
        }
        assert!(!text.contains("User-agent: *"));
        let agents = text
            .lines()
            .filter(|l| l.starts_with("User-agent:"))
            .count();
        assert_eq!(agents, 1 + 3);
    }

    #[test]
    fn a_search_engine_is_blocked_only_when_the_list_names_it() {
        let text = robots_txt(&site(), &list("Googlebot\n"));
        assert!(text.contains("User-agent: Googlebot\n"));
    }

    /// The rule that lets robots through lives in `robots_list.py`; this module must not
    /// repeat it, or the two could disagree.
    #[test]
    fn does_not_repeat_the_lists_exceptions_or_allowed_functions() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let python = fs::read_to_string(root.join("scripts/robots-list/robots_list.py")).unwrap();
        let source = include_str!("robots_txt.rs");
        let code = source.split("#[cfg(test)]").next().unwrap();
        for tuple in ["EXCEPTIONS = (", "ALLOWED_FUNCTIONS = ("] {
            let body = python.split(tuple).nth(1).expect(tuple);
            let body = body.split("\n)").next().unwrap();
            let values: Vec<&str> = body.split('"').skip(1).step_by(2).collect();
            assert!(!values.is_empty(), "{tuple}");
            for value in values {
                assert!(!code.contains(value), "robots_txt.rs mentions `{value}`");
            }
        }
    }

    #[test]
    fn accepts_names_with_spaces_slashes_and_dots() {
        for name in ["Brightbot 1.0", "Some/Bot-1.0", "a_b", "AI2Bot"] {
            assert_eq!(RobotName::parse(name).unwrap().to_string(), name);
        }
    }

    #[test]
    fn a_malformed_line_is_an_error() {
        for (text, line, reason) in [
            ("A\n\nB\n", 2, RobotNameError::Empty),
            ("A\n#B\n", 2, RobotNameError::Character('#')),
            ("A:B\n", 1, RobotNameError::Character(':')),
            ("A\u{7}B\n", 1, RobotNameError::Character('\u{7}')),
            (" A\n", 1, RobotNameError::Spaces),
            ("A\t\n", 1, RobotNameError::Spaces),
        ] {
            let error = parse_blocked_robots(text).unwrap_err();
            assert_eq!(error.line, line, "{text:?}");
            assert_eq!(error.reason, reason, "{text:?}");
        }
    }

    #[test]
    fn the_malformed_line_error_reads_as_before() {
        let error = parse_blocked_robots("A\n#B\n").unwrap_err();
        assert_eq!(
            error.to_string(),
            "malformed line 2 in the blocked robots file: `#B` (it has the character '#')"
        );
    }

    #[test]
    fn an_empty_list_leaves_only_the_training_signal() {
        let text = robots_txt(&site(), &[]);
        assert!(text.contains("User-agent: __usage_ai-training\nDisallow: /\n"));
        assert!(text.ends_with("Sitemap: https://example.com/blog/sitemap.xml\n"));
        assert_eq!(text.matches("User-agent:").count(), 1);
    }
}
