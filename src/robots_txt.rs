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

impl RobotName {
    pub fn parse(text: &str) -> Result<Self, String> {
        if text.is_empty() {
            return Err("it is empty".to_owned());
        }
        if text != text.trim() {
            return Err("it has spaces at its start or end".to_owned());
        }
        if let Some(bad) = text
            .chars()
            .find(|c| c.is_control() || *c == '#' || *c == ':')
        {
            return Err(format!("it has the character {bad:?}"));
        }
        Ok(Self(text.to_owned()))
    }
}

impl fmt::Display for RobotName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Reads the list: one robot name per line.
pub fn parse_blocked_robots(text: &str) -> Result<Vec<RobotName>, String> {
    text.lines()
        .enumerate()
        .map(|(index, line)| {
            RobotName::parse(line).map_err(|reason| {
                format!(
                    "malformed line {} in the blocked robots file: `{line}` ({reason})",
                    index + 1
                )
            })
        })
        .collect()
}

/// The `robots.txt`, and the warnings to show the user.
pub struct RobotsTxt {
    pub text: String,
    pub warnings: Vec<String>,
}

/// Writes the `robots.txt`.
///
/// `blocked_file` is the content of the blocked robots file, or why it could not be read. A
/// missing file is an error in the pipeline (`is_ci`) and a warning anywhere else, with the
/// blocked group left out. A malformed file is an error in both cases.
pub fn build_robots_txt(
    site: &SiteAddress,
    blocked_file: Result<String, String>,
    is_ci: bool,
) -> Result<RobotsTxt, String> {
    let mut warnings = Vec::new();
    let blocked = match blocked_file {
        Ok(text) => parse_blocked_robots(&text)?,
        Err(problem) if is_ci => return Err(format!("no blocked robots list: {problem}")),
        Err(problem) => {
            warnings.push(format!(
                "no blocked robots list ({problem}): the robots.txt blocks only the training signal"
            ));
            Vec::new()
        }
    };
    Ok(RobotsTxt {
        text: render(site, &blocked),
        warnings,
    })
}

fn render(site: &SiteAddress, blocked: &[RobotName]) -> String {
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

    #[test]
    fn writes_the_exact_text() {
        let robots = build_robots_txt(&site(), Ok(LIST.to_owned()), true).unwrap();
        assert!(robots.warnings.is_empty());
        assert_eq!(
            robots.text,
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
        let robots = build_robots_txt(&site, Ok(LIST.to_owned()), true).unwrap();
        let last = robots.text.lines().last().unwrap();
        assert_eq!(
            last,
            format!("Sitemap: {}", sitemap::sitemap_address(&site))
        );
        let pages = sitemap::pages(&site);
        let dates: String = pages
            .iter()
            .filter_map(|p| p.source)
            .map(|s| format!("{s}\t2026-11-06\n"))
            .collect();
        let xml = sitemap::build_sitemap(&pages, &site, Ok(dates), true)
            .unwrap()
            .xml;
        assert!(xml.contains("<loc>https://example.com/blog/about.html</loc>"));
        assert!(sitemap::sitemap_address(&site).starts_with("https://example.com/blog/"));
        assert!(sitemap::sitemap_address(&site).ends_with(sitemap::SITEMAP_FILE));
    }

    #[test]
    fn blocks_only_the_names_it_is_given() {
        let robots = build_robots_txt(&site(), Ok(LIST.to_owned()), true).unwrap();
        for engine in [
            "Googlebot",
            "Bingbot",
            "DuckDuckBot",
            "Applebot",
            "YandexBot",
        ] {
            assert!(!robots.text.contains(engine), "{engine}");
        }
        assert!(!robots.text.contains("User-agent: *"));
        let agents = robots
            .text
            .lines()
            .filter(|l| l.starts_with("User-agent:"))
            .count();
        assert_eq!(agents, 1 + 3);
    }

    #[test]
    fn a_search_engine_is_blocked_only_when_the_list_names_it() {
        let robots = build_robots_txt(&site(), Ok("Googlebot\n".to_owned()), true).unwrap();
        assert!(robots.text.contains("User-agent: Googlebot\n"));
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
    fn a_malformed_line_is_an_error_everywhere() {
        for is_ci in [true, false] {
            for (text, line) in [
                ("A\n\nB\n", "2"),
                ("A\n#B\n", "2"),
                ("A:B\n", "1"),
                ("A\u{7}B\n", "1"),
                (" A\n", "1"),
                ("A\t\n", "1"),
            ] {
                let error = build_robots_txt(&site(), Ok(text.to_owned()), is_ci)
                    .err()
                    .unwrap();
                assert!(error.contains(&format!("malformed line {line}")), "{error}");
            }
        }
    }

    #[test]
    fn strict_in_the_pipeline_when_the_list_is_missing() {
        let error = build_robots_txt(&site(), Err("not set".to_owned()), true)
            .err()
            .unwrap();
        assert!(error.contains("not set"));
    }

    #[test]
    fn tolerant_locally_when_the_list_is_missing() {
        let robots = build_robots_txt(&site(), Err("not set".to_owned()), false).unwrap();
        assert_eq!(robots.warnings.len(), 1);
        assert!(robots.warnings[0].contains("not set"));
        assert!(
            robots
                .text
                .contains("User-agent: __usage_ai-training\nDisallow: /\n")
        );
        assert!(
            robots
                .text
                .ends_with("Sitemap: https://example.com/blog/sitemap.xml\n")
        );
        assert_eq!(robots.text.matches("User-agent:").count(), 1);
    }
}
