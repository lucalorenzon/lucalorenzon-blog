//! The index of the style guide: the text of `content/style-guide/index.md`, then the list of
//! the guide's pages, written by the build.

use super::{FOLDER, colours_page};
use crate::markdown;
use crate::page_frame::{FooterLicence, Heading, IndexedPage, Indexing, PageFrame, PageKind};
use crate::site_address::SiteAddress;
use leptos::prelude::*;

/// The text above the list, written in Markdown.
pub(crate) const INDEX: &str = include_str!("../../content/style-guide/index.md");

/// What search engines and link previews show of the page.
const DESCRIPTION: &str = "The style guide of this site: its design decisions, right or wrong, gathered in one place and each tested once.";

/// The guide's pages: the title the reader sees and the file, in the guide's folder.
const PAGES: [(&str, &str); 1] = [("Colours", colours_page::FILE)];

#[component]
pub fn IndexPage(canonical: String) -> impl IntoView {
    let entries = PAGES
        .iter()
        .map(|&(title, file)| view! { <li><a href=file>{title}</a></li> })
        .collect_view();
    view! {
        <PageFrame
            title="Style guide - Luca Lorenzon"
            heading=Heading::try_from("Style guide").expect("the style guide's heading fits the frame")
            address_prefix="../"
            footer_licence=FooterLicence::ReuseWithAttribution
            indexing=Indexing::Indexed(IndexedPage {
                canonical,
                description: DESCRIPTION.to_owned(),
                kind: PageKind::Website,
            })
        >
            <main class="long-text-column md:long-text-large">
                <div inner_html=markdown::to_html(INDEX)></div>
                <ul>{entries}</ul>
            </main>
        </PageFrame>
    }
}

/// Renders the index of the style guide to a complete HTML document string. Its address is the
/// folder's, ending with a slash, like the home page's.
pub fn render_index_page(site: &SiteAddress) -> String {
    let owner = Owner::new();
    let canonical = site.join(FOLDER);
    let body = owner.with(|| view! { <IndexPage canonical=canonical /> }.to_html());
    format!("<!DOCTYPE html>{body}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn site() -> SiteAddress {
        SiteAddress::parse("https://www.lucalorenzon.com/").unwrap()
    }

    fn render_index_page() -> String {
        super::render_index_page(&site())
    }

    #[test]
    fn lists_the_colours_page() {
        let html = render_index_page();
        assert!(html.contains("<ul><li><a href=\"colours.html\">Colours</a></li>"));
    }

    #[test]
    fn shows_the_text_before_the_list() {
        let html = render_index_page();
        // Not tied to the wording: the text's first paragraph comes before the list.
        let main = html.find("<main").unwrap();
        let text = main + html[main..].find("<p>").unwrap();
        assert!(text < html.find("<ul>").unwrap());
    }

    /// ADR-001: the page carries only named variants.
    #[test]
    fn every_class_in_the_page_is_a_named_variant_of_the_stylesheet() {
        crate::style_guide::assert_only_named_variants(&render_index_page());
    }

    #[test]
    fn its_heading_fits_the_frame() {
        assert!(render_index_page().contains("<h1 "));
    }

    #[test]
    fn has_its_description_and_preview_tags() {
        let html = render_index_page();
        let url = "https://www.lucalorenzon.com/style-guide/";
        assert!(html.contains(&format!(
            "<meta name=\"description\" content=\"{DESCRIPTION}\">"
        )));
        assert!(html.contains("<meta property=\"og:title\" content=\"Style guide\">"));
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
        assert!(render_index_page().contains(
            "<link rel=\"canonical\" href=\"https://www.lucalorenzon.com/style-guide/\">"
        ));
    }

    #[test]
    fn sets_tab_title_and_has_the_guide_as_only_h1() {
        let html = render_index_page();
        assert!(html.contains("<title>Style guide - Luca Lorenzon</title>"));
        assert_eq!(html.matches("<h1").count(), 1);
        assert!(html.contains(">Style guide</h1>"));
    }

    #[test]
    fn addresses_are_relative_to_the_guides_folder() {
        let html = render_index_page();
        assert!(html.contains("href=\"../style.css\""));
        assert!(html.contains("<a href=\"../about.html\""));
        assert!(!html.contains("href=\"/"));
    }
}
