//! The pages the build writes, and the `sitemap.xml` that lists the indexed ones.
//!
//! The build writes every page from [`pages`] and nowhere else, so a page cannot be written
//! and forgotten in the sitemap. Each page names the Markdown file it is written from; the
//! date of the file's last change comes from a text file made by
//! `scripts/content-dates/content_dates.py` (card docs/wip/add-sitemap.md).

use crate::article_page::Article;
use crate::site_address::SiteAddress;
use crate::style_guide::{self, colours_page, index_page};
use crate::{about_page, article_page, curriculum_page, home_page, licence_page, not_found_page};
use jiff::civil::Date;
use std::collections::BTreeMap;

/// Where the build writes the sitemap, relative to the site's root.
pub const SITEMAP_FILE: &str = "sitemap.xml";

/// The sitemap's full address, as `robots.txt` points to it.
pub fn sitemap_address(site: &SiteAddress) -> String {
    site.join(SITEMAP_FILE)
}

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

/// Every page of the site, rendered. The colours page can fail: its text must match the
/// palette.
pub fn pages(
    site: &SiteAddress,
    articles: &[Article],
) -> Result<Vec<Page>, colours_page::SectionError> {
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
            html: home_page::render_home_page(site, articles),
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
        Page {
            file: "style-guide/index.html",
            address: style_guide::FOLDER,
            source: Some("content/style-guide/index.md"),
            indexed: true,
            html: index_page::render_index_page(site),
        },
        fixed(
            "style-guide/colours.html",
            "content/style-guide/colours.md",
            colours_page::render_colours_page(site)?,
        ),
    ];
    for article in articles {
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
    Ok(pages)
}

/// The date of the last change of each content file, by path from the repository's root.
#[derive(Debug, Default)]
pub struct ContentDates(BTreeMap<String, Date>);

/// A line of the dates file that is not `<path>`, a tab, `YYYY-MM-DD`.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error(
    "malformed line {line} in the content dates file: `{text}` (expected `<path>`, a tab, `YYYY-MM-DD`)"
)]
pub struct MalformedDateLine {
    /// The line's number, from 1.
    pub line: usize,
    pub text: String,
}

impl ContentDates {
    /// Reads the dates file: one `<path>\t<YYYY-MM-DD>` line per file.
    pub fn parse(text: &str) -> Result<Self, MalformedDateLine> {
        let mut dates = BTreeMap::new();
        for (index, line) in text.lines().enumerate() {
            let malformed = || MalformedDateLine {
                line: index + 1,
                text: line.to_owned(),
            };
            let (path, date) = line.split_once('\t').ok_or_else(malformed)?;
            let date = date.parse::<Date>().map_err(|_| malformed())?;
            if path.is_empty() {
                return Err(malformed());
            }
            dates.insert(path.to_owned(), date);
        }
        Ok(Self(dates))
    }

    fn get(&self, path: &str) -> Option<Date> {
        self.0.get(path).copied()
    }
}

/// An indexed page written from a Markdown file that has no date in the dates file.
#[derive(Debug, PartialEq, Eq)]
pub struct MissingDate {
    /// The page's file, as in [`Page::file`].
    pub file: &'static str,
    /// The Markdown file the page is written from, as in [`Page::source`].
    pub source: &'static str,
}

fn list_missing(missing: &[MissingDate]) -> String {
    missing
        .iter()
        .map(|page| format!("{} (from `{}`)", page.file, page.source))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The indexed pages that have a source file but no date in the dates file.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("no date in the content dates file for: {}", list_missing(.0))]
pub struct MissingDates(pub Vec<MissingDate>);

/// The sitemap, and the pages it could not date.
pub struct Sitemap {
    pub xml: String,
    /// `None` when every page with a source has a date, or when there are no dates at all.
    pub missing: Option<MissingDates>,
}

/// Writes the sitemap of the indexed pages. A page without a date is listed without
/// `<lastmod>` and reported in `missing`; what to do about it is the caller's.
pub fn build_sitemap(pages: &[Page], site: &SiteAddress, dates: Option<&ContentDates>) -> Sitemap {
    let mut listed: Vec<&Page> = pages.iter().filter(|page| page.indexed).collect();
    listed.sort_by_key(|page| (!page.address.is_empty(), page.address));

    let mut missing = Vec::new();
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    for page in listed {
        let lastmod = match (page.source, dates) {
            (Some(source), Some(dates)) => {
                let date = dates.get(source);
                if date.is_none() {
                    missing.push(MissingDate {
                        file: page.file,
                        source,
                    });
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

    Sitemap {
        xml,
        missing: (!missing.is_empty()).then_some(MissingDates(missing)),
    }
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
    use crate::article_page::real_articles;
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
        let pages = pages(site, &real_articles()).unwrap();
        let dates = ContentDates::parse(&full_dates(&pages)).unwrap();
        build_sitemap(&pages, site, Some(&dates)).xml
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
                "https://www.lucalorenzon.com/style-guide/",
                "https://www.lucalorenzon.com/style-guide/colours.html",
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
        for page in pages(&site(), &real_articles()).unwrap() {
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
        for page in pages(&site, &real_articles())
            .unwrap()
            .iter()
            .filter(|p| p.indexed)
        {
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
        let pages = pages(&site(), &real_articles()).unwrap();
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
        for page in pages(&site(), &real_articles()).unwrap() {
            if let Some(source) = page.source {
                assert!(root.join(source).is_file(), "{source}");
            }
        }
    }

    /// `every_source_is_a_file_that_exists` would not notice two fixed pages whose
    /// sources were swapped; this checks each source holds the text its page renders.
    #[test]
    fn each_fixed_page_names_the_file_its_text_comes_from() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let pages = pages(&site(), &real_articles()).unwrap();
        for (file, text) in [
            ("curriculum.html", crate::curriculum_page::CURRICULUM),
            ("about.html", crate::about_page::ABOUT),
            ("licence.html", crate::licence_page::LICENCE),
            (
                "style-guide/index.html",
                crate::style_guide::index_page::INDEX,
            ),
            (
                "style-guide/colours.html",
                crate::style_guide::colours_page::COLOURS,
            ),
        ] {
            let page = pages.iter().find(|p| p.file == file).unwrap();
            let source = page.source.unwrap();
            assert_eq!(
                fs::read_to_string(root.join(source)).unwrap(),
                text,
                "{file}"
            );
        }
    }

    #[test]
    fn accepts_only_well_formed_dates() {
        let read = |date: &str| ContentDates::parse(&format!("about.md\t{date}"));
        assert!(read("2026-11-06").is_ok());
        for bad in [
            "",
            "2026-11-6",
            "26-11-06",
            "2026/11/06",
            "2026-13-01",
            "2026-00-10",
            "2026-11-00",
            "2026-11-32",
            "2026-02-31",
            "2026-1a-06",
            "+026-11-06",
            "2026-11-061",
        ] {
            assert!(read(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn without_dates_no_entry_has_lastmod_and_nothing_is_reported() {
        let site = site();
        let pages = pages(&site, &real_articles()).unwrap();
        let sitemap = build_sitemap(&pages, &site, None);
        assert!(!sitemap.xml.contains("lastmod"));
        assert_eq!(sitemap.missing, None);
    }

    #[test]
    fn with_an_empty_dates_file_every_page_with_a_source_is_missing() {
        let site = site();
        let pages = pages(&site, &real_articles()).unwrap();
        let dates = ContentDates::parse("").unwrap();
        let sitemap = build_sitemap(&pages, &site, Some(&dates));
        assert!(!sitemap.xml.contains("lastmod"));
        let missing = sitemap.missing.unwrap();
        assert!(missing.0.contains(&MissingDate {
            file: "about.html",
            source: "content/about.md",
        }));
        let message = missing.to_string();
        assert!(message.starts_with("no date in the content dates file for: "));
        assert!(message.contains("about.html (from `content/about.md`)"));
    }

    #[test]
    fn a_page_without_a_date_is_reported_and_listed_without_lastmod() {
        let site = site();
        let pages = pages(&site, &real_articles()).unwrap();
        let dates = ContentDates::parse("content/about.md\t2026-11-06\n").unwrap();
        let sitemap = build_sitemap(&pages, &site, Some(&dates));
        let message = sitemap.missing.unwrap().to_string();
        assert!(message.contains("curriculum.html (from `content/curriculum.md`)"));
        assert!(!message.contains("about.html"));
        let curriculum = sitemap
            .xml
            .lines()
            .find(|l| l.contains("/curriculum.html"))
            .unwrap();
        assert!(!curriculum.contains("lastmod"));
        let about = sitemap
            .xml
            .lines()
            .find(|l| l.contains("/about.html"))
            .unwrap();
        assert!(about.contains("<lastmod>2026-11-06</lastmod>"));
    }

    #[test]
    fn nothing_is_reported_when_every_page_has_a_date() {
        let site = site();
        let pages = pages(&site, &real_articles()).unwrap();
        let dates = ContentDates::parse(&full_dates(&pages)).unwrap();
        assert_eq!(build_sitemap(&pages, &site, Some(&dates)).missing, None);
    }

    #[test]
    fn missing_dates_read_as_a_list_joined_with_commas() {
        let missing = MissingDates(vec![
            MissingDate {
                file: "a.html",
                source: "content/a.md",
            },
            MissingDate {
                file: "b.html",
                source: "content/b.md",
            },
        ]);
        assert_eq!(
            missing.to_string(),
            "no date in the content dates file for: a.html (from `content/a.md`), b.html (from `content/b.md`)"
        );
    }

    #[test]
    fn a_malformed_line_is_reported_with_its_number() {
        for (text, line, bad) in [
            (
                "content/about.md 2026-11-06\n",
                1,
                "content/about.md 2026-11-06",
            ),
            (
                "content/about.md\t2026-13-06\n",
                1,
                "content/about.md\t2026-13-06",
            ),
            ("\n", 1, ""),
            ("a.md\t2026-11-06\nb.md\tnope\n", 2, "b.md\tnope"),
        ] {
            let error = ContentDates::parse(text).unwrap_err();
            assert_eq!(
                error,
                MalformedDateLine {
                    line,
                    text: bad.to_owned()
                }
            );
            assert!(error.to_string().starts_with(&format!(
                "malformed line {line} in the content dates file: `{bad}` (expected"
            )));
        }
    }
}
