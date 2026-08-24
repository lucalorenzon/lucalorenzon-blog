use crate::domain::ports::{ContentSource, FetchError};
use crate::domain::value_objects::slug::Slug;

#[derive(Debug, thiserror::Error)]
pub enum SlugUniquenessError {
    #[error("slug already in use: {slug}")]
    AlreadyExists { slug: Slug },
    #[error("could not verify slug uniqueness: {0}")]
    CheckFailed(FetchError),
}

pub fn ensure_slug_is_unique(
    source: &impl ContentSource,
    candidate: &Slug,
) -> Result<(), SlugUniquenessError> {
    match source.exists(candidate) {
        Ok(false) => Ok(()),
        Ok(true) => Err(SlugUniquenessError::AlreadyExists {
            slug: candidate.clone(),
        }),
        Err(err) => Err(SlugUniquenessError::CheckFailed(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::secondary::content_source::fake::InMemoryContentSource;
    use crate::domain::article::{Article, RawFrontmatter};

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

    // Component: ensure_slug_is_unique — esito della verifica, AT-EP-001-UC-001-S002

    #[test]
    fn rejects_slug_already_in_use() {
        let candidate = Slug::parse(Some("hello-world")).unwrap();
        let source = InMemoryContentSource::new(vec![article("hello-world")]);

        let err = ensure_slug_is_unique(&source, &candidate).unwrap_err();

        assert!(matches!(
            err,
            SlugUniquenessError::AlreadyExists { slug } if slug == candidate
        ));
    }

    #[test]
    fn accepts_slug_not_yet_used() {
        let candidate = Slug::parse(Some("hello-world")).unwrap();
        let source = InMemoryContentSource::new(vec![]);

        assert!(ensure_slug_is_unique(&source, &candidate).is_ok());
    }
}
