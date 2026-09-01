use leptos::prelude::*;
use leptos_meta::Title;

use crate::adapters::secondary::content_source::filesystem::FilesystemContentSource;
use crate::domain::article::Article;
use crate::domain::image_resolution::resolve_image;
use crate::domain::ports::ContentSource;
use crate::layout::ListLayout;
use crate::pages::view_model::{effective_abstract, effective_image_path};

/// One published article, resolved to owned, presentation-ready data — no
/// `Article`, no `ContentSource`, no Leptos types. Kept separate from view
/// construction so it's plain-Rust testable, independent of the
/// island/hydration machinery `<ListLayout>` pulls in via `BlogHeader`.
struct ListingEntry {
    href: String,
    image_path: String,
    title: String,
    abstract_text: String,
}

/// All published articles, resolved and in chronological order (most
/// recent first, deterministic tie-break on `Slug`) — same policy as the
/// composition root's own sort, duplicated here rather than shared because
/// the composition root does not (yet) pass a pre-sorted list down; see
/// docs/architecture/hexagonal.md. Generic over `ContentSource`, not
/// `FilesystemContentSource` — same reasoning as
/// `resolve_article_presentation` in `article_page.rs`.
fn resolve_listing_entries(source: &impl ContentSource) -> Vec<ListingEntry> {
    let mut articles: Vec<Article> = source.list_published().expect(
        "a malformed article must abort the build, not silently exclude it \
         — AT-EP-001-UC-001-S003",
    );
    articles.sort_by(|a, b| {
        a.date()
            .cmp(b.date())
            .reverse()
            .then_with(|| a.slug().as_str().cmp(b.slug().as_str()))
    });

    articles
        .iter()
        .map(|article| {
            let slug = article.slug().as_str().to_string();
            let title = article.title().as_str().to_string();
            let abstract_text = effective_abstract(article);
            let resolved_image = resolve_image(source, article.image()).expect(
                "image_exists I/O failure must abort the build, not fall back silently \
                 — AT-EP-001-UC-001-S003",
            );
            let image_path = effective_image_path(&resolved_image).to_string();
            let href = format!("/articles/{slug}");
            ListingEntry {
                href,
                image_path,
                title,
                abstract_text,
            }
        })
        .collect()
}

fn entry_view(entry: ListingEntry) -> impl IntoView {
    view! {
        <li>
            <a href=entry.href>
                <img src=entry.image_path alt="" />
                <h2>{entry.title}</h2>
                <p>{entry.abstract_text}</p>
            </a>
        </li>
    }
}

/// LISTING-PAGE: full list of published articles, most recent first.
/// Route is `/articles/page/:page` (reserved for future pagination, see
/// story's Open questions) — `:page` is not yet read, every build of this
/// story shows the full list regardless of its value.
#[component]
pub fn ListingPage() -> impl IntoView {
    let content_source = expect_context::<FilesystemContentSource>();
    let entries: Vec<_> = resolve_listing_entries(&content_source)
        .into_iter()
        .map(entry_view)
        .collect();

    view! {
        <Title text="Lvk@73r Blog"/>
        <ListLayout>
            <ul>{entries}</ul>
        </ListLayout>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn temp_content_dir(test_name: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        dir.push(format!(
            "lucalorenzon-blog-test-listing-page-{test_name}-{}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).expect("create temp content dir");
        dir
    }

    fn write_article(dir: &std::path::Path, slug: &str, date: &str) {
        fs::write(
            dir.join(format!("{slug}.md")),
            format!("---\ndate: {date}\nslug: {slug}\ntags:\n  - rust\ntitle: {slug}\n---\nBody.\n"),
        )
        .unwrap();
    }

    // Component: LISTING-PAGE / resolve_listing_entries, AT-EP-001-UC-001-S003

    #[test]
    fn resolves_an_entry_per_published_article_in_chronological_order() {
        let dir = temp_content_dir("resolves_an_entry_per_published_article_in_chronological_order");
        write_article(&dir, "older", "2026-08-20");
        write_article(&dir, "newer", "2026-08-25");
        let source = FilesystemContentSource::new(&dir);

        let entries = resolve_listing_entries(&source);

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].href, "/articles/newer", "most recent article listed first");
        assert_eq!(entries[1].href, "/articles/older");

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn resolves_to_an_empty_list_when_nothing_is_published() {
        let dir = temp_content_dir("resolves_to_an_empty_list_when_nothing_is_published");
        let source = FilesystemContentSource::new(&dir);

        assert!(resolve_listing_entries(&source).is_empty());

        fs::remove_dir_all(&dir).ok();
    }
}
