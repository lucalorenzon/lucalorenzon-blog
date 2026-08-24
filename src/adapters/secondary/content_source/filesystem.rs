use std::path::PathBuf;

use serde::Deserialize;

use crate::domain::article::{Article, RawFrontmatter};
use crate::domain::ports::{ContentSource, FetchError};
use crate::domain::value_objects::slug::Slug;

/// Reads articles from `.md` files with YAML frontmatter on the dedicated
/// content repo's checkout. Filesystem access is meaningless outside `ssr`.
pub struct FilesystemContentSource {
    articles_dir: PathBuf,
}

impl FilesystemContentSource {
    pub fn new(articles_dir: impl Into<PathBuf>) -> Self {
        Self {
            articles_dir: articles_dir.into(),
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

impl From<YamlFrontmatter> for RawFrontmatter {
    fn from(yaml: YamlFrontmatter) -> Self {
        RawFrontmatter {
            date: yaml.date,
            slug: yaml.slug,
            tags: yaml.tags,
            title: yaml.title,
            abstract_text: yaml.abstract_text,
            image: yaml.image,
        }
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

impl ContentSource for FilesystemContentSource {
    fn get_by_slug(&self, slug: &Slug) -> Result<Article, FetchError> {
        let path = self.articles_dir.join(format!("{slug}.md"));
        let content = std::fs::read_to_string(&path).map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => FetchError::NotFound,
            _ => FetchError::Io(err),
        })?;

        let yaml_block = extract_yaml_block(&content);
        let frontmatter: YamlFrontmatter = yaml_serde::from_str(yaml_block).unwrap_or_default();

        Article::new(frontmatter.into()).map_err(FetchError::Malformed)
    }

    fn list_published(&self) -> Result<Vec<Article>, FetchError> {
        Err(FetchError::NotImplemented)
    }

    fn exists(&self, slug: &Slug) -> Result<bool, FetchError> {
        let path = self.articles_dir.join(format!("{slug}.md"));
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
    fn list_published_is_not_yet_implemented() {
        let dir = temp_content_dir("list_published_is_not_yet_implemented");
        let source = FilesystemContentSource::new(&dir);

        assert!(matches!(
            source.list_published(),
            Err(FetchError::NotImplemented)
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
}
