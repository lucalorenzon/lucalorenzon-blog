use pulldown_cmark::{Parser, html};

use crate::domain::article::Article;
use crate::domain::image_resolution::ResolvedImage;

const ABSTRACT_TRUNCATION_LIMIT: usize = 200;
const FALLBACK_IMAGE_PATH: &str = "/images/article-image-not-found.svg";

/// The article's own abstract when the author wrote one; otherwise `Body`
/// truncated to `ABSTRACT_TRUNCATION_LIMIT` characters at the last word
/// boundary, with a trailing "…". See docs/design/ssg-page-generation.md.
pub fn effective_abstract(article: &Article) -> String {
    match article.abstract_text() {
        Some(abstract_text) => abstract_text.as_str().to_string(),
        None => truncate(article.body().as_str(), ABSTRACT_TRUNCATION_LIMIT),
    }
}

fn truncate(text: &str, limit: usize) -> String {
    if text.chars().count() <= limit {
        return text.to_string();
    }
    let mut truncated: String = text.chars().take(limit).collect();
    if let Some(last_word_boundary) = truncated.rfind(char::is_whitespace) {
        truncated.truncate(last_word_boundary);
    }
    truncated.push('…');
    truncated
}

/// Where the composition root copies the content repo's `images_dir` into
/// the generated site (minimal fix, 2026-09-01) — `/images`, not
/// `/assets/images`: `cargo-leptos` flattens `assets-dir`'s contents
/// directly into `site-root` (`assets/images/x.svg` → `/images/x.svg`,
/// confirmed against a real `cargo leptos build`'s actual output, not
/// assumed), same real location `FALLBACK_IMAGE_PATH` (this app's own
/// asset) already lived at. Shares that directory with this app's own
/// brand assets — a real asset-storage design (separate location, no
/// collision risk) is its own future epic — flagged, not solved here.
const ARTICLE_IMAGES_BASE_PATH: &str = "/images";

/// The path to render for an article's image: its own path, rooted under
/// `ARTICLE_IMAGES_BASE_PATH` (an author-written path like `cover.webp` is
/// relative and would resolve wrong from a nested page like
/// `/articles/hello-world` — confirmed empirically via a real
/// `cargo leptos build`), or a fixed fallback asset (this application's
/// own, not content-repo data) when resolution fell back for any reason.
pub fn effective_image_path(resolved: &ResolvedImage) -> String {
    match resolved {
        ResolvedImage::Own(path) => format!("{ARTICLE_IMAGES_BASE_PATH}/{}", path.as_str()),
        ResolvedImage::Fallback { .. } => FALLBACK_IMAGE_PATH.to_string(),
    }
}

/// Renders an article's markdown body to an HTML string.
pub fn markdown_to_html(body: &str) -> String {
    let parser = Parser::new(body);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::article::{Article, RawFrontmatter};
    use crate::domain::value_objects::image_path::ImagePath;

    fn article_with(abstract_text: Option<&str>, body: &str) -> Article {
        Article::new(RawFrontmatter {
            date: Some("2026-08-23".to_string()),
            slug: Some("test-article".to_string()),
            tags: vec!["rust".to_string()],
            title: Some("Title".to_string()),
            abstract_text: abstract_text.map(String::from),
            image: None,
            body: Some(body.to_string()),
        })
        .expect("well-formed frontmatter should construct")
    }

    // Component: effective_abstract, AT-EP-001-UC-001-S003

    #[test]
    fn returns_the_authored_abstract_unchanged_when_present() {
        let article = article_with(Some("An abstract"), "Body content.");
        assert_eq!(effective_abstract(&article), "An abstract");
    }

    #[test]
    fn returns_the_full_body_when_absent_and_within_the_limit() {
        let article = article_with(None, "Some content.");
        assert_eq!(effective_abstract(&article), "Some content.");
    }

    #[test]
    fn truncates_at_a_word_boundary_with_an_ellipsis_when_absent_and_over_the_limit() {
        let body = "lorem ".repeat(40); // 240 chars, well over the 200-char limit
        let article = article_with(None, &body);

        let abstract_text = effective_abstract(&article);

        assert!(abstract_text.ends_with('…'));
        let without_ellipsis = abstract_text.trim_end_matches('…');
        assert!(without_ellipsis.chars().count() <= ABSTRACT_TRUNCATION_LIMIT);
        assert!(!without_ellipsis.ends_with(' '));
    }

    // Component: effective_image_path, AT-EP-001-UC-001-S003

    #[test]
    fn returns_the_resolved_path_rooted_under_the_images_base_path_when_own() {
        let path = ImagePath::parse(Some("image.webp")).unwrap();
        let resolved = ResolvedImage::Own(path);
        assert_eq!(effective_image_path(&resolved), "/images/image.webp");
    }

    #[test]
    fn returns_the_fallback_svg_when_no_image_was_ever_set() {
        let resolved = ResolvedImage::Fallback { attempted: None };
        assert_eq!(
            effective_image_path(&resolved),
            "/images/article-image-not-found.svg"
        );
    }

    #[test]
    fn returns_the_fallback_svg_when_a_referenced_image_is_missing() {
        let attempted = ImagePath::parse(Some("missing.webp"));
        let resolved = ResolvedImage::Fallback { attempted };
        assert_eq!(
            effective_image_path(&resolved),
            "/images/article-image-not-found.svg"
        );
    }

    // Component: markdown_to_html

    #[test]
    fn renders_a_heading() {
        assert_eq!(markdown_to_html("# Title"), "<h1>Title</h1>\n");
    }

    #[test]
    fn renders_a_paragraph() {
        assert_eq!(markdown_to_html("Some text."), "<p>Some text.</p>\n");
    }
}
