use crate::markdown;
use crate::page_frame::{
    FooterLicence, Heading, HeadingError, IndexedPage, Indexing, PageFrame, PageKind, SITE_NAME,
};
use crate::site_address::SiteAddress;
use leptos::prelude::*;
use serde::Deserialize;

/// The title of an article: the content's own name for it. Not empty. It has no length limit:
/// the limit belongs to the page's heading ([`Heading`]), which an article builds from its
/// title when it is read.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct Title(String);

/// Why a text is not a [`Title`].
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum TitleError {
    #[error("field `title` is empty")]
    Empty,
}

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
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum DescriptionError {
    #[error("field `description` is empty")]
    Empty,
    #[error(
        "field `description` has {characters} characters, more than the {max} a search result shows",
        max = DESCRIPTION_MAX_CHARACTERS
    )]
    TooLong { characters: usize },
}

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
    date: jiff::civil::Date,
}

/// What is wrong in an article's front matter.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum FrontMatterError {
    /// The file does not start with a line `---`.
    #[error("the file does not start with a front matter (a line `---`)")]
    Missing,
    /// The front matter has no closing line `---`.
    #[error("the front matter has no closing line `---`")]
    Unclosed,
    /// The YAML reader's complaint: malformed YAML, a field missing, an unknown field, a field
    /// of the wrong kind, or a field whose value is not valid (`Title`, `Description`, `Date`).
    /// Where it is and what it is are kept apart; [`ArticleError`] puts them together with the
    /// file.
    #[error("front matter: {message}")]
    Invalid {
        /// Where the reader found it, in the file's lines and columns; `None` when it does not
        /// know.
        position: Option<Position>,
        /// What is wrong, as the reader says it: one line, with no position and no `error:`.
        message: String,
    },
    /// The title is valid as content but does not fit the frame's heading (too long).
    #[error("front matter: field `title`: {0}")]
    Heading(HeadingError),
}

/// A place in a file: line and column, both counted from 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub line: u64,
    pub column: u64,
}

impl FrontMatterError {
    /// Takes the reader's error apart: its position, if it knows one, and its bare message.
    fn invalid(problem: &serde_saphyr::Error) -> Self {
        use serde_saphyr::MessageFormatter as _;
        let position = problem
            .location()
            .filter(|location| *location != serde_saphyr::Location::UNKNOWN)
            .map(|location| Position {
                line: location.line(),
                column: location.column(),
            });
        // The formatter gives the message alone, as `Display` does, without the location the
        // renderer adds after it and without the `error:` it puts before. It looks through the
        // reader's wrappers (snippet, alias) itself.
        let message = serde_saphyr::DefaultMessageFormatter.format_message(problem);
        Self::Invalid {
            position,
            message: message.lines().next().unwrap_or_default().to_owned(),
        }
    }
}

/// Why an article's file cannot become an [`Article`]: which file, and what is wrong in its
/// front matter.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{}", self.text())]
pub struct ArticleError {
    pub file: &'static str,
    pub error: FrontMatterError,
}

impl ArticleError {
    /// The message: where, then what. A reader's position follows the file the way compilers
    /// put it (`file:line:column: message`), which terminals and editors turn into a link.
    fn text(&self) -> String {
        match &self.error {
            FrontMatterError::Invalid {
                position: Some(Position { line, column }),
                ..
            } => format!("{}:{line}:{column}: {}", self.file, self.error),
            other => format!("{}: {other}", self.file),
        }
    }
}

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
    /// What the frame shows as the page's heading, built from the title when the article is
    /// read: a title that does not fit is a problem of the article, found with the others.
    pub heading: Heading,
    pub description: Description,
    pub date: jiff::civil::Date,
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
            file: file.source,
            error,
        };
        let (front_matter, body) = split_front_matter(file.text).map_err(error)?;
        let fields: FrontMatter = serde_saphyr::from_str(front_matter)
            .map_err(|problem| error(FrontMatterError::invalid(&problem)))?;
        let heading = Heading::try_from(fields.title.as_str())
            .map_err(|problem| error(FrontMatterError::Heading(problem)))?;
        Ok(Self {
            title: fields.title,
            heading,
            description: fields.description,
            date: fields.date,
            address: file.address,
            source: file.source,
            body: body.to_owned(),
        })
    }
}

/// The articles that could not be read, each with why, in the order of the files.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("{} article(s) have problems:\n{}", .0.len(), self.list())]
pub struct ArticleProblems(pub Vec<ArticleError>);

impl ArticleProblems {
    /// One line per article, "  - {problem}", in the order of the files.
    pub fn list(&self) -> String {
        self.0
            .iter()
            .map(|error| format!("  - {error}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// The articles that could be read, and the problems of the others.
#[derive(Debug)]
pub struct LoadedArticles {
    pub articles: Vec<Article>,
    /// `None` when every file is an article.
    pub problems: Option<ArticleProblems>,
}

/// Builds every article it can. The others are returned with why, in the order of the files;
/// what to do about them is the caller's.
pub fn load_articles(files: &[ArticleFile]) -> LoadedArticles {
    let mut articles = Vec::new();
    let mut skipped = Vec::new();
    for file in files {
        match Article::try_from(file) {
            Ok(article) => articles.push(article),
            Err(error) => skipped.push(error),
        }
    }
    LoadedArticles {
        articles,
        problems: (!skipped.is_empty()).then_some(ArticleProblems(skipped)),
    }
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
            heading=article.heading.clone()
            address_prefix="../"
            footer_licence=FooterLicence::ReuseWithAttribution
            indexing=Indexing::Indexed(IndexedPage {
                canonical,
                description: article.description.as_str().to_owned(),
                kind: PageKind::Article { published: article.date.to_string() },
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
    let loaded = load_articles(&ARTICLE_FILES);
    assert!(loaded.problems.is_none(), "{:?}", loaded.problems);
    loaded.articles
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

        fn article(title: &str, date: &str) -> Article {
            Article {
                title: Title::try_from(title).unwrap(),
                heading: Heading::try_from(title).unwrap(),
                description: Description::try_from("d").unwrap(),
                date: date.parse().unwrap(),
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

        fn file_with_date(date: &str) -> String {
            format!("---\ntitle: T\ndescription: D\ndate: {date}\n---\nB\n")
        }

        fn date_of(date: &str) -> Result<String, ArticleError> {
            let text: &'static str = Box::leak(file_with_date(date).into_boxed_str());
            let file = ArticleFile {
                address: "articles/a.html",
                source: "content/articles/a.md",
                text,
            };
            Article::try_from(&file).map(|article| article.date.to_string())
        }

        #[test]
        fn accepts_real_days() {
            for text in ["2023-04-30", "2024-02-29", "2000-02-29", "2023-12-31"] {
                assert_eq!(date_of(text).as_deref(), Ok(text), "{text}");
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
            ] {
                assert!(date_of(text).is_err(), "{text}");
            }
        }

        #[test]
        fn rejects_text_that_is_not_a_date() {
            for text in [
                "tomorrow",
                "2023-4-3",
                "26-11-06",
                "2023/04/30",
                "2023-04-3x",
                "+023-04-30",
                "2023-04-300",
                "2023-04-é1",
                "\"\"",
            ] {
                assert!(date_of(text).is_err(), "{text}");
            }
        }

        #[test]
        fn accepts_the_forms_the_user_chose_to_take_from_the_library() {
            // The library's ISO reading also takes these two; the user accepted that on
            // purpose (card "give dates a module of their own").
            assert_eq!(date_of("20230430").as_deref(), Ok("2023-04-30"));
            assert_eq!(date_of("0000-01-01").as_deref(), Ok("0000-01-01"));
        }
    }

    mod construction {
        use super::super::*;
        use crate::page_frame::heading_max_graphemes;

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
            assert_eq!(article.date.to_string(), "2023-04-30");
            assert_eq!(article.address, "articles/a.html");
            assert_eq!(article.body, "\nBody *text*.\n");
        }

        #[test]
        fn fields_may_come_in_any_order_with_blank_lines() {
            let text = "---\ntitle: Order does not matter\n\ndate: 2024-02-29\ndescription: Blank lines neither.\n\n\n---\n\nThe body.\n";
            let article = Article::try_from(&file(text)).unwrap();
            assert_eq!(article.title.as_str(), "Order does not matter");
            assert_eq!(article.description.as_str(), "Blank lines neither.");
            assert_eq!(article.date.to_string(), "2024-02-29");
            assert_eq!(article.body, "\nThe body.\n");
        }

        #[test]
        fn error_points_at_the_line_of_the_file() {
            // The date is on line 6 of the file, counting the opening `---` as line 1.
            let text = "---\ntitle: T\n\ndescription: D\n\ndate: 2023-02-30\n---\nB\n";
            let error = error_of(text);
            assert!(
                matches!(
                    error.error,
                    FrontMatterError::Invalid {
                        position: Some(Position { line: 6, column: 7 }),
                        ..
                    }
                ),
                "{error}"
            );
            assert!(
                error.to_string().starts_with("content/articles/a.md:6:7: "),
                "{error}"
            );
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
            assert!(matches!(error.error, FrontMatterError::Invalid { .. }));
            assert!(error.to_string().contains("content/articles/a.md"));
        }

        #[test]
        fn a_missing_field_is_an_error_naming_it() {
            let error = error_of("---\ntitle: T\ndate: 2023-04-30\n---\nBody\n");
            assert!(matches!(error.error, FrontMatterError::Invalid { .. }));
            assert!(error.to_string().contains("description"), "{error}");
        }

        fn invalid_of(text: &'static str) -> (Option<Position>, String) {
            match error_of(text).error {
                FrontMatterError::Invalid { position, message } => (position, message),
                other => panic!("not an Invalid: {other:?}"),
            }
        }

        #[test]
        fn a_missing_field_has_its_position_in_the_files_lines() {
            // The opening `---` is line 1, so `date` is line 3: the reader counts the file's
            // lines, not the front matter's.
            let (position, message) = invalid_of("---\ntitle: T\ndate: 2023-04-30\n---\nBody\n");
            assert_eq!(position, Some(Position { line: 3, column: 1 }));
            assert_eq!(message, "missing field `description`");
        }

        #[test]
        fn the_message_has_no_error_prefix_and_no_position() {
            let (_, message) = invalid_of("---\ntitle: T\ndate: 2023-04-30\n---\nBody\n");
            assert!(!message.contains("error:"), "{message}");
            assert!(!message.contains("line 3"), "{message}");
            let text = error_of("---\ntitle: T\ndate: 2023-04-30\n---\nBody\n").to_string();
            assert!(!text.contains("error:"), "{text}");
        }

        #[test]
        fn the_text_puts_the_position_after_the_file() {
            let error = ArticleError {
                file: "content/articles/a.md",
                error: FrontMatterError::Invalid {
                    position: Some(Position { line: 3, column: 1 }),
                    message: "missing field `description`".to_owned(),
                },
            };
            assert_eq!(
                error.to_string(),
                "content/articles/a.md:3:1: front matter: missing field `description`"
            );
        }

        #[test]
        fn the_text_without_a_position_has_only_the_file() {
            let error = ArticleError {
                file: "content/articles/a.md",
                error: FrontMatterError::Invalid {
                    position: None,
                    message: "something".to_owned(),
                },
            };
            assert_eq!(
                error.to_string(),
                "content/articles/a.md: front matter: something"
            );
        }

        #[test]
        fn an_unknown_field_is_an_error_naming_it() {
            let error =
                error_of("---\ntitle: T\ndescription: D\ndate: 2023-04-30\ntittle: x\n---\nBody\n");
            assert!(matches!(error.error, FrontMatterError::Invalid { .. }));
            assert!(error.to_string().contains("tittle"), "{error}");
        }

        #[test]
        fn a_wrong_title_names_the_file_the_field_and_the_reason() {
            let error = error_of("---\ntitle: \" \"\ndescription: D\ndate: 2023-04-30\n---\nB\n");
            assert!(matches!(error.error, FrontMatterError::Invalid { .. }));
            let message = error.to_string();
            assert!(
                message.starts_with("content/articles/a.md:2:8: front matter: "),
                "{message}"
            );
            assert!(message.contains("field `title` is empty"), "{message}");
        }

        #[test]
        fn a_title_too_long_for_the_heading_names_the_file_the_field_and_the_count() {
            let max = heading_max_graphemes();
            let title = "a".repeat(max + 7);
            let text: &'static str = Box::leak(
                format!("---\ntitle: {title}\ndescription: D\ndate: 2023-04-30\n---\nB\n")
                    .into_boxed_str(),
            );
            let error = error_of(text);
            assert!(matches!(error.error, FrontMatterError::Heading(_)));
            assert_eq!(
                error.to_string(),
                format!(
                    "content/articles/a.md: front matter: field `title`: the title has {} \
                     characters, at most {max} fit in the page heading",
                    max + 7
                )
            );
        }

        #[test]
        fn a_title_too_long_is_one_more_problem_of_the_articles() {
            let max = heading_max_graphemes();
            let text: &'static str = Box::leak(
                format!(
                    "---\ntitle: {}\ndescription: D\ndate: 2023-04-30\n---\nB\n",
                    "a".repeat(max + 1)
                )
                .into_boxed_str(),
            );
            let loaded = load_articles(&[file(GOOD), file(text), file("no front matter")]);
            assert_eq!(loaded.articles.len(), 1);
            let problems = loaded.problems.unwrap().0;
            assert_eq!(problems.len(), 2);
            assert!(problems[0].to_string().contains("fit in the page heading"));
        }

        #[test]
        fn a_title_at_the_limit_builds_and_is_the_heading() {
            let title = "a".repeat(heading_max_graphemes());
            let text: &'static str = Box::leak(
                format!("---\ntitle: {title}\ndescription: D\ndate: 2023-04-30\n---\nB\n")
                    .into_boxed_str(),
            );
            let article = Article::try_from(&file(text)).unwrap();
            assert_eq!(article.heading.as_str(), title);
        }

        #[test]
        fn a_wrong_description_names_the_reason() {
            let text = "---\ntitle: T\ndescription: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\ndate: 2023-04-30\n---\nB\n";
            let error = error_of(text);
            assert!(matches!(error.error, FrontMatterError::Invalid { .. }));
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
            assert!(matches!(error.error, FrontMatterError::Invalid { .. }));
            let message = error.to_string();
            // The reason is the date library's own message.
            assert!(message.contains("parameter 'day'"), "{message}");
            assert!(message.contains("must be in range `1..=28`"), "{message}");
        }

        #[test]
        fn a_date_that_is_not_text_is_an_error() {
            let error = error_of("---\ntitle: T\ndescription: D\ndate: [1, 2]\n---\nB\n");
            assert!(matches!(error.error, FrontMatterError::Invalid { .. }));
        }

        #[test]
        fn loading_separates_the_good_from_the_bad() {
            let files = [file(GOOD), file("no front matter"), file(GOOD)];
            let loaded = load_articles(&files);
            assert_eq!(loaded.articles.len(), 2);
            assert_eq!(loaded.problems.unwrap().0.len(), 1);
        }

        #[test]
        fn the_problems_come_back_in_the_order_of_the_files() {
            let loaded = load_articles(&[
                file(GOOD),
                file("no front matter"),
                file("---\nx: 1\n---\n"),
            ]);
            assert_eq!(loaded.articles.len(), 1);
            let problems = loaded.problems.unwrap().0;
            assert_eq!(problems.len(), 2);
            assert_eq!(problems[0], error_of("no front matter"));
            assert_eq!(problems[1], error_of("---\nx: 1\n---\n"));
        }

        #[test]
        fn without_problems_there_are_none() {
            let loaded = load_articles(&[file(GOOD)]);
            assert_eq!(loaded.articles.len(), 1);
            assert!(loaded.problems.is_none());
        }

        #[test]
        fn the_problems_read_as_a_count_and_a_list() {
            let problems = load_articles(&[file("no front matter"), file("---\nx: 1\n---\n")])
                .problems
                .unwrap();
            let lines: Vec<String> = problems.0.iter().map(|e| format!("  - {e}")).collect();
            assert_eq!(
                problems.to_string(),
                format!("2 article(s) have problems:\n{}", lines.join("\n"))
            );
            assert!(
                problems
                    .to_string()
                    .starts_with("2 article(s) have problems:\n  - content/articles/a.md: ")
            );
            assert!(!problems.to_string().ends_with('\n'));
        }

        #[test]
        fn every_real_article_builds() {
            let loaded = load_articles(&ARTICLE_FILES);
            assert!(loaded.problems.is_none(), "{:?}", loaded.problems);
            assert_eq!(loaded.articles.len(), ARTICLE_FILES.len());
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
