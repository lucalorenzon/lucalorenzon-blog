use pulldown_cmark::{Parser, html};

/// Converts CommonMark text to an HTML fragment (no extensions enabled).
pub fn to_html(markdown: &str) -> String {
    let mut out = String::new();
    html::push_html(&mut out, Parser::new(markdown));
    out
}

#[cfg(test)]
mod tests {
    use super::to_html;
    use pulldown_cmark::{Event, Parser};

    #[test]
    fn converts_headings_and_lists() {
        let html = to_html("## A\n\n- b\n- c\n");
        assert!(html.contains("<h2>A</h2>"));
        assert!(html.contains("<li>b</li>"));
    }

    #[test]
    fn backslash_line_break_becomes_br() {
        assert!(to_html("a\\\nb").contains("a<br />\nb"));
    }

    #[test]
    fn inline_code_becomes_code() {
        assert!(to_html("`x`").contains("<code>x</code>"));
    }

    /// The author writes no raw HTML inside Markdown.
    #[test]
    fn curriculum_contains_no_raw_html() {
        for source in [
            include_str!("../content/curriculum.md"),
            include_str!("../content/licence.md"),
            include_str!("../content/about.md"),
            include_str!("../content/articles/after-save-event.md"),
        ] {
            let raw: Vec<Event> = Parser::new(source)
                .filter(|e| matches!(e, Event::Html(_) | Event::InlineHtml(_)))
                .collect();
            assert!(raw.is_empty(), "raw HTML found: {raw:?}");
        }
    }
}
