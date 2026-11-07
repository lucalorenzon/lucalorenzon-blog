use crate::markdown;
use crate::page_frame::{FooterLicence, IndexedPage, Indexing, PageFrame, PageKind};
use crate::site_address::SiteAddress;
use leptos::prelude::*;

/// A day, written in the code with the article it dates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
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

/// What the site knows of an article: written here, in one list, for the article pages and the
/// home page's list alike. The model of an "article" with fields of its own, read from the
/// Markdown, is left for later (card docs/done/2026-11-06_list-articles-on-home-page.md).
pub struct Article {
    pub title: &'static str,
    pub tab_title: &'static str,
    /// Where the page is written, relative to the site's root.
    pub address: &'static str,
    pub description: &'static str,
    pub date: Date,
    /// The Markdown file `text` is read from, relative to the repository's root (the sitemap's
    /// dates are looked up by it; a test checks it names the file `text` comes from).
    pub source: &'static str,
    text: &'static str,
}

/// Every article of the site. A new article is added here, and nowhere else.
pub const ARTICLES: [Article; 2] = [
    Article {
        title: "After Save Event",
        tab_title: "After Save Event - Luca Lorenzon",
        address: "articles/after-save-event.html",
        description: "SAP hybris change events system: architecture and inner behaviour.",
        date: Date {
            year: 2023,
            month: 4,
            day: 30,
        },
        source: "content/articles/after-save-event.md",
        text: include_str!("../content/articles/after-save-event.md"),
    },
    Article {
        title: "How to generate hybris PK values without hybris",
        tab_title: "How to generate hybris PK values without hybris - Luca Lorenzon",
        address: "articles/how-to-generate-hybris-pk-values-without-hybris.html",
        description: "Dive into the PK generation logic of SAP hybris, for extreme use cases.",
        date: Date {
            year: 2023,
            month: 4,
            day: 25,
        },
        source: "content/articles/how-to-generate-hybris-pk-values-without-hybris.md",
        text: include_str!(
            "../content/articles/how-to-generate-hybris-pk-values-without-hybris.md"
        ),
    },
];

/// The articles, newest first.
pub fn newest_first() -> Vec<&'static Article> {
    newest_first_of(&ARTICLES)
}

fn newest_first_of(articles: &[Article]) -> Vec<&Article> {
    let mut sorted: Vec<&Article> = articles.iter().collect();
    sorted.sort_by_key(|article| std::cmp::Reverse(article.date));
    sorted
}

/// A whole article page document, `<html>` element included. The page lives in the `articles/`
/// folder, so every address is relative to that folder (ADR-002). The Markdown has no title:
/// the title is the page heading.
#[component]
pub fn ArticlePage(article: &'static Article, canonical: String) -> impl IntoView {
    view! {
        <PageFrame
            title=article.tab_title
            heading=article.title
            address_prefix="../"
            footer_licence=FooterLicence::ReuseWithAttribution
            indexing=Indexing::Indexed(IndexedPage {
                canonical,
                description: article.description,
                kind: PageKind::Article { published: article.date.iso() },
            })
        >
            <main class="long-text-column md:long-text-large" inner_html=markdown::to_html(article.text)></main>
        </PageFrame>
    }
}

/// Renders an article page to a complete HTML document string. Its canonical link is the
/// site's full address followed by the article's address.
pub fn render_article_page(article: &'static Article, site: &SiteAddress) -> String {
    let owner = Owner::new();
    let canonical = site.join(article.address);
    let body =
        owner.with(|| view! { <ArticlePage article=article canonical=canonical /> }.to_html());
    format!("<!DOCTYPE html>{body}")
}

#[cfg(test)]
mod tests {

    mod after_save_event {
        use super::super::*;

        fn page() -> String {
            render_article_page(&ARTICLES[0], &site())
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
        fn every_article_description_fits_a_search_result() {
            for article in &ARTICLES {
                assert!(article.description.chars().count() <= 160);
            }
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
            let html = render_article_page(&ARTICLES[0], &site);
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
                &ARTICLES[1],
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
            let date = Date {
                year: 2023,
                month: 4,
                day: 30,
            };
            assert_eq!(date.long_form(), "30 April 2023");
            assert_eq!(date.iso(), "2023-04-30");
            let date = Date {
                year: 2024,
                month: 12,
                day: 5,
            };
            assert_eq!(date.long_form(), "5 December 2024");
            assert_eq!(date.iso(), "2024-12-05");
        }

        #[test]
        fn orders_by_date_whatever_the_order_of_the_list() {
            let article = |title, day| Article {
                title,
                tab_title: "",
                address: "",
                description: "",
                date: Date {
                    year: 2023,
                    month: 4,
                    day,
                },
                source: "",
                text: "",
            };
            let oldest_first = [article("a", 1), article("c", 30), article("b", 25)];
            let titles: Vec<&str> = newest_first_of(&oldest_first)
                .iter()
                .map(|a| a.title)
                .collect();
            assert_eq!(titles, ["c", "b", "a"]);
        }

        #[test]
        fn names_the_file_its_text_comes_from() {
            let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
            for article in &ARTICLES {
                let text = std::fs::read_to_string(root.join(article.source)).unwrap();
                assert_eq!(article.text, text, "{}", article.source);
            }
        }

        #[test]
        fn lists_the_site_articles_newest_first() {
            let titles: Vec<&str> = newest_first().iter().map(|a| a.title).collect();
            assert_eq!(
                titles,
                [
                    "After Save Event",
                    "How to generate hybris PK values without hybris"
                ]
            );
        }
    }
}
