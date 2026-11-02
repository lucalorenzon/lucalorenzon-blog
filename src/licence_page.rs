use crate::markdown;
use crate::page_frame::{FooterLicence, PageFrame};
use leptos::prelude::*;

/// The licences of the site, written in Markdown and converted to HTML at
/// build time.
const LICENCE: &str = include_str!("../content/licence.md");

/// The whole licence page document, `<html>` element included.
#[component]
pub fn LicencePage() -> impl IntoView {
    view! {
        <PageFrame
            title="Licence - Luca Lorenzon"
            heading="Licence"
            address_prefix=""
            footer_licence=FooterLicence::ReuseWithAttribution
        >
            <main class="long-text-column" inner_html=markdown::to_html(LICENCE)></main>
        </PageFrame>
    }
}

/// Renders the licence page to a complete HTML document string.
pub fn render_licence_page() -> String {
    let owner = Owner::new();
    let body = owner.with(|| view! { <LicencePage /> }.to_html());
    format!("<!DOCTYPE html>{body}")
}

#[cfg(test)]
mod tests {
    use super::render_licence_page;

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
    fn footer_says_reuse_with_attribution_and_links_to_this_page() {
        let html = render_licence_page();
        let footer = &html[html.find("<footer").expect("footer")..];
        assert!(footer.contains("© 2026 Luca Lorenzon · "));
        assert!(footer.contains(">Reuse with attribution</a>"));
        assert!(footer.contains("<a href=\"licence.html\""));
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
