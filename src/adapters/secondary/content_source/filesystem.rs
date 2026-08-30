use std::path::PathBuf;

use serde::Deserialize;

use crate::domain::article::{Article, RawFrontmatter};
use crate::domain::ports::{ContentSource, FetchError};
use crate::domain::value_objects::image_path::ImagePath;
use crate::domain::value_objects::slug::Slug;

/// Reads articles from `.md` files with YAML frontmatter on the dedicated
/// content repo's checkout. Filesystem access is meaningless outside `ssr`.
pub struct FilesystemContentSource {
    articles_dir: PathBuf,
    /// `articles_dir/assets/images` — a temporary placement, confirmed by
    /// Luca 2026-08-30: article images live in this same content repo
    /// checkout, not this application's own `assets/`. See
    /// docs/design/ssg-page-generation.md.
    images_dir: PathBuf,
}

impl FilesystemContentSource {
    pub fn new(articles_dir: impl Into<PathBuf>) -> Self {
        let articles_dir = articles_dir.into();
        let images_dir = articles_dir.join("assets/images");
        Self {
            articles_dir,
            images_dir,
        }
    }
}

/// Raw shape of the YAML frontmatter block, deserialized by `yaml_serde`.
/// `abstract` is a Rust keyword, hence the rename. Any structural problem
/// here (no frontmatter block, invalid YAML, wrong field types) falls back
/// to `Default` — an all-empty `RawFrontmatter` — and is reported through
/// `Article::new`'s own validation instead of a second failure path.
#[derive(Debug, Default, Deserialize)]
struct YamlFrontmatter {
    date: Option<String>,
    slug: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    title: Option<String>,
    #[serde(rename = "abstract")]
    abstract_text: Option<String>,
    image: Option<String>,
}

fn with_body(yaml: YamlFrontmatter, body: Option<String>) -> RawFrontmatter {
    RawFrontmatter {
        date: yaml.date,
        slug: yaml.slug,
        tags: yaml.tags,
        title: yaml.title,
        abstract_text: yaml.abstract_text,
        image: yaml.image,
        body,
    }
}

/// Extracts the YAML block between the opening and closing `---` fences.
/// Returns "" for anything that doesn't match (no leading fence, no closing
/// fence) — `yaml_serde` then fails on an empty document, which is caught
/// the same way as any other malformed frontmatter.
fn extract_yaml_block(content: &str) -> &str {
    let Some(rest) = content.strip_prefix("---") else {
        return "";
    };
    let rest = rest
        .strip_prefix("\r\n")
        .or_else(|| rest.strip_prefix('\n'))
        .unwrap_or(rest);

    match rest.find("\n---") {
        Some(end) => &rest[..end],
        None => "",
    }
}

/// Extracts the article body: everything after the closing `---` fence of
/// the YAML block. `None` for anything that doesn't have a well-formed
/// closing fence — same "let `Article::new` report it" strategy as
/// `extract_yaml_block`, via `Body::parse(None)` → `BodyError::Missing`.
fn extract_body(content: &str) -> Option<&str> {
    let rest = content.strip_prefix("---")?;
    let rest = rest
        .strip_prefix("\r\n")
        .or_else(|| rest.strip_prefix('\n'))
        .unwrap_or(rest);

    let end = rest.find("\n---")?;
    let after_fence = &rest[end + "\n---".len()..];
    let after_fence = after_fence
        .strip_prefix("\r\n")
        .or_else(|| after_fence.strip_prefix('\n'))
        .unwrap_or(after_fence);

    Some(after_fence)
}

impl ContentSource for FilesystemContentSource {
    fn get_by_slug(&self, slug: &Slug) -> Result<Article, FetchError> {
        let path = self.articles_dir.join(format!("{slug}.md"));
        let content = std::fs::read_to_string(&path).map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => FetchError::NotFound,
            _ => FetchError::Io(err),
        })?;

        let yaml_block = extract_yaml_block(&content);
        let frontmatter: YamlFrontmatter = yaml_serde::from_str(yaml_block).unwrap_or_default();
        let body = extract_body(&content).map(str::to_string);

        Article::new(with_body(frontmatter, body)).map_err(FetchError::Malformed)
    }

    fn list_published(&self) -> Result<Vec<Article>, FetchError> {
        let entries = std::fs::read_dir(&self.articles_dir)?;
        let mut articles = Vec::new();

        for entry in entries {
            let path = entry?.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let Ok(slug) = Slug::parse(Some(stem)) else {
                continue;
            };
            articles.push(self.get_by_slug(&slug)?);
        }

        Ok(articles)
    }

    fn exists(&self, slug: &Slug) -> Result<bool, FetchError> {
        let path = self.articles_dir.join(format!("{slug}.md"));
        match std::fs::metadata(&path) {
            Ok(_) => Ok(true),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(err) => Err(FetchError::Io(err)),
        }
    }

    fn image_exists(&self, image: &ImagePath) -> Result<bool, FetchError> {
        let path = self.images_dir.join(image.as_str());
        match std::fs::metadata(&path) {
            Ok(_) => Ok(true),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(err) => Err(FetchError::Io(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_content_dir(test_name: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        dir.push(format!(
            "lucalorenzon-blog-test-{test_name}-{}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).expect("create temp content dir");
        dir
    }

    // Component: ContentSource — forma della porta, AC-6 (get_by_slug, via l'adapter reale)

    #[test]
    fn reads_well_formed_article() {
        let dir = temp_content_dir("reads_well_formed_article");
        fs::write(
            dir.join("hello-world.md"),
            "---\ndate: 2026-08-23\nslug: hello-world\ntags:\n  - rust\ntitle: Hello\n---\nBody.\n",
        )
        .unwrap();

        let source = FilesystemContentSource::new(&dir);
        let slug = Slug::parse(Some("hello-world")).unwrap();

        let article = source
            .get_by_slug(&slug)
            .expect("should read a well-formed article");

        assert_eq!(article.slug().as_str(), "hello-world");
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn returns_not_found_for_missing_file() {
        let dir = temp_content_dir("returns_not_found_for_missing_file");
        let source = FilesystemContentSource::new(&dir);
        let slug = Slug::parse(Some("does-not-exist")).unwrap();

        let err = source.get_by_slug(&slug).unwrap_err();

        assert!(matches!(err, FetchError::NotFound));
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn extract_yaml_block_returns_empty_when_no_leading_fence() {
        assert_eq!(extract_yaml_block("no frontmatter here\n"), "");
    }

    #[test]
    fn extract_yaml_block_returns_empty_when_no_closing_fence() {
        assert_eq!(extract_yaml_block("---\ndate: 2026-08-23\n"), "");
    }

    #[test]
    fn extract_body_returns_content_after_closing_fence() {
        assert_eq!(
            extract_body("---\ndate: 2026-08-23\n---\nBody.\n"),
            Some("Body.\n")
        );
    }

    #[test]
    fn extract_body_returns_none_when_no_leading_fence() {
        assert_eq!(extract_body("no frontmatter here\n"), None);
    }

    #[test]
    fn extract_body_returns_none_when_no_closing_fence() {
        assert_eq!(extract_body("---\ndate: 2026-08-23\n"), None);
    }

    #[test]
    fn reads_body_after_frontmatter() {
        let dir = temp_content_dir("reads_body_after_frontmatter");
        fs::write(
            dir.join("hello-world.md"),
            "---\ndate: 2026-08-23\nslug: hello-world\ntags:\n  - rust\ntitle: Hello\n---\nBody.\n",
        )
        .unwrap();

        let source = FilesystemContentSource::new(&dir);
        let slug = Slug::parse(Some("hello-world")).unwrap();

        let article = source
            .get_by_slug(&slug)
            .expect("should read a well-formed article");

        assert_eq!(article.body().as_str(), "Body.");
        fs::remove_dir_all(&dir).ok();
    }

    // Component: ContentSource::list_published — EP-001-UC-001-S003

    #[test]
    fn list_published_returns_every_article_in_the_directory() {
        let dir = temp_content_dir("list_published_returns_every_article_in_the_directory");
        fs::write(
            dir.join("hello-world.md"),
            "---\ndate: 2026-08-23\nslug: hello-world\ntags:\n  - rust\ntitle: Hello\n---\nBody.\n",
        )
        .unwrap();
        fs::write(
            dir.join("second.md"),
            "---\ndate: 2026-08-22\nslug: second\ntags:\n  - rust\ntitle: Second\n---\nBody two.\n",
        )
        .unwrap();

        let source = FilesystemContentSource::new(&dir);

        let mut slugs: Vec<String> = source
            .list_published()
            .expect("list_published should not fail")
            .iter()
            .map(|a| a.slug().as_str().to_string())
            .collect();
        slugs.sort();

        assert_eq!(slugs, vec!["hello-world", "second"]);
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn list_published_returns_empty_for_empty_directory() {
        let dir = temp_content_dir("list_published_returns_empty_for_empty_directory");
        let source = FilesystemContentSource::new(&dir);

        assert!(
            source
                .list_published()
                .expect("list_published should not fail")
                .is_empty()
        );
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn list_published_propagates_malformed_article() {
        let dir = temp_content_dir("list_published_propagates_malformed_article");
        fs::write(dir.join("broken.md"), "no frontmatter here\n").unwrap();

        let source = FilesystemContentSource::new(&dir);

        assert!(matches!(
            source.list_published(),
            Err(FetchError::Malformed(_))
        ));
        fs::remove_dir_all(&dir).ok();
    }

    // Component: ContentSource::exists — presence check, AT-EP-001-UC-001-S002

    #[test]
    fn exists_returns_true_for_well_formed_article() {
        let dir = temp_content_dir("exists_returns_true_for_well_formed_article");
        fs::write(
            dir.join("hello-world.md"),
            "---\ndate: 2026-08-23\nslug: hello-world\ntags:\n  - rust\ntitle: Hello\n---\nBody.\n",
        )
        .unwrap();

        let source = FilesystemContentSource::new(&dir);
        let slug = Slug::parse(Some("hello-world")).unwrap();

        assert!(source.exists(&slug).expect("exists should not fail"));
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn exists_returns_true_for_malformed_article() {
        let dir = temp_content_dir("exists_returns_true_for_malformed_article");
        fs::write(dir.join("broken.md"), "no frontmatter here\n").unwrap();

        let source = FilesystemContentSource::new(&dir);
        let slug = Slug::parse(Some("broken")).unwrap();

        assert!(source.exists(&slug).expect("exists should not fail"));
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn exists_returns_false_for_missing_file() {
        let dir = temp_content_dir("exists_returns_false_for_missing_file");
        let source = FilesystemContentSource::new(&dir);
        let slug = Slug::parse(Some("does-not-exist")).unwrap();

        assert!(!source.exists(&slug).expect("exists should not fail"));
        fs::remove_dir_all(&dir).ok();
    }

    // Component: ContentSource::image_exists — residuality extension, EP-001-UC-001-S003

    #[test]
    fn image_exists_returns_true_for_a_file_present_under_images_dir() {
        let dir = temp_content_dir("image_exists_returns_true_for_a_file_present_under_images_dir");
        let images_dir = dir.join("assets/images");
        fs::create_dir_all(&images_dir).unwrap();
        fs::write(images_dir.join("cover.webp"), b"fake image bytes").unwrap();

        let source = FilesystemContentSource::new(&dir);
        let image = ImagePath::parse(Some("cover.webp")).unwrap();

        assert!(source.image_exists(&image).expect("image_exists should not fail"));
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn image_exists_returns_false_for_a_missing_file() {
        let dir = temp_content_dir("image_exists_returns_false_for_a_missing_file");
        let source = FilesystemContentSource::new(&dir);
        let image = ImagePath::parse(Some("does-not-exist.webp")).unwrap();

        assert!(!source.image_exists(&image).expect("image_exists should not fail"));
        fs::remove_dir_all(&dir).ok();
    }
}
