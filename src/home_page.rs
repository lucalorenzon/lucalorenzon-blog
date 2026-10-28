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
            <body class="bg-black text-amber">
                <div class="min-h-screen flex flex-col">
                    <header class="grid grid-cols-[1fr_auto_1fr] items-center border-b border-amber">
                        <pre
                            role="img"
                            aria-label="Lvk@73r"
                            class="text-base leading-none"
                        >{LOGO}</pre>
                        <h1 class="text-[32px] leading-[32px] font-normal">"Home Page"</h1>
                        <div></div>
                    </header>
                    <main class="grow flex items-center justify-center">
                        <p>"Sito in costruzione"</p>
                    </main>
                    <footer class="sticky bottom-0 bg-black border-t border-amber text-left">
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
}
