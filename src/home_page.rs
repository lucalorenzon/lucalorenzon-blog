use leptos::prelude::*;

/// The whole home page document, `<html>` element included.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <html lang="it">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>"Luca Lorenzon"</title>
                <link rel="stylesheet" href="style.css" />
            </head>
            <body class="bg-slate-50 text-slate-900">
                <main class="min-h-screen flex items-center justify-center">
                    <h1 class="text-4xl font-semibold tracking-tight">"Sito in costruzione"</h1>
                </main>
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
    fn shows_under_construction_text() {
        assert!(render_home_page().contains("Sito in costruzione"));
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
