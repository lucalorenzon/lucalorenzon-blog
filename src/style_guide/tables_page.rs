//! The tables page of the style guide: the text of `content/style-guide/tables.md`, whose
//! tables are the cases the browser tests of the component run on. The page is the Markdown
//! converted as every article is, so the tables are the real component.

use super::FOLDER;
use super::navigation::GuideBody;
use crate::markdown;
use crate::page_frame::{FooterLicence, Heading, IndexedPage, Indexing, PageFrame, PageKind};
use crate::site_address::SiteAddress;
use leptos::prelude::*;

/// The page's file in the guide's folder, as the index links to it.
pub const FILE: &str = "tables.html";

/// The text of the page, written in Markdown: an introduction, then one section for each case
/// the component is put to the test with.
pub(crate) const TABLES: &str = include_str!("../../content/style-guide/tables.md");

/// What search engines and link previews show of the page.
const DESCRIPTION: &str = "The tables of this site: when a table stays a table, when it becomes a record viewer, rows of different heights, and values kept on one line.";

#[component]
pub fn TablesPage(canonical: String, body: String) -> impl IntoView {
    view! {
        <PageFrame
            title="Tables - Style guide - Luca Lorenzon"
            heading=Heading::try_from("Tables").expect("the tables page's heading fits the frame")
            address_prefix="../"
            footer_licence=FooterLicence::ReuseWithAttribution
            indexing=Indexing::Indexed(IndexedPage {
                canonical,
                description: DESCRIPTION.to_owned(),
                kind: PageKind::Website,
            })
        >
            <GuideBody title="Tables" body=body />
        </PageFrame>
    }
}

/// Renders the tables page to a complete HTML document string.
pub fn render_tables_page(site: &SiteAddress) -> String {
    let body = markdown::to_html(TABLES);
    let owner = Owner::new();
    let canonical = site.join(&format!("{FOLDER}{FILE}"));
    let page = owner.with(|| view! { <TablesPage canonical=canonical body=body /> }.to_html());
    format!("<!DOCTYPE html>{page}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn site() -> SiteAddress {
        SiteAddress::parse("https://www.lucalorenzon.com/").unwrap()
    }

    fn render_tables_page() -> String {
        super::render_tables_page(&site())
    }

    /// The titles of the six cases: the browser tests find each case by its title.
    const CASE_TITLES: [&str; 6] = [
        "A table that always fits",
        "Six columns",
        "Rows of different heights",
        "A range with an en dash",
        "A word with a hyphen",
        "Texts at the limit",
    ];

    #[test]
    fn the_cases_are_the_real_component() {
        let html = markdown::to_html(TABLES);
        assert!(html.matches("<h2>").count() >= 6);
        assert!(html.matches("class=\"table-with-viewer").count() >= 6);
        assert!(render_tables_page().contains("class=\"record-viewer"));
    }

    #[test]
    fn the_page_holds_the_six_cases_each_in_a_section_with_one_table() {
        let html = markdown::to_html(TABLES);
        let sections: Vec<&str> = html.split("<h2>").skip(1).collect();
        assert_eq!(sections.len(), CASE_TITLES.len());
        for (section, title) in sections.iter().zip(CASE_TITLES) {
            assert!(section.starts_with(&format!("{title}</h2>")), "{title}");
            assert_eq!(
                section.matches("class=\"table-with-viewer").count(),
                1,
                "{title}"
            );
        }
    }

    #[test]
    fn its_heading_fits_the_frame() {
        assert!(render_tables_page().contains("<h1 "));
    }

    #[test]
    fn has_its_description_and_preview_tags() {
        let html = render_tables_page();
        let url = "https://www.lucalorenzon.com/style-guide/tables.html";
        assert!(html.contains(&format!(
            "<meta name=\"description\" content=\"{DESCRIPTION}\">"
        )));
        assert!(html.contains("<meta property=\"og:title\" content=\"Tables\">"));
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
        assert!(render_tables_page().contains(
            "<link rel=\"canonical\" href=\"https://www.lucalorenzon.com/style-guide/tables.html\">"
        ));
    }

    #[test]
    fn sets_tab_title_and_has_tables_as_only_h1() {
        let html = render_tables_page();
        assert!(html.contains("<title>Tables - Style guide - Luca Lorenzon</title>"));
        assert_eq!(html.matches("<h1").count(), 1);
        assert!(html.contains(">Tables</h1>"));
    }

    #[test]
    fn addresses_are_relative_to_the_guides_folder() {
        let html = render_tables_page();
        assert!(html.contains("href=\"../style.css\""));
        assert!(html.contains("<link rel=\"icon\" href=\"../favicon.ico\""));
        assert!(html.contains("<a href=\"../about.html\""));
        assert!(html.contains("<a href=\"../\" aria-label=\"Lvk@73r, home page\""));
        assert!(!html.contains("href=\"/"));
        assert!(!html.contains("src=\"/"));
    }

    #[test]
    fn footer_says_reuse_with_attribution() {
        let html = render_tables_page();
        let footer = &html[html.find("<footer").unwrap()..];
        assert!(footer.contains(">Reuse with attribution</a>"));
    }
}
