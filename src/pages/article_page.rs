use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::adapters::secondary::content_source::filesystem::FilesystemContentSource;
use crate::domain::image_resolution::resolve_image;
use crate::domain::ports::ContentSource;
use crate::domain::value_objects::slug::Slug;
use crate::layout::{ArticleAbstract, ArticleContent, ArticleTitle, Layout};
use crate::pages::view_model::{effective_abstract, effective_image_path, markdown_to_html};

/// Everything ARTICLE-PAGE needs to render, already resolved to owned,
/// presentation-ready data — no `Article`, no `ContentSource`, no Leptos
/// types. Kept separate from view construction so it's plain-Rust
/// testable, independent of the island/hydration machinery `<Layout>`
/// pulls in via `BlogHeader`.
struct ArticlePresentation {
    title: String,
    abstract_text: String,
    image_path: String,
    content_html: String,
}

/// Resolves `:slug` to a fully-formed `ArticlePresentation`, or `None` when
/// the slug is missing/malformed/unresolvable. Doesn't depend on being
/// inside a matched `<Route>` (unlike `use_params_map()`), so it's testable
/// without a real router — see the module's tests.
fn resolve_article_presentation(
    content_source: &FilesystemContentSource,
    slug_param: Option<&str>,
) -> Option<ArticlePresentation> {
    let article = slug_param
        .and_then(|raw| Slug::parse(Some(raw)).ok())
        .and_then(|slug| content_source.get_by_slug(&slug).ok())?;

    let title = article.title().as_str().to_string();
    let abstract_text = effective_abstract(&article);
    let resolved_image = resolve_image(content_source, article.image()).expect(
        "image_exists I/O failure must abort the build, not fall back silently \
         — AT-EP-001-UC-001-S003",
    );
    let image_path = effective_image_path(&resolved_image).to_string();
    let content_html = markdown_to_html(article.body().as_str());

    Some(ArticlePresentation {
        title,
        abstract_text,
        image_path,
        content_html,
    })
}

/// ARTICLE-PAGE: reads `:slug` from the route, resolves the `Article`
/// synchronously via the `ContentSource` provided in context by the
/// composition root — no `#[server]`/`Resource`, see
/// docs/architecture/hexagonal.md (Presentation, S003).
#[component]
pub fn ArticlePage() -> impl IntoView {
    let slug_param = use_params_map().get().get("slug");
    let content_source = expect_context::<FilesystemContentSource>();

    match resolve_article_presentation(&content_source, slug_param.as_deref()) {
        Some(presentation) => view! {
            <Layout>
                <ArticleTitle>{presentation.title}</ArticleTitle>
                <ArticleAbstract>{presentation.abstract_text}</ArticleAbstract>
                <ArticleContent>
                    <img src=presentation.image_path alt="" />
                    <div inner_html=presentation.content_html></div>
                </ArticleContent>
            </Layout>
        }
        .into_any(),
        // Structurally reachable (the type can't rule it out) but not
        // expected in a correct build: `prerender_params` only ever
        // generates params for slugs `list_published` actually returned.
        None => view! { <p>"Article not found."</p> }.into_any(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_content_dir(test_name: &str) -> std::path::PathBuf {
        let mut dir = std::env::temp_dir();
        dir.push(format!(
            "lucalorenzon-blog-test-article-page-{test_name}-{}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).expect("create temp content dir");
        dir
    }

    // Component: ARTICLE-PAGE / resolve_article_presentation, AT-EP-001-UC-001-S003

    #[test]
    fn resolves_the_presentation_when_the_slug_matches_a_published_article() {
        let dir = temp_content_dir("resolves_the_presentation_when_the_slug_matches_a_published_article");
        fs::write(
            dir.join("hello-world.md"),
            "---\ndate: 2026-08-23\nslug: hello-world\ntags:\n  - rust\ntitle: Hello\n---\nBody content.\n",
        )
        .unwrap();
        let source = FilesystemContentSource::new(&dir);

        let presentation = resolve_article_presentation(&source, Some("hello-world"))
            .expect("a published, well-formed slug should resolve");

        assert_eq!(presentation.title, "Hello");
        assert_eq!(presentation.content_html, "<p>Body content.</p>\n");
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn resolves_to_none_when_the_slug_does_not_match_any_article() {
        let dir = temp_content_dir("resolves_to_none_when_the_slug_does_not_match_any_article");
        let source = FilesystemContentSource::new(&dir);

        assert!(resolve_article_presentation(&source, Some("does-not-exist")).is_none());
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn resolves_to_none_when_no_slug_param_is_present() {
        let dir = temp_content_dir("resolves_to_none_when_no_slug_param_is_present");
        let source = FilesystemContentSource::new(&dir);

        assert!(resolve_article_presentation(&source, None).is_none());
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn resolves_to_none_when_the_slug_param_is_malformed() {
        let dir = temp_content_dir("resolves_to_none_when_the_slug_param_is_malformed");
        let source = FilesystemContentSource::new(&dir);

        assert!(resolve_article_presentation(&source, Some("Not A Slug!")).is_none());
        fs::remove_dir_all(&dir).ok();
    }
}
