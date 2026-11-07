//! The pages the build writes, and the `sitemap.xml` that lists the indexed ones.
//!
//! The build writes every page from [`pages`] and nowhere else, so a page cannot be written
//! and forgotten in the sitemap. Each page names the Markdown file it is written from; the
//! date of the file's last change comes from a text file made by
//! `scripts/content-dates/content_dates.py` (card docs/wip/add-sitemap.md).

use crate::site_address::SiteAddress;
use crate::{about_page, article_page, curriculum_page, home_page, licence_page, not_found_page};
use std::collections::BTreeMap;
use std::fmt;

/// A page the build writes.
pub struct Page {
    /// Where the file is written, relative to the site's root: `about.html`.
    pub file: &'static str,
    /// The page's address relative to the site's root, as the canonical link is made from it
    /// (`site.join(address)`): `about.html`, and `` for the home page.
    pub address: &'static str,
    /// The Markdown file the page is written from, relative to the repository's root.
    pub source: Option<&'static str>,
    /// Whether the page asks search engines to index it, and so belongs to the sitemap.
    pub indexed: bool,
    pub html: String,
}

/// Every page of the site, rendered.
pub fn pages(site: &SiteAddress) -> Vec<Page> {
    let fixed = |file, source, html| Page {
        file,
        address: file,
        source: Some(source),
        indexed: true,
        html,
    };
    let mut pages = vec![
        Page {
            file: "index.html",
            address: "",
            source: None,
            indexed: true,
            html: home_page::render_home_page(site),
        },
        fixed(
            "curriculum.html",
            "content/curriculum.md",
            curriculum_page::render_curriculum_page(site),
        ),
        fixed(
            "about.html",
            "content/about.md",
            about_page::render_about_page(site),
        ),
        fixed(
            "licence.html",
            "content/licence.md",
            licence_page::render_licence_page(site),
        ),
    ];
    for article in &article_page::ARTICLES {
        pages.push(fixed(
            article.address,
            article.source,
            article_page::render_article_page(article, site),
        ));
    }
    pages.push(Page {
        file: "404.html",
        address: "404.html",
        source: None,
        indexed: false,
        html: not_found_page::render_not_found_page(site),
    });
    pages
}

/// A day, `YYYY-MM-DD`: only a well-formed one can be built.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContentDate {
    year: u16,
    month: u8,
    day: u8,
}

impl ContentDate {
    pub fn parse(text: &str) -> Option<Self> {
        let bytes = text.as_bytes();
        if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
            return None;
        }
        let digits = |range: std::ops::Range<usize>| {
            let part = text.get(range)?;
            part.bytes()
                .all(|b| b.is_ascii_digit())
                .then(|| part.parse::<u16>().ok())
                .flatten()
        };
        let year = digits(0..4)?;
        let month = u8::try_from(digits(5..7)?).ok()?;
        let day = u8::try_from(digits(8..10)?).ok()?;
        ((1..=12).contains(&month) && (1..=31).contains(&day)).then_some(Self { year, month, day })
    }
}

impl fmt::Display for ContentDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// The date of the last change of each content file, by path from the repository's root.
#[derive(Debug, Default)]
pub struct ContentDates(BTreeMap<String, ContentDate>);

impl ContentDates {
    /// Reads the dates file: one `<path>\t<YYYY-MM-DD>` line per file.
    pub fn parse(text: &str) -> Result<Self, String> {
        let mut dates = BTreeMap::new();
        for (index, line) in text.lines().enumerate() {
            let malformed = || {
                format!(
                    "malformed line {} in the content dates file: `{line}` (expected `<path>`, a tab, `YYYY-MM-DD`)",
                    index + 1
                )
            };
            let (path, date) = line.split_once('\t').ok_or_else(malformed)?;
            let date = ContentDate::parse(date).ok_or_else(malformed)?;
            if path.is_empty() {
                return Err(malformed());
            }
            dates.insert(path.to_owned(), date);
        }
        Ok(Self(dates))
    }

    fn get(&self, path: &str) -> Option<ContentDate> {
        self.0.get(path).copied()
    }
}

/// The sitemap, and the warnings to show the user.
pub struct Sitemap {
    pub xml: String,
    pub warnings: Vec<String>,
}

/// Writes the sitemap of the indexed pages.
///
/// `dates_file` is the content of the dates file, or why it could not be read. Missing dates are
/// an error in the pipeline (`is_ci`) and a warning, with the entry left without `<lastmod>`,
/// anywhere else. A malformed file is an error in both cases.
pub fn build_sitemap(
    pages: &[Page],
    site: &SiteAddress,
    dates_file: Result<String, String>,
    is_ci: bool,
) -> Result<Sitemap, String> {
    let mut warnings = Vec::new();
    let dates = match dates_file {
        Ok(text) => Some(ContentDates::parse(&text)?),
        Err(problem) if is_ci => return Err(format!("no content dates: {problem}")),
        Err(problem) => {
            warnings.push(format!(
                "no content dates ({problem}): the sitemap has no <lastmod>"
            ));
            None
        }
    };

    let mut listed: Vec<&Page> = pages.iter().filter(|page| page.indexed).collect();
    listed.sort_by_key(|page| (!page.address.is_empty(), page.address));

    let mut missing = Vec::new();
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    for page in listed {
        let lastmod = match (page.source, &dates) {
            (Some(source), Some(dates)) => {
                let date = dates.get(source);
                if date.is_none() {
                    missing.push(format!("{} (from `{source}`)", page.file));
                }
                date
            }
            _ => None,
        };
        xml.push_str(&format!(
            "  <url><loc>{}</loc>",
            escape_xml(&site.join(page.address))
        ));
        if let Some(date) = lastmod {
            xml.push_str(&format!("<lastmod>{date}</lastmod>"));
        }
        xml.push_str("</url>\n");
    }
    xml.push_str("</urlset>\n");

    if !missing.is_empty() {
        let message = format!(
            "no date in the content dates file for: {}",
            missing.join(", ")
        );
        if is_ci {
            return Err(message);
        }
        warnings.push(format!("{message}; written without <lastmod>"));
    }
    Ok(Sitemap { xml, warnings })
}

fn escape_xml(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '\'' => escaped.push_str("&apos;"),
            '"' => escaped.push_str("&quot;"),
            other => escaped.push(other),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};

    fn site() -> SiteAddress {
        SiteAddress::parse("https://www.lucalorenzon.com/").unwrap()
    }

    /// A dates file with a date for every page that has a source.
    fn full_dates(pages: &[Page]) -> String {
        pages
            .iter()
            .filter_map(|page| page.source)
            .map(|source| format!("{source}\t2026-11-06\n"))
            .collect()
    }

    fn sitemap_of(site: &SiteAddress) -> String {
        let pages = pages(site);
        build_sitemap(&pages, site, Ok(full_dates(&pages)), true)
            .unwrap()
            .xml
    }

    /// A structural check: tags open and close in order, and the text has no raw `&`.
    fn assert_well_formed(xml: &str) {
        let mut stack: Vec<&str> = Vec::new();
        let mut rest = xml
            .strip_prefix("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")
            .unwrap();
        while let Some(start) = rest.find('<') {
            let text = &rest[..start];
            assert!(!text.contains('>'), "stray `>` in `{text}`");
            for (i, _) in text.match_indices('&') {
                assert!(
                    ["&amp;", "&lt;", "&gt;", "&apos;", "&quot;"]
                        .iter()
                        .any(|entity| text[i..].starts_with(entity)),
                    "raw `&` in `{text}`"
                );
            }
            let end = rest.find('>').expect("unclosed tag");
            let tag = &rest[start + 1..end];
            if let Some(name) = tag.strip_prefix('/') {
                assert_eq!(stack.pop(), Some(name), "unbalanced `</{name}>`");
            } else {
                stack.push(tag.split_whitespace().next().unwrap());
            }
            rest = &rest[end + 1..];
        }
        assert!(stack.is_empty(), "unclosed: {stack:?}");
        assert_eq!(rest.trim(), "");
    }

    #[test]
    fn is_well_formed_xml_in_the_sitemaps_namespace() {
        let xml = sitemap_of(&site());
        assert_well_formed(&xml);
        assert!(xml.contains("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">"));
        assert!(xml.ends_with("</urlset>\n"));
    }

    #[test]
    fn lists_full_addresses_home_first_then_by_address() {
        let xml = sitemap_of(&site());
        let locs: Vec<&str> = xml
            .lines()
            .filter_map(|l| l.split("<loc>").nth(1)?.split("</loc>").next())
            .collect();
        assert_eq!(
            locs,
            [
                "https://www.lucalorenzon.com/",
                "https://www.lucalorenzon.com/about.html",
                "https://www.lucalorenzon.com/articles/after-save-event.html",
                "https://www.lucalorenzon.com/articles/how-to-generate-hybris-pk-values-without-hybris.html",
                "https://www.lucalorenzon.com/curriculum.html",
                "https://www.lucalorenzon.com/licence.html",
            ]
        );
    }

    #[test]
    fn escapes_xml_special_characters_in_addresses() {
        let site = SiteAddress::parse("https://example.com/a&b<c>'d\"/").unwrap();
        let xml = sitemap_of(&site);
        assert!(xml.contains("<loc>https://example.com/a&amp;b&lt;c&gt;&apos;d&quot;/</loc>"));
        assert!(!xml.contains("a&b"));
    }

    #[test]
    fn dates_every_page_but_the_home_page() {
        let xml = sitemap_of(&site());
        for line in xml.lines().filter(|l| l.contains("<url>")) {
            if line.contains("<loc>https://www.lucalorenzon.com/</loc>") {
                assert!(!line.contains("lastmod"), "{line}");
            } else {
                assert!(
                    line.ends_with("<lastmod>2026-11-06</lastmod></url>"),
                    "{line}"
                );
            }
        }
    }

    #[test]
    fn does_not_list_the_page_not_found() {
        assert!(!sitemap_of(&site()).contains("404"));
    }

    #[test]
    fn indexed_means_not_noindex() {
        for page in pages(&site()) {
            assert_eq!(
                page.indexed,
                !page.html.contains("noindex"),
                "{}",
                page.file
            );
        }
    }

    #[test]
    fn each_listed_page_has_its_address_as_canonical_link() {
        let site = site();
        for page in pages(&site).iter().filter(|p| p.indexed) {
            let canonical = format!(
                "<link rel=\"canonical\" href=\"{}\"",
                site.join(page.address)
            );
            assert!(page.html.contains(&canonical), "{}", page.file);
        }
    }

    fn walk(folder: &Path, found: &mut Vec<PathBuf>) {
        for entry in fs::read_dir(folder).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(&path, found);
            } else {
                found.push(path);
            }
        }
    }

    #[test]
    fn every_markdown_file_in_content_is_the_source_of_exactly_one_entry() {
        // Not pages. Mirrors EXCLUDED in scripts/content-dates/content_dates.py: change both.
        const NOT_PAGES: [&str; 2] = ["content/README.md", "content/LICENSE-MIT-0"];
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut files = Vec::new();
        walk(&root.join("content"), &mut files);
        let pages = pages(&site());
        for file in files {
            let relative = file
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            if relative.ends_with(".md") && !NOT_PAGES.contains(&relative.as_str()) {
                let count = pages
                    .iter()
                    .filter(|p| p.indexed && p.source == Some(relative.as_str()))
                    .count();
                assert_eq!(
                    count, 1,
                    "{relative} is the source of {count} sitemap entries"
                );
            }
        }
    }

    #[test]
    fn every_source_is_a_file_that_exists() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        for page in pages(&site()) {
            if let Some(source) = page.source {
                assert!(root.join(source).is_file(), "{source}");
            }
        }
    }

    #[test]
    fn accepts_only_well_formed_dates() {
        assert!(ContentDate::parse("2026-11-06").is_some());
        for bad in [
            "",
            "2026-11-6",
            "26-11-06",
            "2026/11/06",
            "2026-13-01",
            "2026-00-10",
            "2026-11-00",
            "2026-11-32",
            "2026-1a-06",
            "+026-11-06",
            "2026-11-061",
        ] {
            assert!(ContentDate::parse(bad).is_none(), "{bad}");
        }
    }

    #[test]
    fn tolerant_locally_when_the_file_is_missing() {
        let site = site();
        let pages = pages(&site);
        let sitemap = build_sitemap(&pages, &site, Err("not set".to_owned()), false).unwrap();
        assert!(!sitemap.xml.contains("lastmod"));
        assert_eq!(sitemap.warnings.len(), 1);
        assert!(sitemap.warnings[0].contains("not set"));
    }

    #[test]
    fn strict_in_the_pipeline_when_the_file_is_missing() {
        let site = site();
        let pages = pages(&site);
        let error = build_sitemap(&pages, &site, Err("not set".to_owned()), true)
            .err()
            .unwrap();
        assert!(error.contains("not set"));
    }

    #[test]
    fn tolerant_locally_when_a_page_has_no_date() {
        let site = site();
        let pages = pages(&site);
        let sitemap = build_sitemap(&pages, &site, Ok(String::new()), false).unwrap();
        assert!(!sitemap.xml.contains("lastmod"));
        assert!(sitemap.warnings[0].contains("about.html"));
        assert!(sitemap.warnings[0].contains("content/about.md"));
    }

    #[test]
    fn strict_in_the_pipeline_when_a_page_has_no_date() {
        let site = site();
        let pages = pages(&site);
        let dates = "content/about.md\t2026-11-06\n".to_owned();
        let error = build_sitemap(&pages, &site, Ok(dates), true).err().unwrap();
        assert!(error.contains("curriculum.html"));
        assert!(error.contains("content/curriculum.md"));
        assert!(!error.contains("about.html"));
    }

    #[test]
    fn a_malformed_line_is_an_error_everywhere() {
        let site = site();
        let pages = pages(&site);
        for is_ci in [true, false] {
            for text in [
                "content/about.md 2026-11-06\n",
                "content/about.md\t2026-13-06\n",
                "\n",
            ] {
                let error = build_sitemap(&pages, &site, Ok(text.to_owned()), is_ci)
                    .err()
                    .unwrap();
                assert!(error.contains("malformed line 1"), "{error}");
            }
        }
    }
}
