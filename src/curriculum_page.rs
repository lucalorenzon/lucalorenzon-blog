use crate::markdown;
use crate::page_frame::{FooterLicence, IndexedPage, Indexing, PageFrame, PageKind};
use crate::site_address::SiteAddress;
use leptos::prelude::*;

/// The curriculum, written in Markdown and converted to HTML at build time.
pub(crate) const CURRICULUM: &str = include_str!("../content/curriculum.md");

/// The whole curriculum page document, `<html>` element included.
/// What search engines and link previews show of the page.
const DESCRIPTION: &str = "Curriculum of Luca Lorenzon: collaboration, architecture and hands-on implementation in e-commerce and enterprise integration, in international contexts.";

#[component]
pub fn CurriculumPage(canonical: String) -> impl IntoView {
    view! {
        <PageFrame
            title="Curriculum Vitae - Luca Lorenzon"
            heading="Curriculum Vitae"
            address_prefix=""
            footer_licence=FooterLicence::CurriculumCopyNotChange
            indexing=Indexing::Indexed(IndexedPage {
                canonical,
                description: DESCRIPTION,
                kind: PageKind::Profile,
            })
        >
            <main class="long-text-column md:long-text-large" inner_html=markdown::to_html(CURRICULUM)></main>
        </PageFrame>
    }
}

/// Renders the curriculum page to a complete HTML document string.
pub fn render_curriculum_page(site: &SiteAddress) -> String {
    let owner = Owner::new();
    let canonical = site.join("curriculum.html");
    let body = owner.with(|| view! { <CurriculumPage canonical=canonical /> }.to_html());
    format!("<!DOCTYPE html>{body}")
}

#[cfg(test)]
mod tests {
    use super::render_curriculum_page as render;
    use crate::site_address::SiteAddress;

    fn site() -> SiteAddress {
        SiteAddress::parse("https://www.lucalorenzon.com/").unwrap()
    }

    fn render_curriculum_page() -> String {
        render(&site())
    }

    #[test]
    fn has_its_description_and_preview_tags() {
        let html = render_curriculum_page();
        let url = "https://www.lucalorenzon.com/curriculum.html";
        assert!(html.contains("<meta name=\"description\" content=\"Curriculum of Luca Lorenzon: collaboration, architecture and hands-on implementation in e-commerce and enterprise integration, in international contexts.\">"));
        assert!(html.contains("<meta property=\"og:title\" content=\"Curriculum Vitae\">"));
        assert!(html.contains("<meta property=\"og:type\" content=\"profile\">"));
        assert!(html.contains(&format!("<meta property=\"og:url\" content=\"{url}\">")));
        assert!(html.contains("<meta property=\"og:description\" content=\"Curriculum of Luca Lorenzon: collaboration, architecture and hands-on implementation in e-commerce and enterprise integration, in international contexts.\">"));
        assert!(html.contains("<meta property=\"og:site_name\" content=\"Luca Lorenzon\">"));
        assert!(html.contains("<meta property=\"og:locale\" content=\"en_GB\">"));
        assert!(!html.contains("og:image"));
        assert!(html.contains("<meta property=\"profile:first_name\" content=\"Luca\">"));
        assert!(html.contains("<meta property=\"profile:last_name\" content=\"Lorenzon\">"));
    }

    #[test]
    fn description_fits_a_search_result() {
        assert!(super::DESCRIPTION.chars().count() <= 160);
    }

    #[test]
    fn has_its_own_full_address_as_canonical_link() {
        assert!(render_curriculum_page().contains(
            "<link rel=\"canonical\" href=\"https://www.lucalorenzon.com/curriculum.html\">"
        ));
    }

    #[test]
    fn canonical_link_follows_the_prefix_of_the_site_address() {
        let site = SiteAddress::parse("https://example.com/blog/").unwrap();
        assert!(render(&site).contains(
            "<link rel=\"canonical\" href=\"https://example.com/blog/curriculum.html\">"
        ));
    }

    #[test]
    fn starts_with_doctype() {
        assert!(render_curriculum_page().starts_with("<!DOCTYPE html>"));
    }

    #[test]
    fn declares_english_language() {
        assert!(render_curriculum_page().contains("lang=\"en\""));
    }

    #[test]
    fn sets_tab_title() {
        assert!(
            render_curriculum_page().contains("<title>Curriculum Vitae - Luca Lorenzon</title>")
        );
    }

    #[test]
    fn has_curriculum_vitae_as_only_h1() {
        let html = render_curriculum_page();
        assert_eq!(html.matches("<h1").count(), 1);
        assert!(html.contains(">Curriculum Vitae</h1>"));
    }

    #[test]
    fn shows_the_curriculum_in_main() {
        let html = render_curriculum_page();
        let main = &html[html.find("<main").expect("main")..html.find("</main>").expect("end")];
        assert!(main.contains("<h2>Summary</h2>"));
        assert!(!main.contains("<h1"));
    }

    #[test]
    fn shows_copyright_in_footer() {
        let html = render_curriculum_page();
        let footer = &html[html.find("<footer").expect("footer")..];
        assert!(footer.contains("© 2026 Luca Lorenzon · "));
    }

    #[test]
    fn footer_says_the_cv_may_be_copied_and_links_to_the_about_page() {
        let html = render_curriculum_page();
        let footer = &html[html.find("<footer").expect("footer")..];
        assert!(footer.contains(">This CV may be copied, not changed</a>"));
        assert!(footer.contains("<a href=\"about.html\""));
    }

    #[test]
    fn links_favicon_with_relative_path() {
        assert!(render_curriculum_page().contains("<link rel=\"icon\" href=\"favicon.ico\""));
    }

    #[test]
    fn logo_is_one_link_to_the_home_page_with_one_name() {
        let html = render_curriculum_page();
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
        let html = render_curriculum_page();
        assert!(!html.contains("<pre"));
    }

    #[test]
    fn logo_contains_the_ascii_art() {
        let html = render_curriculum_page();
        let escaped = crate::page_frame::LOGO.replace('<', "&lt;");
        assert!(html.contains(&escaped), "ASCII art not found in HTML");
    }

    #[test]
    fn links_stylesheet_with_relative_path() {
        assert!(render_curriculum_page().contains("href=\"style.css\""));
    }

    #[test]
    fn has_no_root_absolute_references() {
        let html = render_curriculum_page();
        assert!(!html.contains("href=\"/"));
        assert!(!html.contains("src=\"/"));
    }

    #[test]
    fn has_an_empty_reserved_menu_slot() {
        let html = render_curriculum_page();
        assert!(html.contains("data-testid=\"menu-slot\""));
        assert!(html.contains("aria-hidden=\"true\""));
    }

    #[test]
    fn has_text_logo_for_narrow_screens() {
        assert!(render_curriculum_page().contains(">Lvk@73r</div>"));
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

        let html = render_curriculum_page();
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
