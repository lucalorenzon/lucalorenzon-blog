//! The page frame's page of the style guide: the text of `src/page_frame.md`, which is also the
//! documentation of the frame's module (ADR-005). The page is the Markdown converted as every
//! article is.

use super::FOLDER;
use super::navigation::GuideBody;
use crate::markdown;
use crate::page_frame::{FooterLicence, Heading, IndexedPage, Indexing, PageFrame, PageKind};
use crate::site_address::SiteAddress;
use leptos::prelude::*;

/// The page's file in the guide's folder, as the index links to it.
pub const FILE: &str = "frame.html";

/// The text of the page, written in Markdown beside the frame's code, which it also documents.
pub(crate) const FRAME: &str = crate::page_frame::EXPLANATION;

/// What search engines and link previews show of the page.
const DESCRIPTION: &str = "The frame every page of this site shares: the header and its change at 1280 pixels, the text that grows with the page, and the column.";

#[component]
pub fn FramePage(canonical: String, body: String) -> impl IntoView {
    view! {
        <PageFrame
            title="Page frame - Style guide - Luca Lorenzon"
            heading=Heading::try_from("Page frame").expect("the frame page's heading fits the frame")
            address_prefix="../"
            footer_licence=FooterLicence::ReuseWithAttribution
            indexing=Indexing::Indexed(IndexedPage {
                canonical,
                description: DESCRIPTION.to_owned(),
                kind: PageKind::Website,
            })
        >
            <GuideBody title="Page frame" body=body />
        </PageFrame>
    }
}

/// Renders the page frame's page to a complete HTML document string.
pub fn render_frame_page(site: &SiteAddress) -> String {
    let body = markdown::to_html(FRAME);
    let owner = Owner::new();
    let canonical = site.join(&format!("{FOLDER}{FILE}"));
    let page = owner.with(|| view! { <FramePage canonical=canonical body=body /> }.to_html());
    format!("<!DOCTYPE html>{page}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn site() -> SiteAddress {
        SiteAddress::parse("https://www.lucalorenzon.com/").unwrap()
    }

    fn render_frame_page() -> String {
        super::render_frame_page(&site())
    }

    #[test]
    fn the_body_is_the_text_of_the_frame_explanation() {
        let html = render_frame_page();
        let main = &html[html.find("<main").unwrap()..];
        assert!(main.contains("<h2>The header</h2>"));
        assert!(html.matches("<h2>").count() >= 3);
    }

    #[test]
    fn the_text_holds_no_raw_html() {
        use pulldown_cmark::{Event, Parser};
        let raw: Vec<Event> = Parser::new(FRAME)
            .filter(|e| matches!(e, Event::Html(_) | Event::InlineHtml(_)))
            .collect();
        assert!(raw.is_empty(), "{raw:?}");
    }

    #[test]
    fn its_heading_fits_the_frame() {
        assert!(render_frame_page().contains("<h1 "));
    }

    #[test]
    fn has_its_description_and_preview_tags() {
        let html = render_frame_page();
        let url = "https://www.lucalorenzon.com/style-guide/frame.html";
        assert!(html.contains(&format!(
            "<meta name=\"description\" content=\"{DESCRIPTION}\">"
        )));
        assert!(html.contains("<meta property=\"og:title\" content=\"Page frame\">"));
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
        assert!(render_frame_page().contains(
            "<link rel=\"canonical\" href=\"https://www.lucalorenzon.com/style-guide/frame.html\">"
        ));
    }

    #[test]
    fn sets_tab_title_and_has_page_frame_as_only_h1() {
        let html = render_frame_page();
        assert!(html.contains("<title>Page frame - Style guide - Luca Lorenzon</title>"));
        assert_eq!(html.matches("<h1").count(), 1);
        assert!(html.contains(">Page frame</h1>"));
    }

    #[test]
    fn addresses_are_relative_to_the_guides_folder() {
        let html = render_frame_page();
        assert!(html.contains("href=\"../style.css\""));
        assert!(html.contains("<link rel=\"icon\" href=\"../favicon.ico\""));
        assert!(html.contains("<a href=\"../about.html\""));
        assert!(html.contains("<a href=\"../\" aria-label=\"Lvk@73r, home page\""));
        assert!(!html.contains("href=\"/"));
        assert!(!html.contains("src=\"/"));
    }

    #[test]
    fn footer_says_reuse_with_attribution() {
        let html = render_frame_page();
        let footer = &html[html.find("<footer").unwrap()..];
        assert!(footer.contains(">Reuse with attribution</a>"));
    }
}
