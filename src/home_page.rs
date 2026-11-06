use crate::article_page::newest_first;
use crate::page_frame::{FooterLicence, IndexedPage, Indexing, PageFrame, PageKind};
use crate::site_address::SiteAddress;
use leptos::prelude::*;

/// The whole home page document, `<html>` element included: the list of the articles, newest
/// first.
/// What search engines and link previews show of the page.
const DESCRIPTION: &str = "Articles by Luca Lorenzon, product engineer: what I have learned, my thoughts on topics of public debate, and stories of paths I have taken.";

#[component]
pub fn HomePage(canonical: String) -> impl IntoView {
    let entries = newest_first()
        .into_iter()
        .map(|article| {
            view! {
                <li>
                    <h2>
                        <a href=article.address>{article.title}</a>
                    </h2>
                    <p>{article.description}</p>
                    <time datetime=article.date.iso()>{article.date.long_form()}</time>
                </li>
            }
        })
        .collect_view();
    view! {
        <PageFrame
            title="Home Page - Luca Lorenzon"
            heading="Home Page"
            address_prefix=""
            footer_licence=FooterLicence::ReuseWithAttribution
            indexing=Indexing::Indexed(IndexedPage {
                canonical,
                description: DESCRIPTION,
                kind: PageKind::Website,
            })
        >
            <main class="long-text-column md:long-text-large">
                <ul class="article-list">{entries}</ul>
            </main>
        </PageFrame>
    }
}

/// Renders the home page to a complete HTML document string.
pub fn render_home_page(site: &SiteAddress) -> String {
    let owner = Owner::new();
    let canonical = site.join("");
    let body = owner.with(|| view! { <HomePage canonical=canonical /> }.to_html());
    format!("<!DOCTYPE html>{body}")
}

#[cfg(test)]
mod tests {
    use super::render_home_page as render;
    use crate::site_address::SiteAddress;

    fn site() -> SiteAddress {
        SiteAddress::parse("https://www.lucalorenzon.com/").unwrap()
    }

    fn render_home_page() -> String {
        render(&site())
    }

    #[test]
    fn has_its_description_and_preview_tags() {
        let html = render_home_page();
        let url = "https://www.lucalorenzon.com/";
        assert!(html.contains("<meta name=\"description\" content=\"Articles by Luca Lorenzon, product engineer: what I have learned, my thoughts on topics of public debate, and stories of paths I have taken.\">"));
        assert!(html.contains("<meta property=\"og:title\" content=\"Home Page\">"));
        assert!(html.contains("<meta property=\"og:type\" content=\"website\">"));
        assert!(html.contains(&format!("<meta property=\"og:url\" content=\"{url}\">")));
        assert!(html.contains("<meta property=\"og:description\" content=\"Articles by Luca Lorenzon, product engineer: what I have learned, my thoughts on topics of public debate, and stories of paths I have taken.\">"));
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
        assert!(
            render_home_page()
                .contains("<link rel=\"canonical\" href=\"https://www.lucalorenzon.com/\">")
        );
    }

    #[test]
    fn canonical_link_follows_the_prefix_of_the_site_address() {
        let site = SiteAddress::parse("https://example.com/blog/").unwrap();
        assert!(
            render(&site).contains("<link rel=\"canonical\" href=\"https://example.com/blog/\">")
        );
    }

    fn main_of(html: &str) -> &str {
        &html[html.find("<main").expect("main")..html.find("</main>").expect("end")]
    }

    #[test]
    fn starts_with_doctype() {
        assert!(render_home_page().starts_with("<!DOCTYPE html>"));
    }

    #[test]
    fn declares_english_language() {
        assert!(render_home_page().contains("lang=\"en\""));
    }

    #[test]
    fn sets_tab_title() {
        assert!(render_home_page().contains("<title>Home Page - Luca Lorenzon</title>"));
    }

    #[test]
    fn has_home_page_as_only_h1() {
        let html = render_home_page();
        assert_eq!(html.matches("<h1").count(), 1);
        assert!(html.contains(">Home Page</h1>"));
        assert!(!main_of(&html).contains("<h1"));
    }

    #[test]
    fn lists_both_articles_with_relative_links() {
        let main = render_home_page();
        let main = main_of(&main);
        assert!(main.contains("<ul class=\"article-list\">"));
        assert_eq!(main.matches("<li>").count(), 2);
        assert!(main.contains("<a href=\"articles/after-save-event.html\">After Save Event</a>"));
        assert!(main.contains(
            "<a href=\"articles/how-to-generate-hybris-pk-values-without-hybris.html\">How to generate hybris PK values without hybris</a>"
        ));
        assert!(!main.contains("href=\"/"));
    }

    #[test]
    fn titles_are_h2_headings() {
        let main = render_home_page();
        assert_eq!(main_of(&main).matches("<h2>").count(), 2);
    }

    #[test]
    fn lists_the_newest_article_first() {
        let html = render_home_page();
        let main = main_of(&html);
        let newer = main.find("After Save Event").expect("newer");
        let older = main.find("How to generate hybris PK").expect("older");
        assert!(newer < older);
    }

    #[test]
    fn shows_each_description_then_its_date() {
        let html = render_home_page();
        let main = main_of(&html);
        let first = "<p>SAP hybris change events system: architecture and inner behaviour.</p>\
            <time datetime=\"2023-04-30\">30 April 2023</time>";
        let second = "<p>Dive into the PK generation logic of SAP hybris, for extreme use cases.</p>\
            <time datetime=\"2023-04-25\">25 April 2023</time>";
        assert!(main.contains(first), "{main}");
        assert!(main.contains(second), "{main}");
    }

    #[test]
    fn does_not_show_the_curriculum() {
        assert!(!render_home_page().contains("Curriculum Vitae"));
    }

    #[test]
    fn shows_copyright_in_footer() {
        let html = render_home_page();
        let footer = &html[html.find("<footer").expect("footer")..];
        assert!(footer.contains("© 2026 Luca Lorenzon · "));
    }

    #[test]
    fn footer_says_reuse_with_attribution_and_links_to_the_about_page() {
        let html = render_home_page();
        let footer = &html[html.find("<footer").expect("footer")..];
        assert!(footer.contains(">Reuse with attribution</a>"));
        assert!(footer.contains("<a href=\"about.html\""));
    }

    #[test]
    fn links_favicon_with_relative_path() {
        assert!(render_home_page().contains("<link rel=\"icon\" href=\"favicon.ico\""));
    }

    #[test]
    fn logo_is_one_link_to_the_home_page_with_one_name() {
        let html = render_home_page();
        assert!(
            html.contains("<a href=\"./\" aria-label=\"Lvk@73r, home page\" class=\"logo-link\">")
        );
        // The name is the link's; the art is hidden, not read, and no nested image.
        assert!(!html.contains("role=\"img\""));
        assert!(html.contains("data-testid=\"ascii-logo\" aria-hidden=\"true\""));
    }

    /// Orca 51 presents a `<pre>` as a code block and does not speak its name
    /// (card docs/wip/logo-name-not-read-by-orca-51.md).
    #[test]
    fn logo_is_not_a_pre_element() {
        let html = render_home_page();
        assert!(!html.contains("<pre"));
    }

    #[test]
    fn logo_contains_the_ascii_art() {
        let html = render_home_page();
        let escaped = crate::page_frame::LOGO.replace('<', "&lt;");
        assert!(html.contains(&escaped), "ASCII art not found in HTML");
    }

    #[test]
    fn links_stylesheet_with_relative_path() {
        assert!(render_home_page().contains("href=\"style.css\""));
    }

    #[test]
    fn has_no_root_absolute_references() {
        let html = render_home_page();
        assert!(!html.contains("href=\"/"));
        assert!(!html.contains("src=\"/"));
    }

    #[test]
    fn has_an_empty_reserved_menu_slot() {
        let html = render_home_page();
        assert!(html.contains("data-testid=\"menu-slot\""));
        assert!(html.contains("aria-hidden=\"true\""));
    }

    #[test]
    fn has_text_logo_for_narrow_screens() {
        assert!(render_home_page().contains(">Lvk@73r</div>"));
    }

    /// ADR-001: the page carries only our own named variants.
    #[test]
    fn every_class_in_the_page_is_a_named_variant_of_the_stylesheet() {
        let css = include_str!("../style/input.css");
        let defined: Vec<&str> = css
            .lines()
            .filter_map(|l| l.strip_prefix("@utility "))
            .map(|rest| rest.trim_end_matches('{').trim())
            .collect();
        assert!(!defined.is_empty());

        let html = render_home_page();
        let mut used = 0;
        for chunk in html.split("class=\"").skip(1) {
            let value = chunk.split('"').next().unwrap();
            for token in value.split_whitespace() {
                let name = token
                    .strip_prefix("md:")
                    .or_else(|| token.strip_prefix("xl:"))
                    .unwrap_or(token);
                assert!(
                    defined.contains(&name),
                    "class `{token}` is not an @utility in style/input.css"
                );
                used += 1;
            }
        }
        assert!(used > 0);
    }
}
