//! What the style guide's pages, except its index, have in common around their text: the
//! breadcrumb above it ("Style guide › Code blocks") and the link back to the top below it.
//! A page of the guide puts its text in [`GuideMain`] instead of writing its own `<main>`, so
//! the pages to come get both without asking.
//!
//! The breadcrumb follows the WAI-ARIA Authoring Practices pattern, except that it does not
//! start with "Home": the logo already leads home from every page. The current page is the
//! last item, not a link, marked with `aria-current="page"`. The separator is drawn by a
//! `span` hidden from screen readers.
//!
//! "Back to top" points at `#top`, the frame's outermost element. A link to a fragment moves
//! the browser's starting point for the Tab key to its target, so the next Tab reaches the
//! first thing of the page (the logo) rather than the thing after the link.

use leptos::prelude::*;

/// The guide's index, as a page of the guide writes the address: the folder holds it, and the
/// pages are in that folder.
const INDEX_ADDRESS: &str = "./";

/// The text of the link below the page.
const BACK_TO_TOP: &str = "Back to top";

/// The `<main>` of a page of the guide: breadcrumb, the page's text (HTML made from its
/// Markdown), and the link back to the top. `title` is the page's name in the breadcrumb.
#[component]
pub fn GuideMain(title: &'static str, body: String) -> impl IntoView {
    view! {
        <main class="long-text-column md:long-text-large">
            <nav aria-label="Breadcrumb" class="breadcrumb">
                <ol>
                    <li>
                        <a href=INDEX_ADDRESS>"Style guide"</a>
                        <span aria-hidden="true" class="breadcrumb-separator">"›"</span>
                    </li>
                    <li><span aria-current="page">{title}</span></li>
                </ol>
            </nav>
            <div inner_html=body></div>
            <p><a href="#top">{BACK_TO_TOP}</a></p>
        </main>
    }
}

#[cfg(test)]
mod tests {
    use crate::site_address::SiteAddress;
    use crate::style_guide::{code_page, colours_page, index_page, tables_page};

    fn site() -> SiteAddress {
        SiteAddress::parse("https://www.lucalorenzon.com/").unwrap()
    }

    fn pages() -> [(&'static str, String); 3] {
        [
            (
                "Colours",
                colours_page::render_colours_page(&site()).unwrap(),
            ),
            ("Code blocks", code_page::render_code_page(&site())),
            ("Tables", tables_page::render_tables_page(&site())),
        ]
    }

    #[test]
    fn the_breadcrumb_is_a_navigation_named_breadcrumb_with_a_list() {
        for (title, html) in pages() {
            assert!(
                html.contains("<nav aria-label=\"Breadcrumb\" class=\"breadcrumb\"><ol>"),
                "{title}"
            );
            assert_eq!(html.matches("<nav").count(), 1, "{title}");
        }
    }

    #[test]
    fn the_guide_links_to_the_index_and_the_current_page_is_not_a_link() {
        for (title, html) in pages() {
            let nav = &html[html.find("<nav").unwrap()..html.find("</nav>").unwrap()];
            assert!(nav.contains("<a href=\"./\">Style guide</a>"), "{title}");
            assert_eq!(
                nav.matches("<a ").count(),
                1,
                "{title}: only the guide is a link"
            );
            assert!(
                nav.contains(&format!("<span aria-current=\"page\">{title}</span>")),
                "{title}"
            );
            assert_eq!(nav.matches("aria-current").count(), 1, "{title}");
        }
    }

    #[test]
    fn the_separator_is_hidden_from_screen_readers_and_there_is_no_home() {
        for (title, html) in pages() {
            let nav = &html[html.find("<nav").unwrap()..html.find("</nav>").unwrap()];
            assert_eq!(nav.matches('›').count(), 1, "{title}");
            assert!(nav.contains("aria-hidden=\"true\" class=\"breadcrumb-separator\">›</span>"));
            assert!(!nav.contains("Home"), "{title}");
        }
    }

    #[test]
    fn the_breadcrumb_comes_after_the_header_and_before_the_text() {
        for (title, html) in pages() {
            let header_end = html.find("</header>").unwrap();
            let nav = html.find("<nav").unwrap();
            let text = html.find("<h2>").unwrap();
            assert!(header_end < nav && nav < text, "{title}");
        }
    }

    #[test]
    fn back_to_top_closes_the_main_text_and_points_at_the_top_of_the_page() {
        for (title, html) in pages() {
            assert_eq!(html.matches(">Back to top</a>").count(), 1, "{title}");
            let link = html.find(">Back to top</a>").unwrap();
            assert!(html[..link].ends_with("<p><a href=\"#top\""), "{title}");
            assert!(html.find("<h2>").unwrap() < link, "{title}");
            assert!(link < html.find("</main>").unwrap(), "{title}");
            assert!(html.find("</main>").unwrap() < html.find("<footer").unwrap());
            // The target exists, once, and holds the whole page, logo included.
            assert_eq!(html.matches("id=\"top\"").count(), 1, "{title}");
            assert!(html.find("id=\"top\"").unwrap() < html.find("<header").unwrap());
        }
    }

    #[test]
    fn the_index_has_neither() {
        let html = index_page::render_index_page(&site());
        assert!(!html.contains("<nav"));
        assert!(!html.contains("Breadcrumb"));
        assert!(!html.contains("Back to top"));
    }

    /// ADR-001: the component carries only named variants. Only its own markup is checked: the
    /// code blocks in the page's text carry container-query classes of their own.
    #[test]
    fn every_class_of_the_component_is_a_named_variant_of_the_stylesheet() {
        for (_, html) in pages() {
            let main = &html[html.find("<main").unwrap()..];
            let nav = &main[..main.find("</nav>").unwrap()];
            crate::style_guide::assert_only_named_variants(nav);
        }
    }
}
