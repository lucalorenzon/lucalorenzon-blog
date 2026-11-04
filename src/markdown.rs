use pulldown_cmark::{Options, Parser, html};

/// Converts CommonMark text to an HTML fragment. Only one extension is enabled: tables.
pub fn to_html(markdown: &str) -> String {
    let mut out = String::new();
    html::push_html(&mut out, Parser::new_ext(markdown, Options::ENABLE_TABLES));
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
    fn table_becomes_table_with_header_and_data_cells() {
        let html = to_html("| a | b |\n|---|---|\n| c | d |\n");
        assert!(html.contains("<table>"));
        assert!(html.contains("<th>a</th>"));
        assert!(html.contains("<td>d</td>"));
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
            include_str!("../content/articles/how-to-generate-hybris-pk-values-without-hybris.md"),
        ] {
            let raw: Vec<Event> = Parser::new(source)
                .filter(|e| matches!(e, Event::Html(_) | Event::InlineHtml(_)))
                .collect();
            assert!(raw.is_empty(), "raw HTML found: {raw:?}");
        }
    }
}
