//! The style guide: the decisions about how the site looks, written once in Rust and
//! written out for the stylesheet by the build, and the pages of the guide.

pub mod colours;
pub mod colours_page;
pub mod index_page;

/// Where the guide's pages are written, relative to the site's root. The pages live in a
/// folder, like the articles, so their own addresses are relative to it (ADR-002).
pub const FOLDER: &str = "style-guide/";

/// ADR-001 for the guide's pages: every class of the page is an `@utility` of
/// `style/input.css` or of the colours the build writes, with or without `md:` or `xl:`.
#[cfg(test)]
fn assert_only_named_variants(html: &str) {
    let generated = colours::css(&colours::PALETTE);
    let defined: Vec<&str> = include_str!("../style/input.css")
        .lines()
        .chain(generated.lines())
        .filter_map(|line| line.strip_prefix("@utility "))
        .map(|rest| rest.trim_end_matches('{').trim())
        .collect();
    assert!(!defined.is_empty());
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
                "class `{token}` is not an @utility of style/input.css or of the colours"
            );
            used += 1;
        }
    }
    assert!(used > 0);
}
