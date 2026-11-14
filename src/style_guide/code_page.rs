//! The code blocks page of the style guide: the text of `content/style-guide/code.md`, whose
//! fenced blocks are the cases the browser tests of the component run on. The page is the
//! Markdown converted as every article is, so the blocks are the real component.

use super::FOLDER;
use super::navigation::GuideMain;
use crate::markdown;
use crate::page_frame::{FooterLicence, Heading, IndexedPage, Indexing, PageFrame, PageKind};
use crate::site_address::SiteAddress;
use leptos::prelude::*;

/// The page's file in the guide's folder, as the index links to it.
pub const FILE: &str = "code.html";

/// The text of the page, written in Markdown: an introduction, then one section for each case
/// the component is put to the test with.
pub(crate) const CODE: &str = include_str!("../../content/style-guide/code.md");

/// What search engines and link previews show of the page.
const DESCRIPTION: &str = "The code blocks of this site: how a long line, a short block, indentation, tabs and a blank line are shown, and the wrap toggle.";

#[component]
pub fn CodePage(canonical: String, body: String) -> impl IntoView {
    view! {
        <PageFrame
            title="Code blocks - Style guide - Luca Lorenzon"
            heading=Heading::try_from("Code blocks").expect("the code page's heading fits the frame")
            address_prefix="../"
            footer_licence=FooterLicence::ReuseWithAttribution
            indexing=Indexing::Indexed(IndexedPage {
                canonical,
                description: DESCRIPTION.to_owned(),
                kind: PageKind::Website,
            })
        >
            <GuideMain title="Code blocks" body=body />
        </PageFrame>
    }
}

/// Renders the code blocks page to a complete HTML document string.
pub fn render_code_page(site: &SiteAddress) -> String {
    let body = markdown::to_html(CODE);
    let owner = Owner::new();
    let canonical = site.join(&format!("{FOLDER}{FILE}"));
    let page = owner.with(|| view! { <CodePage canonical=canonical body=body /> }.to_html());
    format!("<!DOCTYPE html>{page}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn site() -> SiteAddress {
        SiteAddress::parse("https://www.lucalorenzon.com/").unwrap()
    }

    fn render_code_page() -> String {
        super::render_code_page(&site())
    }

    #[test]
    fn the_cases_are_the_real_component() {
        let html = markdown::to_html(CODE);
        assert!(html.matches("<h2>").count() >= 5);
        assert!(html.matches("class=\"code-block").count() >= 5);
        assert!(render_code_page().contains("class=\"code-scroller\""));
    }

    #[test]
    fn every_case_has_a_language_and_the_tab_case_has_real_tabs() {
        let fences = CODE.lines().filter(|l| l.starts_with("```")).count();
        let closing = CODE.lines().filter(|l| *l == "```").count();
        assert_eq!(fences, 2 * closing, "every fence names a language");
        assert!(CODE.contains("\n\t"));
    }

    #[test]
    fn its_heading_fits_the_frame() {
        assert!(render_code_page().contains("<h1 "));
    }

    #[test]
    fn has_its_description_and_preview_tags() {
        let html = render_code_page();
        let url = "https://www.lucalorenzon.com/style-guide/code.html";
        assert!(html.contains(&format!(
            "<meta name=\"description\" content=\"{DESCRIPTION}\">"
        )));
        assert!(html.contains("<meta property=\"og:title\" content=\"Code blocks\">"));
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
        assert!(render_code_page().contains(
            "<link rel=\"canonical\" href=\"https://www.lucalorenzon.com/style-guide/code.html\">"
        ));
    }

    #[test]
    fn sets_tab_title_and_has_code_blocks_as_only_h1() {
        let html = render_code_page();
        assert!(html.contains("<title>Code blocks - Style guide - Luca Lorenzon</title>"));
        assert_eq!(html.matches("<h1").count(), 1);
        assert!(html.contains(">Code blocks</h1>"));
    }

    #[test]
    fn addresses_are_relative_to_the_guides_folder() {
        let html = render_code_page();
        assert!(html.contains("href=\"../style.css\""));
        assert!(html.contains("<link rel=\"icon\" href=\"../favicon.ico\""));
        assert!(html.contains("<a href=\"../about.html\""));
        assert!(html.contains("<a href=\"../\" aria-label=\"Lvk@73r, home page\""));
        assert!(!html.contains("href=\"/"));
        assert!(!html.contains("src=\"/"));
    }

    #[test]
    fn footer_says_reuse_with_attribution() {
        let html = render_code_page();
        let footer = &html[html.find("<footer").unwrap()..];
        assert!(footer.contains(">Reuse with attribution</a>"));
    }
}
