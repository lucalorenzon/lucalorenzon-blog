use crate::page_frame::{FooterLicence, PageFrame};
use crate::site_address::SiteAddress;
use leptos::prelude::*;

/// The site's own "page not found", `<html>` element included.
///
/// Unlike every other page, it writes its addresses from the path part of the
/// site's address (`/style.css`, not `style.css`): the host serves it for any
/// missing address, at any depth, where a relative path would point to the
/// wrong place (ADR-002).
#[component]
pub fn NotFoundPage(site_path: String) -> impl IntoView {
    view! {
        <PageFrame
            title="Page not found - Luca Lorenzon"
            heading="Page not found"
            address_prefix=site_path.clone()
            footer_licence=FooterLicence::ReuseWithAttribution
        >
            <main class="long-text-column">
                <p>
                    <a href=site_path>"Go to the home page"</a>
                </p>
            </main>
        </PageFrame>
    }
}

/// Renders the "page not found" to a complete HTML document string.
pub fn render_not_found_page(address: &SiteAddress) -> String {
    let owner = Owner::new();
    let path = address.path().to_owned();
    let body = owner.with(|| view! { <NotFoundPage site_path=path /> }.to_html());
    format!("<!DOCTYPE html>{body}")
}

#[cfg(test)]
mod tests {
    use super::render_not_found_page;
    use crate::site_address::SiteAddress;

    fn render(address: &str) -> String {
        render_not_found_page(&SiteAddress::parse(address).unwrap())
    }

    fn default_page() -> String {
        render("https://www.lucalorenzon.com/")
    }

    #[test]
    fn starts_with_doctype() {
        assert!(default_page().starts_with("<!DOCTYPE html>"));
    }

    #[test]
    fn declares_english_language() {
        assert!(default_page().contains("lang=\"en\""));
    }

    #[test]
    fn sets_tab_title() {
        assert!(default_page().contains("<title>Page not found - Luca Lorenzon</title>"));
    }

    #[test]
    fn has_page_not_found_as_only_h1() {
        let html = default_page();
        assert_eq!(html.matches("<h1").count(), 1);
        assert!(html.contains(">Page not found</h1>"));
    }

    #[test]
    fn links_to_the_home_page() {
        let html = default_page();
        assert!(html.contains("<a href=\"/\">Go to the home page</a>"));
    }

    #[test]
    fn addresses_files_from_the_site_root() {
        let html = default_page();
        assert!(html.contains("href=\"/style.css\""));
        assert!(html.contains("<link rel=\"icon\" href=\"/favicon.ico\""));
    }

    #[test]
    fn addresses_files_from_the_prefix_of_the_site_address() {
        let html = render("https://host/blog/");
        assert!(html.contains("href=\"/blog/style.css\""));
        assert!(html.contains("<link rel=\"icon\" href=\"/blog/favicon.ico\""));
        assert!(html.contains("<a href=\"/blog/\">Go to the home page</a>"));
    }

    #[test]
    fn logo_links_to_the_home_page_from_any_depth() {
        assert!(default_page().contains("<a href=\"/\" aria-label=\"Lvk@73r, home page\""));
        assert!(
            render("https://host/blog/")
                .contains("<a href=\"/blog/\" aria-label=\"Lvk@73r, home page\"")
        );
    }

    #[test]
    fn has_the_same_header_and_footer_as_the_home_page() {
        let html = default_page();
        assert!(html.contains("data-testid=\"menu-slot\""));
        assert!(html.contains("aria-label=\"Lvk@73r, home page\""));
        assert!(html.contains("© 2026 Luca Lorenzon · "));
    }

    #[test]
    fn footer_says_reuse_with_attribution_and_links_to_the_licence_from_any_depth() {
        let html = default_page();
        let footer = &html[html.find("<footer").expect("footer")..];
        assert!(footer.contains(">Reuse with attribution</a>"));
        assert!(footer.contains("<a href=\"/licence.html\""));

        let html = render("https://host/blog/");
        assert!(html.contains("<a href=\"/blog/licence.html\""));
    }
}
