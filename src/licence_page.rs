use crate::markdown;
use crate::page_frame::{FooterLicence, IndexedPage, Indexing, PageFrame, PageKind};
use crate::site_address::SiteAddress;
use leptos::prelude::*;

/// The licences of the site, written in Markdown and converted to HTML at
/// build time.
pub(crate) const LICENCE: &str = include_str!("../content/licence.md");

/// The whole licence page document, `<html>` element included.
/// What search engines and link previews show of the page.
const DESCRIPTION: &str = "How you may reuse this site: the articles under CC BY 4.0, the code samples under MIT-0, the curriculum under CC BY-ND 4.0.";

#[component]
pub fn LicencePage(canonical: String) -> impl IntoView {
    view! {
        <PageFrame
            title="Licence - Luca Lorenzon"
            heading="Licence"
            address_prefix=""
            footer_licence=FooterLicence::ReuseWithAttribution
            indexing=Indexing::Indexed(IndexedPage {
                canonical,
                description: DESCRIPTION,
                kind: PageKind::Website,
            })
        >
            <main class="long-text-column md:long-text-large" inner_html=markdown::to_html(LICENCE)></main>
        </PageFrame>
    }
}

/// Renders the licence page to a complete HTML document string.
pub fn render_licence_page(site: &SiteAddress) -> String {
    let owner = Owner::new();
    let canonical = site.join("licence.html");
    let body = owner.with(|| view! { <LicencePage canonical=canonical /> }.to_html());
    format!("<!DOCTYPE html>{body}")
}

#[cfg(test)]
mod tests {
    use super::render_licence_page as render;
    use crate::site_address::SiteAddress;

    fn site() -> SiteAddress {
        SiteAddress::parse("https://www.lucalorenzon.com/").unwrap()
    }

    fn render_licence_page() -> String {
        render(&site())
    }

    #[test]
    fn has_its_description_and_preview_tags() {
        let html = render_licence_page();
        let url = "https://www.lucalorenzon.com/licence.html";
        assert!(html.contains("<meta name=\"description\" content=\"How you may reuse this site: the articles under CC BY 4.0, the code samples under MIT-0, the curriculum under CC BY-ND 4.0.\">"));
        assert!(html.contains("<meta property=\"og:title\" content=\"Licence\">"));
        assert!(html.contains("<meta property=\"og:type\" content=\"website\">"));
        assert!(html.contains(&format!("<meta property=\"og:url\" content=\"{url}\">")));
        assert!(html.contains("<meta property=\"og:description\" content=\"How you may reuse this site: the articles under CC BY 4.0, the code samples under MIT-0, the curriculum under CC BY-ND 4.0.\">"));
        assert!(html.contains("<meta property=\"og:site_name\" content=\"Luca Lorenzon\">"));
        assert!(html.contains("<meta property=\"og:locale\" content=\"en_GB\">"));
        assert!(!html.contains("og:image"));
    }

    #[test]
    fn description_fits_a_search_result() {
        assert!(super::DESCRIPTION.chars().count() <= 160);
    }

    #[test]
    fn has_its_own_full_address_as_canonical_link() {
        assert!(render_licence_page().contains(
            "<link rel=\"canonical\" href=\"https://www.lucalorenzon.com/licence.html\">"
        ));
    }

    #[test]
    fn canonical_link_follows_the_prefix_of_the_site_address() {
        let site = SiteAddress::parse("https://example.com/blog/").unwrap();
        assert!(
            render(&site).contains(
                "<link rel=\"canonical\" href=\"https://example.com/blog/licence.html\">"
            )
        );
    }

    #[test]
    fn starts_with_doctype() {
        assert!(render_licence_page().starts_with("<!DOCTYPE html>"));
    }

    #[test]
    fn declares_english_language() {
        assert!(render_licence_page().contains("lang=\"en\""));
    }

    #[test]
    fn sets_tab_title() {
        assert!(render_licence_page().contains("<title>Licence - Luca Lorenzon</title>"));
    }

    #[test]
    fn has_licence_as_only_h1() {
        let html = render_licence_page();
        assert_eq!(html.matches("<h1").count(), 1);
        assert!(html.contains(">Licence</h1>"));
    }

    #[test]
    fn shows_the_licence_text_in_main() {
        let html = render_licence_page();
        let main = &html[html.find("<main").expect("main")..html.find("</main>").expect("end")];
        assert!(main.contains("<h2>In short</h2>"));
        assert!(!main.contains("<h1"));
    }

    #[test]
    fn footer_says_reuse_with_attribution_and_links_to_the_about_page() {
        let html = render_licence_page();
        let footer = &html[html.find("<footer").expect("footer")..];
        assert!(footer.contains("© 2026 Luca Lorenzon · "));
        assert!(footer.contains(">Reuse with attribution</a>"));
        assert!(footer.contains("<a href=\"about.html\""));
    }

    #[test]
    fn logo_links_to_the_home_page_with_a_relative_address() {
        assert!(render_licence_page().contains("<a href=\"./\" aria-label=\"Lvk@73r, home page\""));
    }

    #[test]
    fn links_favicon_with_relative_path() {
        assert!(render_licence_page().contains("<link rel=\"icon\" href=\"favicon.ico\""));
    }

    #[test]
    fn links_stylesheet_with_relative_path() {
        assert!(render_licence_page().contains("href=\"style.css\""));
    }

    #[test]
    fn has_no_root_absolute_references() {
        let html = render_licence_page();
        assert!(!html.contains("href=\"/"));
        assert!(!html.contains("src=\"/"));
    }
}
