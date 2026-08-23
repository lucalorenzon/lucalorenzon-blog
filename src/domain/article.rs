use crate::domain::value_objects::{
    publication_date::{PublicationDate, PublicationDateError},
    slug::{Slug, SlugError},
    tag::{Tags, TagsError},
    title::{Title, TitleError},
};

pub struct RawFrontmatter {
    pub date: Option<String>,
    pub slug: Option<String>,
    pub tags: Vec<String>,
    pub title: Option<String>,
    pub abstract_text: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ArticleError {
    #[error("date: {0}")]
    Date(#[from] PublicationDateError),
    #[error("slug: {0}")]
    Slug(#[from] SlugError),
    #[error("tags: {0}")]
    Tags(#[from] TagsError),
    #[error("title: {0}")]
    Title(#[from] TitleError),
}

#[derive(Debug)]
pub struct Article {
    date: PublicationDate,
    slug: Slug,
    tags: Tags,
    title: Title,
    abstract_text: Option<String>,
    image: Option<String>,
}

impl Article {
    pub fn new(raw: RawFrontmatter) -> Result<Self, ArticleError> {
        let date = PublicationDate::parse(raw.date.as_deref())?;
        let slug = Slug::parse(raw.slug.as_deref())?;
        let tags = Tags::parse(raw.tags)?;
        let title = Title::parse(raw.title.as_deref())?;

        Ok(Self {
            date,
            slug,
            tags,
            title,
            abstract_text: raw.abstract_text,
            image: raw.image,
        })
    }

    pub fn slug(&self) -> &Slug {
        &self.slug
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw(
        date: Option<&str>,
        slug: Option<&str>,
        tags: Vec<&str>,
        title: Option<&str>,
        abstract_text: Option<&str>,
        image: Option<&str>,
    ) -> RawFrontmatter {
        RawFrontmatter {
            date: date.map(String::from),
            slug: slug.map(String::from),
            tags: tags.into_iter().map(String::from).collect(),
            title: title.map(String::from),
            abstract_text: abstract_text.map(String::from),
            image: image.map(String::from),
        }
    }

    // Component: Article — costruzione da frontespizio (happy path), AC-1

    #[test]
    fn constructs_with_abstract_and_image_present() {
        let article = Article::new(raw(
            Some("2026-08-23"),
            Some("valid-slug"),
            vec!["rust"],
            Some("Title"),
            Some("An abstract"),
            Some("image.webp"),
        ))
        .expect("well-formed frontmatter should construct");

        assert_eq!(article.abstract_text, Some("An abstract".to_string()));
        assert_eq!(article.image, Some("image.webp".to_string()));
    }

    #[test]
    fn constructs_with_abstract_absent() {
        let article = Article::new(raw(
            Some("2026-08-23"),
            Some("valid-slug"),
            vec!["rust"],
            Some("Title"),
            None,
            Some("image.webp"),
        ))
        .expect("well-formed frontmatter should construct");

        assert_eq!(article.abstract_text, None);
        assert_eq!(article.image, Some("image.webp".to_string()));
    }

    #[test]
    fn constructs_with_image_absent() {
        let article = Article::new(raw(
            Some("2026-08-23"),
            Some("valid-slug"),
            vec!["rust"],
            Some("Title"),
            Some("An abstract"),
            None,
        ))
        .expect("well-formed frontmatter should construct");

        assert_eq!(article.abstract_text, Some("An abstract".to_string()));
        assert_eq!(article.image, None);
    }

    // Component: Article — costruzione rifiutata (campo obbligatorio assente), AC-2..AC-5

    #[test]
    fn rejects_missing_date() {
        let err = Article::new(raw(None, Some("valid-slug"), vec!["rust"], Some("Title"), None, None))
            .unwrap_err();
        assert!(matches!(err, ArticleError::Date(_)));
    }

    #[test]
    fn rejects_missing_slug() {
        let err = Article::new(raw(Some("2026-08-23"), None, vec!["rust"], Some("Title"), None, None))
            .unwrap_err();
        assert!(matches!(err, ArticleError::Slug(_)));
    }

    #[test]
    fn rejects_missing_tags() {
        let err = Article::new(raw(Some("2026-08-23"), Some("valid-slug"), vec![], Some("Title"), None, None))
            .unwrap_err();
        assert!(matches!(err, ArticleError::Tags(_)));
    }

    #[test]
    fn rejects_missing_title() {
        let err = Article::new(raw(Some("2026-08-23"), Some("valid-slug"), vec!["rust"], None, None, None))
            .unwrap_err();
        assert!(matches!(err, ArticleError::Title(_)));
    }

    // Component: Article — costruzione rifiutata (campo obbligatorio malformato), AC-2..AC-5

    #[test]
    fn rejects_malformed_date() {
        let err = Article::new(raw(
            Some("2026-02-30"),
            Some("valid-slug"),
            vec!["rust"],
            Some("Title"),
            None,
            None,
        ))
        .unwrap_err();
        assert!(matches!(err, ArticleError::Date(_)));
    }

    #[test]
    fn rejects_malformed_slug() {
        let err = Article::new(raw(
            Some("2026-08-23"),
            Some("Il Mio Slug!"),
            vec!["rust"],
            Some("Title"),
            None,
            None,
        ))
        .unwrap_err();
        assert!(matches!(err, ArticleError::Slug(_)));
    }

    #[test]
    fn rejects_malformed_tag() {
        let err = Article::new(raw(
            Some("2026-08-23"),
            Some("valid-slug"),
            vec!["Rust Web"],
            Some("Title"),
            None,
            None,
        ))
        .unwrap_err();
        assert!(matches!(err, ArticleError::Tags(_)));
    }

    #[test]
    fn rejects_malformed_title() {
        let err = Article::new(raw(
            Some("2026-08-23"),
            Some("valid-slug"),
            vec!["rust"],
            Some("Bad\nTitle"),
            None,
            None,
        ))
        .unwrap_err();
        assert!(matches!(err, ArticleError::Title(_)));
    }
}
