//! The text page of the style guide: the text of `content/style-guide/text.md`, whose headings,
//! paragraphs and lists are the cases the browser tests of the text run on. The page is the
//! Markdown converted as every article is, so the elements are the real ones.

use super::FOLDER;
use super::navigation::GuideMain;
use crate::markdown;
use crate::page_frame::{FooterLicence, Heading, IndexedPage, Indexing, PageFrame, PageKind};
use crate::site_address::SiteAddress;
use leptos::prelude::*;

/// The page's file in the guide's folder, as the index links to it.
pub const FILE: &str = "text.html";

/// The text of the page, written in Markdown: an introduction, then one section for each case
/// the text is put to the test with. The first paragraph after each heading, and the
/// introduction, explain the case to the reader; the rest is the case, and the browser tests'
/// guard (`expectEveryCaseWraps` in `support.ts`) relies on that.
pub(crate) const TEXT: &str = include_str!("../../content/style-guide/text.md");

/// What search engines and link previews show of the page.
const DESCRIPTION: &str = "The text of this site: headings of three levels, paragraphs, bulleted and numbered lists, and the elements inside a line.";

#[component]
pub fn TextPage(canonical: String, body: String) -> impl IntoView {
    view! {
        <PageFrame
            title="Text - Style guide - Luca Lorenzon"
            heading=Heading::try_from("Text").expect("the text page's heading fits the frame")
            address_prefix="../"
            footer_licence=FooterLicence::ReuseWithAttribution
            indexing=Indexing::Indexed(IndexedPage {
                canonical,
                description: DESCRIPTION.to_owned(),
                kind: PageKind::Website,
            })
        >
            <GuideMain title="Text" body=body />
        </PageFrame>
    }
}

/// Renders the text page to a complete HTML document string.
pub fn render_text_page(site: &SiteAddress) -> String {
    let body = markdown::to_html(TEXT);
    let owner = Owner::new();
    let canonical = site.join(&format!("{FOLDER}{FILE}"));
    let page = owner.with(|| view! { <TextPage canonical=canonical body=body /> }.to_html());
    format!("<!DOCTYPE html>{page}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn site() -> SiteAddress {
        SiteAddress::parse("https://www.lucalorenzon.com/").unwrap()
    }

    fn render_text_page() -> String {
        super::render_text_page(&site())
    }

    /// The starts of the titles of the four sections: each title is a case of its own, and says what it puts to the
    /// test.
    const SECTION_STARTS: [&str; 4] = [
        "A heading of level two, ",
        "Two paragraphs, one after the other, ",
        "A bulleted list and a numbered list, ",
        "Bold, italic, inline code and a link ",
    ];

    #[test]
    fn the_page_holds_the_cases_of_the_text() {
        let html = markdown::to_html(TEXT);
        // Headings of the three levels the text checks look at.
        assert_eq!(html.matches("<h2>").count(), 4);
        assert_eq!(html.matches("<h3>").count(), 1);
        assert_eq!(html.matches("<h4>").count(), 1);
        for start in &SECTION_STARTS[..] {
            assert!(html.contains(&format!("<h2>{start}")), "{start}");
        }
        // Two consecutive paragraphs, after the section's own explanation.
        assert!(html.contains("</p>\n<p>This is the second paragraph"));
        // A bulleted and a numbered list, of items that are not one line.
        assert_eq!(html.matches("<ul>").count(), 1);
        assert_eq!(html.matches("<ol>").count(), 1);
        assert_eq!(html.matches("<li>").count(), 6);
        // The elements inside a line, which the page says are shown only.
        for tag in ["<strong>", "<em>", "<code>", "<a href=\"../about.html\">"] {
            assert!(html.contains(tag), "{tag}");
        }
        assert!(html.contains("shown and not tested"));
    }

    #[test]
    fn the_page_holds_no_code_block_and_no_table() {
        let html = render_text_page();
        for part in [
            "code-block",
            "<pre",
            "<table",
            "table-with-viewer",
            "record-viewer",
        ] {
            assert!(!html.contains(part), "{part}");
        }
    }

    #[test]
    fn its_heading_fits_the_frame() {
        assert!(render_text_page().contains("<h1 "));
    }

    #[test]
    fn has_its_description_and_preview_tags() {
        let html = render_text_page();
        let url = "https://www.lucalorenzon.com/style-guide/text.html";
        assert!(html.contains(&format!(
            "<meta name=\"description\" content=\"{DESCRIPTION}\">"
        )));
        assert!(html.contains("<meta property=\"og:title\" content=\"Text\">"));
        assert!(html.contains("<meta property=\"og:type\" content=\"website\">"));
        assert!(html.contains(&format!("<meta property=\"og:url\" content=\"{url}\">")));
        assert!(html.contains(&format!(
            "<meta property=\"og:description\" content=\"{DESCRIPTION}\">"
        )));
        assert!(html.contains("<meta property=\"og:site_name\" content=\"Luca Lorenzon\">"));
        assert!(!html.contains("og:image"));
    }

    #[test]
    fn description_fits_a_search_result() {
        assert!(DESCRIPTION.chars().count() <= 160);
    }

    #[test]
    fn has_its_own_full_address_as_canonical_link() {
        assert!(render_text_page().contains(
            "<link rel=\"canonical\" href=\"https://www.lucalorenzon.com/style-guide/text.html\">"
        ));
    }

    #[test]
    fn sets_tab_title_and_has_text_as_only_h1() {
        let html = render_text_page();
        assert!(html.contains("<title>Text - Style guide - Luca Lorenzon</title>"));
        assert_eq!(html.matches("<h1").count(), 1);
        assert!(html.contains(">Text</h1>"));
    }

    #[test]
    fn addresses_are_relative_to_the_guides_folder() {
        let html = render_text_page();
        assert!(html.contains("href=\"../style.css\""));
        assert!(html.contains("<link rel=\"icon\" href=\"../favicon.ico\""));
        assert!(html.contains("<a href=\"../about.html\""));
        assert!(html.contains("<a href=\"../\" aria-label=\"Lvk@73r, home page\""));
        assert!(!html.contains("href=\"/"));
        assert!(!html.contains("src=\"/"));
    }

    #[test]
    fn footer_says_reuse_with_attribution() {
        let html = render_text_page();
        let footer = &html[html.find("<footer").unwrap()..];
        assert!(footer.contains(">Reuse with attribution</a>"));
    }
}
