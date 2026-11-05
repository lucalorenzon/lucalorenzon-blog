use crate::markdown;
use crate::page_frame::{FooterLicence, PageFrame};
use leptos::prelude::*;

const AFTER_SAVE_EVENT_TITLE: &str = "After Save Event";
const AFTER_SAVE_EVENT_TAB: &str = "After Save Event - Luca Lorenzon";
const AFTER_SAVE_EVENT: &str = include_str!("../content/articles/after-save-event.md");

const HYBRIS_PK_TITLE: &str = "How to generate hybris PK values without hybris";
const HYBRIS_PK_TAB: &str = "How to generate hybris PK values without hybris - Luca Lorenzon";
const HYBRIS_PK: &str =
    include_str!("../content/articles/how-to-generate-hybris-pk-values-without-hybris.md");

/// A whole article page document, `<html>` element included. The page lives in the `articles/`
/// folder, so every address is relative to that folder (ADR-002). The Markdown has no title:
/// the title is the page heading.
#[component]
pub fn ArticlePage(
    tab_title: &'static str,
    title: &'static str,
    text: &'static str,
) -> impl IntoView {
    view! {
        <PageFrame
            title=tab_title
            heading=title
            address_prefix="../"
            footer_licence=FooterLicence::ReuseWithAttribution
        >
            <main class="long-text-column md:long-text-large" inner_html=markdown::to_html(text)></main>
        </PageFrame>
    }
}

fn render_article_page(tab_title: &'static str, title: &'static str, text: &'static str) -> String {
    let owner = Owner::new();
    let body = owner
        .with(|| view! { <ArticlePage tab_title=tab_title title=title text=text /> }.to_html());
    format!("<!DOCTYPE html>{body}")
}

/// Renders the After Save Event article page to a complete HTML document string.
pub fn render_after_save_event_page() -> String {
    render_article_page(
        AFTER_SAVE_EVENT_TAB,
        AFTER_SAVE_EVENT_TITLE,
        AFTER_SAVE_EVENT,
    )
}

/// Renders the hybris PK article page to a complete HTML document string.
pub fn render_hybris_pk_page() -> String {
    render_article_page(HYBRIS_PK_TAB, HYBRIS_PK_TITLE, HYBRIS_PK)
}

#[cfg(test)]
mod tests {

    mod after_save_event {
        use super::super::*;

        fn page() -> String {
            render_after_save_event_page()
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
        fn footer_says_reuse_with_attribution_and_links_to_the_about_page() {
            let html = page();
            let footer = &html[html.find("<footer").expect("footer")..];
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
            render_hybris_pk_page()
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
        fn footer_says_reuse_with_attribution_and_links_to_the_about_page() {
            let html = page();
            let footer = &html[html.find("<footer").expect("footer")..];
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
}
