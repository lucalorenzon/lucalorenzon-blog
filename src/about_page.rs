use crate::markdown;
use crate::page_frame::{FooterLicence, PageFrame};
use leptos::prelude::*;

/// Who the author is and how to reuse the site, written in Markdown and converted to HTML at
/// build time.
const ABOUT: &str = include_str!("../content/about.md");

/// The whole about page document, `<html>` element included.
#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <PageFrame
            title="About - Luca Lorenzon"
            heading="About"
            address_prefix=""
            footer_licence=FooterLicence::ReuseWithAttribution
        >
            <main class="long-text-column md:long-text-large" inner_html=markdown::to_html(ABOUT)></main>
        </PageFrame>
    }
}

/// Renders the about page to a complete HTML document string.
pub fn render_about_page() -> String {
    let owner = Owner::new();
    let body = owner.with(|| view! { <AboutPage /> }.to_html());
    format!("<!DOCTYPE html>{body}")
}

#[cfg(test)]
mod tests {
    use super::render_about_page;

    #[test]
    fn starts_with_doctype() {
        assert!(render_about_page().starts_with("<!DOCTYPE html>"));
    }

    #[test]
    fn declares_english_language() {
        assert!(render_about_page().contains("lang=\"en\""));
    }

    #[test]
    fn sets_tab_title() {
        assert!(render_about_page().contains("<title>About - Luca Lorenzon</title>"));
    }

    #[test]
    fn has_about_as_only_h1() {
        let html = render_about_page();
        assert_eq!(html.matches("<h1").count(), 1);
        assert!(html.contains(">About</h1>"));
    }

    #[test]
    fn shows_the_about_text_in_main() {
        let html = render_about_page();
        let main = &html[html.find("<main").expect("main")..html.find("</main>").expect("end")];
        assert!(main.contains("<h2>My views</h2>"));
        assert!(main.contains("<h2>Reusing what you read</h2>"));
        assert!(!main.contains("<h1"));
    }

    #[test]
    fn links_to_the_curriculum_page_with_a_relative_address() {
        assert!(render_about_page().contains("<a href=\"curriculum.html\">my curriculum</a>"));
    }

    #[test]
    fn footer_says_reuse_with_attribution_and_links_to_the_about_page() {
        let html = render_about_page();
        let footer = &html[html.find("<footer").expect("footer")..];
        assert!(footer.contains("© 2026 Luca Lorenzon · "));
        assert!(footer.contains(">Reuse with attribution</a>"));
        assert!(footer.contains("<a href=\"about.html\""));
    }

    #[test]
    fn logo_links_to_the_home_page_with_a_relative_address() {
        assert!(render_about_page().contains("<a href=\"./\" aria-label=\"Lvk@73r, home page\""));
    }

    #[test]
    fn links_favicon_with_relative_path() {
        assert!(render_about_page().contains("<link rel=\"icon\" href=\"favicon.ico\""));
    }

    #[test]
    fn links_stylesheet_with_relative_path() {
        assert!(render_about_page().contains("href=\"style.css\""));
    }

    #[test]
    fn has_no_root_absolute_references() {
        let html = render_about_page();
        assert!(!html.contains("href=\"/"));
        assert!(!html.contains("src=\"/"));
    }
}
