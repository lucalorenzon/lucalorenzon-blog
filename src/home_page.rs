use leptos::prelude::*;

/// The ASCII-art logo (narrow version of the old site's `AsciiArtLogo`).
const LOGO: &str = r#"    __        __   ______  __________
   / / _   __/ /__/ ____ \/__  /__  /_____
  / / | | / / //_/ / __ `/  / / /_ </ ___/
 / /__| |/ / ,< / / /_/ /  / /___/ / /
/_____/___/_/\_|\ \__,_/  /_//____/_/
                 \____/"#;

/// The whole home page document, `<html>` element included.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <html lang="it">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>"Luca Lorenzon - Home Page"</title>
                <link rel="icon" href="favicon.ico" />
                <link rel="stylesheet" href="style.css" />
            </head>
            <body>
                <div class="page-fills-screen">
                    <header class="header-two-rows xl:header-one-row">
                        <div class="text-logo-large md:not-shown">"Lvk@73r"</div>
                        <div
                            role="img"
                            aria-label="Lvk@73r"
                            class="not-shown md:art-logo-left"
                        >{LOGO}</div>
                        <h1 class="title-below-line md:title-below-line-large xl:title-beside-logo">"Home Page"</h1>
                        <div
                            data-testid="menu-slot"
                            aria-hidden="true"
                            class="reserved-menu-slot"
                        ></div>
                    </header>
                    <main class="content-centred">
                        <p>"Sito in costruzione"</p>
                    </main>
                    <footer class="footer-sticky-bottom">
                        <p>"© Luca Lorenzon 2026"</p>
                    </footer>
                </div>
            </body>
        </html>
    }
}

/// Renders the home page to a complete HTML document string.
pub fn render_home_page() -> String {
    let owner = Owner::new();
    let body = owner.with(|| view! { <HomePage /> }.to_html());
    format!("<!DOCTYPE html>{body}")
}

#[cfg(test)]
mod tests {
    use super::render_home_page;

    #[test]
    fn starts_with_doctype() {
        assert!(render_home_page().starts_with("<!DOCTYPE html>"));
    }

    #[test]
    fn declares_italian_language() {
        assert!(render_home_page().contains("lang=\"it\""));
    }

    #[test]
    fn sets_tab_title() {
        assert!(render_home_page().contains("<title>Luca Lorenzon - Home Page</title>"));
    }

    #[test]
    fn has_home_page_as_only_h1() {
        let html = render_home_page();
        assert_eq!(html.matches("<h1").count(), 1);
        assert!(html.contains(">Home Page</h1>"));
    }

    #[test]
    fn shows_under_construction_text_as_paragraph() {
        assert!(render_home_page().contains(">Sito in costruzione</p>"));
    }

    #[test]
    fn shows_copyright_in_footer() {
        let html = render_home_page();
        let footer = &html[html.find("<footer").expect("footer")..];
        assert!(footer.contains("© Luca Lorenzon 2026"));
    }

    #[test]
    fn links_favicon_with_relative_path() {
        assert!(render_home_page().contains("<link rel=\"icon\" href=\"favicon.ico\""));
    }

    #[test]
    fn logo_is_an_image_for_screen_readers() {
        let html = render_home_page();
        assert!(html.contains("role=\"img\""));
        assert!(html.contains("aria-label=\"Lvk@73r\""));
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
        let escaped = super::LOGO.replace('<', "&lt;");
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
