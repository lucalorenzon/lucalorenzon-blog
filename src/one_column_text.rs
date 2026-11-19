//! The text column: the `<main>` that holds the text of every page, 60 characters wide at most
//! and centred, its text 18px and from `md` 20px (the classes are in `style/input.css`).

use leptos::prelude::*;

/// The classes of the column: its look, and the larger text from `md`.
const CLASSES: &str = "long-text-column md:long-text-large";

/// The text column. It holds either the HTML made from a Markdown file (`html`) or other
/// components (`children`); the page gives one of the two.
#[component]
pub fn OneColumnText(
    /// The HTML made from a Markdown file, written as it is.
    #[prop(optional, into)]
    html: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    match html {
        Some(html) => view! { <main class=CLASSES inner_html=html></main> }.into_any(),
        None => {
            view! { <main class=CLASSES>{children.map(|children| children())}</main> }.into_any()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OneColumnText;
    use leptos::prelude::*;

    /// The classes the column must carry, written out here so that the test does not depend on
    /// the component's own constant.
    const EXPECTED_CLASSES: &str = "long-text-column md:long-text-large";

    fn render(view: impl IntoView + 'static) -> String {
        let owner = Owner::new();
        owner.with(|| view.to_html())
    }

    #[test]
    fn with_html_it_writes_the_main_with_both_classes_holding_the_html_unchanged() {
        let html = render(view! { <OneColumnText html="<h2>Title</h2><p>Text &amp; more</p>" /> });
        let expected = format!(
            "<main class=\"{EXPECTED_CLASSES}\"><h2>Title</h2><p>Text &amp; more</p></main>"
        );
        assert!(html.contains(&expected), "{html}");
    }

    #[test]
    fn with_children_it_writes_the_same_main_holding_the_children() {
        let html = render(view! { <OneColumnText><p>"Hello"</p></OneColumnText> });
        let expected = format!("<main class=\"{EXPECTED_CLASSES}\"><p>Hello</p></main>");
        assert!(html.contains(&expected), "{html}");
    }
}
