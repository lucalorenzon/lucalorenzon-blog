use std::collections::HashMap;

use crate::domain::article::Article;
use crate::domain::ports::{ContentSource, FetchError};
use crate::domain::value_objects::slug::Slug;

/// No-I/O test fake for `ContentSource`. Never touches the filesystem —
/// used to unit-test anything that depends on `ContentSource` without
/// needing `FilesystemContentSource`.
#[derive(Default)]
pub struct InMemoryContentSource {
    articles: HashMap<Slug, Article>,
}

impl InMemoryContentSource {
    pub fn new(articles: Vec<Article>) -> Self {
        Self {
            articles: articles
                .into_iter()
                .map(|article| (article.slug().clone(), article))
                .collect(),
        }
    }
}

impl ContentSource for InMemoryContentSource {
    fn get_by_slug(&self, slug: &Slug) -> Result<Article, FetchError> {
        self.articles.get(slug).cloned().ok_or(FetchError::NotFound)
    }

    fn list_published(&self) -> Result<Vec<Article>, FetchError> {
        Ok(self.articles.values().cloned().collect())
    }

    fn exists(&self, slug: &Slug) -> Result<bool, FetchError> {
        Ok(self.articles.contains_key(slug))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::article::RawFrontmatter;

    fn article(slug: &str) -> Article {
        Article::new(RawFrontmatter {
            date: Some("2026-08-23".to_string()),
            slug: Some(slug.to_string()),
            tags: vec!["rust".to_string()],
            title: Some("Title".to_string()),
            abstract_text: None,
            image: None,
            body: Some("Body content.".to_string()),
        })
        .expect("well-formed fixture should construct")
    }

    // Component: ContentSource — forma della porta, AC-6

    #[test]
    fn get_by_slug_returns_matching_article() {
        let source = InMemoryContentSource::new(vec![article("hello-world")]);
        let slug = Slug::parse(Some("hello-world")).unwrap();

        let found = source.get_by_slug(&slug).expect("article should be found");

        assert_eq!(found.slug().as_str(), "hello-world");
    }

    #[test]
    fn get_by_slug_returns_not_found_for_unknown_slug() {
        let source = InMemoryContentSource::new(vec![]);
        let slug = Slug::parse(Some("missing")).unwrap();

        assert!(matches!(
            source.get_by_slug(&slug),
            Err(FetchError::NotFound)
        ));
    }

    // Component: ContentSource::list_published — EP-001-UC-001-S003

    #[test]
    fn list_published_returns_all_stored_articles() {
        let source = InMemoryContentSource::new(vec![article("hello-world"), article("second")]);

        let mut slugs: Vec<String> = source
            .list_published()
            .expect("list_published should not fail")
            .iter()
            .map(|a| a.slug().as_str().to_string())
            .collect();
        slugs.sort();

        assert_eq!(slugs, vec!["hello-world", "second"]);
    }

    #[test]
    fn list_published_returns_empty_for_no_articles() {
        let source = InMemoryContentSource::new(vec![]);

        assert!(
            source
                .list_published()
                .expect("list_published should not fail")
                .is_empty()
        );
    }

    // Component: ContentSource::exists — presence check, AT-EP-001-UC-001-S002

    #[test]
    fn exists_returns_true_for_known_slug() {
        let source = InMemoryContentSource::new(vec![article("hello-world")]);
        let slug = Slug::parse(Some("hello-world")).unwrap();

        assert!(source.exists(&slug).expect("exists should not fail"));
    }

    #[test]
    fn exists_returns_false_for_unknown_slug() {
        let source = InMemoryContentSource::new(vec![]);
        let slug = Slug::parse(Some("missing")).unwrap();

        assert!(!source.exists(&slug).expect("exists should not fail"));
    }
}
