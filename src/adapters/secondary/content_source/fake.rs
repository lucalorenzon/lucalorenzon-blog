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
        Err(FetchError::NotImplemented)
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

    #[test]
    fn list_published_is_not_yet_implemented() {
        let source = InMemoryContentSource::new(vec![]);

        assert!(matches!(
            source.list_published(),
            Err(FetchError::NotImplemented)
        ));
    }
}
