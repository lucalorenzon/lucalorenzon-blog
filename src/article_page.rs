use crate::markdown;
use crate::page_frame::{FooterLicence, IndexedPage, Indexing, PageFrame, PageKind, SITE_NAME};
use crate::site_address::SiteAddress;
use leptos::prelude::*;
use serde::Deserialize;
use std::fmt;

/// A day of the calendar. Only a real one can be built: from `YYYY-MM-DD`, with `TryFrom`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(try_from = "String")]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

/// Why a text is not a [`Date`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DateError {
    /// Not written as `YYYY-MM-DD`.
    NotIsoFormat(String),
    /// Written as `YYYY-MM-DD`, but no such day exists: `2023-02-30`.
    NotACalendarDay(String),
}

impl fmt::Display for DateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotIsoFormat(text) => {
                write!(
                    f,
                    "field `date`: `{text}` is not a date written as YYYY-MM-DD"
                )
            }
            Self::NotACalendarDay(text) => {
                write!(f, "field `date`: `{text}` is not a day of the calendar")
            }
        }
    }
}

impl std::error::Error for DateError {}

impl TryFrom<String> for Date {
    type Error = DateError;

    fn try_from(text: String) -> Result<Self, DateError> {
        Self::try_from(text.as_str())
    }
}

impl TryFrom<&str> for Date {
    type Error = DateError;

    fn try_from(text: &str) -> Result<Self, DateError> {
        let not_iso = || DateError::NotIsoFormat(text.to_owned());
        let bytes = text.as_bytes();
        if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
            return Err(not_iso());
        }
        let digits = |range: std::ops::Range<usize>| -> Result<u16, DateError> {
            let part = text.get(range).ok_or_else(not_iso)?;
            if !part.bytes().all(|b| b.is_ascii_digit()) {
                return Err(not_iso());
            }
            part.parse::<u16>().map_err(|_| not_iso())
        };
        let year = digits(0..4)?;
        let month = u8::try_from(digits(5..7)?).map_err(|_| not_iso())?;
        let day = u8::try_from(digits(8..10)?).map_err(|_| not_iso())?;
        let not_a_day = || DateError::NotACalendarDay(text.to_owned());
        if year == 0 || !(1..=12).contains(&month) {
            return Err(not_a_day());
        }
        if day == 0 || day > Self::days_in_month(year, month) {
            return Err(not_a_day());
        }
        Ok(Self { year, month, day })
    }
}

impl Date {
    const MONTHS: [&'static str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    fn days_in_month(year: u16, month: u8) -> u8 {
        let leap =
            (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400);
        match month {
            2 if leap => 29,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }

    /// The machine-readable form, for `<time datetime>`: `2023-04-30`.
    pub fn iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// The form a reader sees: `30 April 2023`.
    pub fn long_form(self) -> String {
        format!(
            "{} {} {}",
            self.day,
            Self::MONTHS[usize::from(self.month) - 1],
            self.year
        )
    }
}

/// The title of an article: the page's heading. Not empty.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct Title(String);

/// Why a text is not a [`Title`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TitleError {
    Empty,
}

impl fmt::Display for TitleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("field `title` is empty"),
        }
    }
}

impl std::error::Error for TitleError {}

impl TryFrom<String> for Title {
    type Error = TitleError;

    fn try_from(text: String) -> Result<Self, TitleError> {
        Self::try_from(text.as_str())
    }
}

impl TryFrom<&str> for Title {
    type Error = TitleError;

    fn try_from(text: &str) -> Result<Self, TitleError> {
        let text = text.trim();
        if text.is_empty() {
            return Err(TitleError::Empty);
        }
        Ok(Self(text.to_owned()))
    }
}

impl Title {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The text of the browser's tab: the title, then the site's name.
    pub fn tab_title(&self) -> String {
        format!("{} - {}", self.0, SITE_NAME)
    }
}

/// The longest description a search result shows whole.
pub const DESCRIPTION_MAX_CHARACTERS: usize = 160;

/// The description of an article: what search engines and link previews show. Not empty, at
/// most [`DESCRIPTION_MAX_CHARACTERS`] characters.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct Description(String);

/// Why a text is not a [`Description`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DescriptionError {
    Empty,
    TooLong { characters: usize },
}

impl fmt::Display for DescriptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("field `description` is empty"),
            Self::TooLong { characters } => write!(
                f,
                "field `description` has {characters} characters, more than the {DESCRIPTION_MAX_CHARACTERS} a search result shows"
            ),
        }
    }
}

impl std::error::Error for DescriptionError {}

impl TryFrom<String> for Description {
    type Error = DescriptionError;

    fn try_from(text: String) -> Result<Self, DescriptionError> {
        Self::try_from(text.as_str())
    }
}

impl TryFrom<&str> for Description {
    type Error = DescriptionError;

    fn try_from(text: &str) -> Result<Self, DescriptionError> {
        let text = text.trim();
        if text.is_empty() {
            return Err(DescriptionError::Empty);
        }
        let characters = text.chars().count();
        if characters > DESCRIPTION_MAX_CHARACTERS {
            return Err(DescriptionError::TooLong { characters });
        }
        Ok(Self(text.to_owned()))
    }
}

impl Description {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An article's file as the code knows it: where the page is written, where the Markdown is,
/// and the Markdown itself. The title, the description and the date are in the Markdown's front
/// matter, and are read by [`Article::try_from`].
pub struct ArticleFile {
    /// Where the page is written, relative to the site's root.
    pub address: &'static str,
    /// The Markdown file `text` is read from, relative to the repository's root (the sitemap's
    /// dates are looked up by it; a test checks it names the file `text` comes from).
    pub source: &'static str,
    text: &'static str,
}

/// Every article file of the site. A new article is added here, and nowhere else.
pub const ARTICLE_FILES: [ArticleFile; 2] = [
    ArticleFile {
        address: "articles/after-save-event.html",
        source: "content/articles/after-save-event.md",
        text: include_str!("../content/articles/after-save-event.md"),
    },
    ArticleFile {
        address: "articles/how-to-generate-hybris-pk-values-without-hybris.html",
        source: "content/articles/how-to-generate-hybris-pk-values-without-hybris.md",
        text: include_str!(
            "../content/articles/how-to-generate-hybris-pk-values-without-hybris.md"
        ),
    },
];

/// What the front matter of an article holds. The fields are the validated types: reading the
/// YAML builds them, so a front matter that is read is a valid one. Any other field is an
/// error, so a misspelt one is reported instead of silently ignored.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FrontMatter {
    title: Title,
    description: Description,
    date: Date,
}

/// What is wrong in an article's front matter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FrontMatterError {
    /// The file does not start with a line `---`.
    Missing,
    /// The front matter has no closing line `---`.
    Unclosed,
    /// The YAML reader's message: malformed YAML, a field missing, an unknown field, a field
    /// of the wrong kind, or a field whose value is not valid (`Title`, `Description`, `Date`).
    Invalid(String),
}

impl fmt::Display for FrontMatterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing => {
                f.write_str("the file does not start with a front matter (a line `---`)")
            }
            Self::Unclosed => f.write_str("the front matter has no closing line `---`"),
            Self::Invalid(problem) => write!(
                f,
                "front matter: {}",
                problem.lines().next().unwrap_or_default()
            ),
        }
    }
}

impl std::error::Error for FrontMatterError {}

/// Why an article's file cannot become an [`Article`]: which file, and what is wrong in its
/// front matter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArticleError {
    pub source: &'static str,
    pub error: FrontMatterError,
}

impl fmt::Display for ArticleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.source, self.error)
    }
}

impl std::error::Error for ArticleError {}

/// Splits a file into its front matter (with its opening `---` line) and its body. The front matter sits between a first line
/// `---` and the next one.
fn split_front_matter(text: &str) -> Result<(&str, &str), FrontMatterError> {
    let rest = text
        .strip_prefix("---\n")
        .or_else(|| text.strip_prefix("---\r\n"))
        .ok_or(FrontMatterError::Missing)?;
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        if line.trim_end() == "---" {
            // The reader gets the opening `---` too (the YAML document start), so the lines it
            // reports are the file's.
            let opening = text.len() - rest.len();
            return Ok((&text[..opening + offset], &rest[offset + line.len()..]));
        }
        offset += line.len();
    }
    Err(FrontMatterError::Unclosed)
}

/// What the site knows of an article. It can only be built from a file whose front matter is
/// valid, so the fields are not checked again further on.
#[derive(Clone, Debug)]
pub struct Article {
    pub title: Title,
    pub description: Description,
    pub date: Date,
    /// Where the page is written, relative to the site's root.
    pub address: &'static str,
    /// The Markdown file, relative to the repository's root.
    pub source: &'static str,
    /// The Markdown after the front matter.
    body: String,
}

impl TryFrom<&ArticleFile> for Article {
    type Error = ArticleError;

    fn try_from(file: &ArticleFile) -> Result<Self, ArticleError> {
        let error = |error| ArticleError {
            source: file.source,
            error,
        };
        let (front_matter, body) = split_front_matter(file.text).map_err(error)?;
        let fields: FrontMatter = serde_saphyr::from_str(front_matter)
            .map_err(|problem| error(FrontMatterError::Invalid(problem.to_string())))?;
        Ok(Self {
            title: fields.title,
            description: fields.description,
            date: fields.date,
            address: file.address,
            source: file.source,
            body: body.to_owned(),
        })
    }
}

/// Builds every article it can. The others are returned with why, in the order of the files.
pub fn load_articles(files: &[ArticleFile]) -> (Vec<Article>, Vec<ArticleError>) {
    let mut articles = Vec::new();
    let mut skipped = Vec::new();
    for file in files {
        match Article::try_from(file) {
            Ok(article) => articles.push(article),
            Err(error) => skipped.push(error),
        }
    }
    (articles, skipped)
}

/// The articles the build goes on with, and what to tell the user about the others.
#[derive(Debug)]
pub struct LoadedArticles {
    pub articles: Vec<Article>,
    /// One line per skipped article, to show when the build starts.
    pub warnings: Vec<String>,
    /// The list of the skipped articles, to show at the end of the build; `None` if none.
    pub summary: Option<String>,
}

/// Decides what a problem in an article means. In the pipeline (`is_ci`) what is built is what
/// gets published, so any problem stops the build, with all the problems listed. Anywhere else
/// the article is skipped, with a warning, and listed in a summary.
pub fn settle(
    loaded: (Vec<Article>, Vec<ArticleError>),
    is_ci: bool,
) -> Result<LoadedArticles, String> {
    let (articles, skipped) = loaded;
    if skipped.is_empty() {
        return Ok(LoadedArticles {
            articles,
            warnings: Vec::new(),
            summary: None,
        });
    }
    if is_ci {
        let lines: Vec<String> = skipped.iter().map(|e| format!("  - {e}")).collect();
        return Err(format!(
            "{} article(s) have problems:\n{}",
            skipped.len(),
            lines.join("\n")
        ));
    }
    let warnings = skipped
        .iter()
        .map(|e| format!("article skipped: {e}"))
        .collect();
    let lines: Vec<String> = skipped.iter().map(|e| format!("  - {e}")).collect();
    let summary = format!(
        "{} article(s) skipped, not in the site:\n{}",
        skipped.len(),
        lines.join("\n")
    );
    Ok(LoadedArticles {
        articles,
        warnings,
        summary: Some(summary),
    })
}

/// The articles, newest first.
pub fn newest_first(articles: &[Article]) -> Vec<&Article> {
    let mut sorted: Vec<&Article> = articles.iter().collect();
    sorted.sort_by_key(|article| std::cmp::Reverse(article.date));
    sorted
}

/// A whole article page document, `<html>` element included. The page lives in the `articles/`
/// folder, so every address is relative to that folder (ADR-002). The Markdown has no title:
/// the title is the page heading.
#[component]
pub fn ArticlePage(article: Article, canonical: String) -> impl IntoView {
    let html = markdown::to_html(&article.body);
    view! {
        <PageFrame
            title=article.title.tab_title()
            heading=article.title.as_str().to_owned()
            address_prefix="../"
            footer_licence=FooterLicence::ReuseWithAttribution
            indexing=Indexing::Indexed(IndexedPage {
                canonical,
                description: article.description.as_str().to_owned(),
                kind: PageKind::Article { published: article.date.iso() },
            })
        >
            <main class="long-text-column md:long-text-large" inner_html=html></main>
        </PageFrame>
    }
}

/// Renders an article page to a complete HTML document string. Its canonical link is the
/// site's full address followed by the article's address.
pub fn render_article_page(article: &Article, site: &SiteAddress) -> String {
    let owner = Owner::new();
    let canonical = site.join(article.address);
    let article = article.clone();
    let body =
        owner.with(|| view! { <ArticlePage article=article canonical=canonical /> }.to_html());
    format!("<!DOCTYPE html>{body}")
}

/// The real articles of the site, for the tests of the other pages. Panics if one is invalid:
/// `every_real_article_builds` says which.
#[cfg(test)]
pub fn real_articles() -> Vec<Article> {
    let (articles, skipped) = load_articles(&ARTICLE_FILES);
    assert!(skipped.is_empty(), "{skipped:?}");
    articles
}

#[cfg(test)]
mod tests {

    mod after_save_event {
        use super::super::*;

        fn page() -> String {
            render_article_page(&real_articles()[0], &site())
        }

        fn site() -> SiteAddress {
            SiteAddress::parse("https://www.lucalorenzon.com/").unwrap()
        }

        #[test]
        fn has_its_description_and_preview_tags() {
            let html = page();
            assert!(html.contains("<meta name=\"description\" content=\"SAP hybris change events system: architecture and inner behaviour.\">"));
            assert!(html.contains("<meta property=\"og:title\" content=\"After Save Event\">"));
            assert!(html.contains("<meta property=\"og:type\" content=\"article\">"));
            assert!(html.contains("<meta property=\"og:url\" content=\"https://www.lucalorenzon.com/articles/after-save-event.html\">"));
            assert!(html.contains("<meta property=\"og:description\" content=\"SAP hybris change events system: architecture and inner behaviour.\">"));
            assert!(html.contains("<meta property=\"og:site_name\" content=\"Luca Lorenzon\">"));
            assert!(html.contains("<meta property=\"og:locale\" content=\"en_GB\">"));
            assert!(
                html.contains("<meta property=\"article:published_time\" content=\"2023-04-30\">")
            );
            assert!(!html.contains("og:image"));
        }

        #[test]
        fn has_the_canonical_link_with_the_full_address() {
            assert!(page().contains(
                "<link rel=\"canonical\" href=\"https://www.lucalorenzon.com/articles/after-save-event.html\""
            ));
        }

        #[test]
        fn canonical_link_follows_the_prefix_of_the_site_address() {
            let site = SiteAddress::parse("https://example.com/blog/").unwrap();
            let html = render_article_page(&real_articles()[0], &site);
            assert!(html.contains(
                "<link rel=\"canonical\" href=\"https://example.com/blog/articles/after-save-event.html\""
            ));
        }

        #[test]
        fn starts_with_doctype() {
            assert!(page().starts_with("<!DOCTYPE html>"));
        }

        #[test]
        fn declares_english_language() {
            assert!(page().contains("lang=\"en\""));
        }

        #[test]
        fn sets_tab_title() {
            assert!(page().contains("<title>After Save Event - Luca Lorenzon</title>"));
        }

        #[test]
        fn has_the_title_as_only_h1() {
            let html = page();
            assert_eq!(html.matches("<h1").count(), 1);
            assert!(html.contains(">After Save Event</h1>"));
        }

        #[test]
        fn shows_the_article_text_in_main() {
            let html = page();
            let main = &html[html.find("<main").expect("main")..html.find("</main>").expect("end")];
            assert!(main.contains("<h2>Definition of AfterSaveEvent</h2>"));
            assert!(main.contains("<table>"));
            assert!(!main.contains("<h1"));
        }

        #[test]
        fn ends_the_text_with_the_licence_note_as_a_footer_in_main() {
            let html = page();
            let main = &html[html.find("<main").expect("main")..html.find("</main>").expect("end")];
            assert!(main.contains("<footer>\n<p>"));
            assert!(!html.contains("<hr"));
        }

        #[test]
        fn footer_says_reuse_with_attribution_and_links_to_the_about_page() {
            let html = page();
            let footer = &html[html.find("</main>").expect("end")..];
            let footer = &footer[footer.find("<footer").expect("footer")..];
            assert!(footer.contains(">Reuse with attribution</a>"));
            assert!(footer.contains("<a href=\"../about.html\""));
        }

        #[test]
        fn logo_links_to_the_home_page_with_a_relative_address() {
            assert!(page().contains("<a href=\"../\" aria-label=\"Lvk@73r, home page\""));
        }

        #[test]
        fn links_favicon_and_stylesheet_with_relative_paths() {
            let html = page();
            assert!(html.contains("<link rel=\"icon\" href=\"../favicon.ico\""));
            assert!(html.contains("href=\"../style.css\""));
        }

        #[test]
        fn has_no_root_absolute_references() {
            let html = page();
            assert!(!html.contains("href=\"/"));
            assert!(!html.contains("src=\"/"));
        }

        #[test]
        fn links_to_the_other_article_with_a_relative_address() {
            assert!(
                page().contains("href=\"how-to-generate-hybris-pk-values-without-hybris.html\"")
            );
        }
    }

    mod hybris_pk {
        use super::super::*;

        fn page() -> String {
            render_article_page(
                &real_articles()[1],
                &SiteAddress::parse("https://www.lucalorenzon.com/").unwrap(),
            )
        }

        #[test]
        fn starts_with_doctype() {
            assert!(page().starts_with("<!DOCTYPE html>"));
        }

        #[test]
        fn declares_english_language() {
            assert!(page().contains("lang=\"en\""));
        }

        #[test]
        fn sets_tab_title() {
            assert!(page().contains(
                "<title>How to generate hybris PK values without hybris - Luca Lorenzon</title>"
            ));
        }

        #[test]
        fn has_the_title_as_only_h1() {
            let html = page();
            assert_eq!(html.matches("<h1").count(), 1);
            assert!(html.contains(">How to generate hybris PK values without hybris</h1>"));
        }

        #[test]
        fn shows_the_article_text_in_main() {
            let html = page();
            let main = &html[html.find("<main").expect("main")..html.find("</main>").expect("end")];
            assert!(main.contains("<h2>What is the type code</h2>"));
            assert!(main.contains("<table>"));
            assert!(!main.contains("<h1"));
        }

        #[test]
        fn ends_the_text_with_the_licence_note_as_a_footer_in_main() {
            let html = page();
            let main = &html[html.find("<main").expect("main")..html.find("</main>").expect("end")];
            assert!(main.contains("<footer>\n<p>"));
            assert!(!html.contains("<hr"));
        }

        #[test]
        fn footer_says_reuse_with_attribution_and_links_to_the_about_page() {
            let html = page();
            let footer = &html[html.find("</main>").expect("end")..];
            let footer = &footer[footer.find("<footer").expect("footer")..];
            assert!(footer.contains(">Reuse with attribution</a>"));
            assert!(footer.contains("<a href=\"../about.html\""));
        }

        #[test]
        fn logo_links_to_the_home_page_with_a_relative_address() {
            assert!(page().contains("<a href=\"../\" aria-label=\"Lvk@73r, home page\""));
        }

        #[test]
        fn links_favicon_and_stylesheet_with_relative_paths() {
            let html = page();
            assert!(html.contains("<link rel=\"icon\" href=\"../favicon.ico\""));
            assert!(html.contains("href=\"../style.css\""));
        }

        #[test]
        fn has_no_root_absolute_references() {
            let html = page();
            assert!(!html.contains("href=\"/"));
            assert!(!html.contains("src=\"/"));
        }
    }

    mod list {
        use super::super::*;

        #[test]
        fn formats_a_date_for_the_reader_and_for_the_machine() {
            let date = Date::try_from("2023-04-30").unwrap();
            assert_eq!(date.long_form(), "30 April 2023");
            assert_eq!(date.iso(), "2023-04-30");
            let date = Date::try_from("2024-12-05").unwrap();
            assert_eq!(date.long_form(), "5 December 2024");
            assert_eq!(date.iso(), "2024-12-05");
        }

        fn article(title: &str, date: &str) -> Article {
            Article {
                title: Title::try_from(title).unwrap(),
                description: Description::try_from("d").unwrap(),
                date: Date::try_from(date).unwrap(),
                address: "",
                source: "",
                body: String::new(),
            }
        }

        #[test]
        fn orders_by_date_whatever_the_order_of_the_list() {
            let oldest_first = [
                article("a", "2023-04-01"),
                article("c", "2023-04-30"),
                article("b", "2023-04-25"),
            ];
            let titles: Vec<&str> = newest_first(&oldest_first)
                .iter()
                .map(|a| a.title.as_str())
                .collect();
            assert_eq!(titles, ["c", "b", "a"]);
        }

        #[test]
        fn names_the_file_its_text_comes_from() {
            let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
            for file in &ARTICLE_FILES {
                let text = std::fs::read_to_string(root.join(file.source)).unwrap();
                assert_eq!(file.text, text, "{}", file.source);
            }
        }

        #[test]
        fn lists_the_site_articles_newest_first() {
            let articles = real_articles();
            let titles: Vec<&str> = newest_first(&articles)
                .iter()
                .map(|a| a.title.as_str())
                .collect();
            assert_eq!(
                titles,
                [
                    "After Save Event",
                    "How to generate hybris PK values without hybris"
                ]
            );
        }
    }

    mod title {
        use super::super::*;

        #[test]
        fn accepts_a_text_and_trims_it() {
            assert_eq!(Title::try_from("  Hello ").unwrap().as_str(), "Hello");
        }

        #[test]
        fn computes_the_tab_title() {
            assert_eq!(
                Title::try_from("Hello").unwrap().tab_title(),
                "Hello - Luca Lorenzon"
            );
        }

        #[test]
        fn rejects_an_empty_or_blank_text() {
            assert_eq!(Title::try_from(""), Err(TitleError::Empty));
            assert_eq!(Title::try_from(" \t "), Err(TitleError::Empty));
        }
    }

    mod description {
        use super::super::*;

        #[test]
        fn accepts_160_characters() {
            assert!(Description::try_from("a".repeat(160).as_str()).is_ok());
        }

        #[test]
        fn counts_characters_not_bytes() {
            assert!(Description::try_from("è".repeat(160).as_str()).is_ok());
        }

        #[test]
        fn rejects_161_characters() {
            assert_eq!(
                Description::try_from("a".repeat(161).as_str()),
                Err(DescriptionError::TooLong { characters: 161 })
            );
        }

        #[test]
        fn rejects_an_empty_or_blank_text() {
            assert_eq!(Description::try_from(""), Err(DescriptionError::Empty));
            assert_eq!(Description::try_from("  "), Err(DescriptionError::Empty));
        }
    }

    mod date {
        use super::super::*;

        #[test]
        fn accepts_real_days() {
            for text in ["2023-04-30", "2024-02-29", "2000-02-29", "2023-12-31"] {
                assert_eq!(Date::try_from(text).unwrap().iso(), text);
            }
        }

        #[test]
        fn rejects_days_that_do_not_exist() {
            for text in [
                "2023-02-30",
                "2023-02-29",
                "1900-02-29",
                "2023-04-31",
                "2023-13-01",
                "2023-00-10",
                "2023-01-00",
                "2023-01-32",
                "0000-01-01",
            ] {
                assert_eq!(
                    Date::try_from(text),
                    Err(DateError::NotACalendarDay(text.to_owned())),
                    "{text}"
                );
            }
        }

        #[test]
        fn rejects_text_that_is_not_a_date() {
            for text in [
                "",
                "tomorrow",
                "2023-4-3",
                "2023/04/30",
                "20230430",
                "2023-04-3x",
                "+023-04-30",
                "2023-04-300",
                "2023-04-é1",
            ] {
                assert_eq!(
                    Date::try_from(text),
                    Err(DateError::NotIsoFormat(text.to_owned())),
                    "{text}"
                );
            }
        }
    }

    mod construction {
        use super::super::*;

        const GOOD: &str = "---\ntitle: A title\ndescription: \"A: description.\"\ndate: 2023-04-30\n---\n\nBody *text*.\n";

        fn file(text: &'static str) -> ArticleFile {
            ArticleFile {
                address: "articles/a.html",
                source: "content/articles/a.md",
                text,
            }
        }

        fn error_of(text: &'static str) -> ArticleError {
            Article::try_from(&file(text)).unwrap_err()
        }

        #[test]
        fn builds_an_article_from_a_valid_file() {
            let article = Article::try_from(&file(GOOD)).unwrap();
            assert_eq!(article.title.as_str(), "A title");
            assert_eq!(article.description.as_str(), "A: description.");
            assert_eq!(article.date.iso(), "2023-04-30");
            assert_eq!(article.address, "articles/a.html");
            assert_eq!(article.body, "\nBody *text*.\n");
        }

        #[test]
        fn fields_may_come_in_any_order_with_blank_lines() {
            let text = "---\ntitle: Order does not matter\n\ndate: 2024-02-29\ndescription: Blank lines neither.\n\n\n---\n\nThe body.\n";
            let article = Article::try_from(&file(text)).unwrap();
            assert_eq!(article.title.as_str(), "Order does not matter");
            assert_eq!(article.description.as_str(), "Blank lines neither.");
            assert_eq!(article.date.iso(), "2024-02-29");
            assert_eq!(article.body, "\nThe body.\n");
        }

        #[test]
        fn error_points_at_the_line_of_the_file() {
            // The date is on line 6 of the file, counting the opening `---` as line 1.
            let text = "---\ntitle: T\n\ndescription: D\n\ndate: 2023-02-30\n---\nB\n";
            let message = error_of(text).to_string();
            assert!(message.contains("line 6 column 7"), "{message}");
        }

        #[test]
        fn accepts_windows_line_ends() {
            let text = "---\r\ntitle: T\r\ndescription: D\r\ndate: 2023-04-30\r\n---\r\nBody\r\n";
            assert!(Article::try_from(&file(text)).is_ok());
        }

        #[test]
        fn a_file_without_front_matter_is_an_error() {
            let error = error_of("Just a body.\n");
            assert_eq!(error.error, FrontMatterError::Missing);
            assert!(error.to_string().starts_with("content/articles/a.md:"));
        }

        #[test]
        fn a_front_matter_never_closed_is_an_error() {
            let error = error_of("---\ntitle: T\ndescription: D\ndate: 2023-04-30\nBody\n");
            assert_eq!(error.error, FrontMatterError::Unclosed);
        }

        #[test]
        fn malformed_yaml_is_an_error() {
            let error = error_of("---\ntitle: [unclosed\ndescription: D\n---\nBody\n");
            assert!(matches!(error.error, FrontMatterError::Invalid(_)));
            assert!(error.to_string().contains("content/articles/a.md"));
        }

        #[test]
        fn a_missing_field_is_an_error_naming_it() {
            let error = error_of("---\ntitle: T\ndate: 2023-04-30\n---\nBody\n");
            assert!(matches!(error.error, FrontMatterError::Invalid(_)));
            assert!(error.to_string().contains("description"), "{error}");
        }

        #[test]
        fn an_unknown_field_is_an_error_naming_it() {
            let error =
                error_of("---\ntitle: T\ndescription: D\ndate: 2023-04-30\ntittle: x\n---\nBody\n");
            assert!(matches!(error.error, FrontMatterError::Invalid(_)));
            assert!(error.to_string().contains("tittle"), "{error}");
        }

        #[test]
        fn a_wrong_title_names_the_file_the_field_and_the_reason() {
            let error = error_of("---\ntitle: \" \"\ndescription: D\ndate: 2023-04-30\n---\nB\n");
            assert!(matches!(error.error, FrontMatterError::Invalid(_)));
            let message = error.to_string();
            assert!(message.starts_with("content/articles/a.md: "), "{message}");
            assert!(message.contains("field `title` is empty"), "{message}");
        }

        #[test]
        fn a_wrong_description_names_the_reason() {
            let text = "---\ntitle: T\ndescription: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\ndate: 2023-04-30\n---\nB\n";
            let error = error_of(text);
            assert!(matches!(error.error, FrontMatterError::Invalid(_)));
            let message = error.to_string();
            assert!(
                message.contains("field `description` has 161 characters"),
                "{message}"
            );
            assert!(message.contains("160"), "{message}");
        }

        #[test]
        fn a_wrong_date_names_the_reason() {
            let error = error_of("---\ntitle: T\ndescription: D\ndate: 2023-02-30\n---\nB\n");
            assert!(matches!(error.error, FrontMatterError::Invalid(_)));
            let message = error.to_string();
            assert!(message.contains("field `date`: `2023-02-30`"), "{message}");
            assert!(message.contains("not a day of the calendar"), "{message}");
        }

        #[test]
        fn a_date_that_is_not_text_is_an_error() {
            let error = error_of("---\ntitle: T\ndescription: D\ndate: [1, 2]\n---\nB\n");
            assert!(matches!(error.error, FrontMatterError::Invalid(_)));
        }

        #[test]
        fn loading_separates_the_good_from_the_bad() {
            let files = [file(GOOD), file("no front matter"), file(GOOD)];
            let (articles, skipped) = load_articles(&files);
            assert_eq!(articles.len(), 2);
            assert_eq!(skipped.len(), 1);
        }

        fn loaded_with_two_problems() -> (Vec<Article>, Vec<ArticleError>) {
            load_articles(&[
                file(GOOD),
                file("no front matter"),
                file("---\nx: 1\n---\n"),
            ])
        }

        #[test]
        fn outside_the_pipeline_problems_are_warnings_and_a_summary() {
            let loaded = settle(loaded_with_two_problems(), false).unwrap();
            assert_eq!(loaded.articles.len(), 1);
            assert_eq!(loaded.warnings.len(), 2);
            assert!(
                loaded
                    .warnings
                    .iter()
                    .all(|w| w.starts_with("article skipped: "))
            );
            let summary = loaded.summary.unwrap();
            assert!(summary.starts_with("2 article(s) skipped"));
            assert_eq!(summary.matches("content/articles/a.md").count(), 2);
        }

        #[test]
        fn in_the_pipeline_all_the_problems_stop_the_build() {
            let error = settle(loaded_with_two_problems(), true).unwrap_err();
            assert!(error.starts_with("2 article(s) have problems:"));
            assert_eq!(error.matches("content/articles/a.md").count(), 2);
        }

        #[test]
        fn without_problems_nothing_is_said_in_either_case() {
            for is_ci in [true, false] {
                let loaded = settle(load_articles(&[file(GOOD)]), is_ci).unwrap();
                assert_eq!(loaded.articles.len(), 1);
                assert!(loaded.warnings.is_empty());
                assert!(loaded.summary.is_none());
            }
        }

        #[test]
        fn every_real_article_builds() {
            let (articles, skipped) = load_articles(&ARTICLE_FILES);
            assert!(skipped.is_empty(), "{skipped:?}");
            assert_eq!(articles.len(), ARTICLE_FILES.len());
        }

        #[test]
        fn the_body_of_every_real_article_has_no_raw_html_and_no_front_matter() {
            use pulldown_cmark::{Event, Parser};
            for article in real_articles() {
                assert!(!article.body.starts_with("---"), "{}", article.source);
                let raw: Vec<Event> = Parser::new(&article.body)
                    .filter(|e| matches!(e, Event::Html(_) | Event::InlineHtml(_)))
                    .collect();
                assert!(raw.is_empty(), "{}: {raw:?}", article.source);
            }
        }
    }
}
