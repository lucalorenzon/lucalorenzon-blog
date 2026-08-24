use crate::domain::value_objects::{
    article_abstract::Abstract,
    body::{Body, BodyError},
    image_path::ImagePath,
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
    pub body: Option<String>,
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
    #[error("body: {0}")]
    Body(#[from] BodyError),
}

#[derive(Debug, Clone)]
pub struct Article {
    date: PublicationDate,
    slug: Slug,
    tags: Tags,
    title: Title,
    abstract_text: Option<Abstract>,
    image: Option<ImagePath>,
    body: Body,
}

impl Article {
    pub fn new(raw: RawFrontmatter) -> Result<Self, ArticleError> {
        let date = PublicationDate::parse(raw.date.as_deref())?;
        let slug = Slug::parse(raw.slug.as_deref())?;
        let tags = Tags::parse(raw.tags)?;
        let title = Title::parse(raw.title.as_deref())?;
        let body = Body::parse(raw.body.as_deref())?;

        Ok(Self {
            date,
            slug,
            tags,
            title,
            abstract_text: Abstract::parse(raw.abstract_text.as_deref()),
            image: ImagePath::parse(raw.image.as_deref()),
            body,
        })
    }

    pub fn slug(&self) -> &Slug {
        &self.slug
    }

    pub fn date(&self) -> &PublicationDate {
        &self.date
    }

    pub fn abstract_text(&self) -> Option<&Abstract> {
        self.abstract_text.as_ref()
    }

    pub fn image(&self) -> Option<&ImagePath> {
        self.image.as_ref()
    }

    pub fn body(&self) -> &Body {
        &self.body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::too_many_arguments)]
    fn raw(
        date: Option<&str>,
        slug: Option<&str>,
        tags: Vec<&str>,
        title: Option<&str>,
        abstract_text: Option<&str>,
        image: Option<&str>,
        body: Option<&str>,
    ) -> RawFrontmatter {
        RawFrontmatter {
            date: date.map(String::from),
            slug: slug.map(String::from),
            tags: tags.into_iter().map(String::from).collect(),
            title: title.map(String::from),
            abstract_text: abstract_text.map(String::from),
            image: image.map(String::from),
            body: body.map(String::from),
        }
    }

    /// Same as `raw`, with a well-formed default body — for tests whose
    /// point is a different field.
    fn raw_with_body(
        date: Option<&str>,
        slug: Option<&str>,
        tags: Vec<&str>,
        title: Option<&str>,
        abstract_text: Option<&str>,
        image: Option<&str>,
    ) -> RawFrontmatter {
        raw(
            date,
            slug,
            tags,
            title,
            abstract_text,
            image,
            Some("Body content."),
        )
    }

    // Component: Article — costruzione da frontespizio (happy path), AC-1

    #[test]
    fn constructs_with_abstract_and_image_present() {
        let article = Article::new(raw_with_body(
            Some("2026-08-23"),
            Some("valid-slug"),
            vec!["rust"],
            Some("Title"),
            Some("An abstract"),
            Some("image.webp"),
        ))
        .expect("well-formed frontmatter should construct");

        assert_eq!(
            article.abstract_text().map(Abstract::as_str),
            Some("An abstract")
        );
        assert_eq!(article.image().map(ImagePath::as_str), Some("image.webp"));
        assert_eq!(article.body().as_str(), "Body content.");
    }

    #[test]
    fn constructs_with_abstract_absent() {
        let article = Article::new(raw_with_body(
            Some("2026-08-23"),
            Some("valid-slug"),
            vec!["rust"],
            Some("Title"),
            None,
            Some("image.webp"),
        ))
        .expect("well-formed frontmatter should construct");

        assert_eq!(article.abstract_text(), None);
        assert_eq!(article.image().map(ImagePath::as_str), Some("image.webp"));
    }

    #[test]
    fn constructs_with_image_absent() {
        let article = Article::new(raw_with_body(
            Some("2026-08-23"),
            Some("valid-slug"),
            vec!["rust"],
            Some("Title"),
            Some("An abstract"),
            None,
        ))
        .expect("well-formed frontmatter should construct");

        assert_eq!(
            article.abstract_text().map(Abstract::as_str),
            Some("An abstract")
        );
        assert_eq!(article.image(), None);
    }

    // Component: Article — costruzione rifiutata (campo obbligatorio assente), AC-2..AC-5

    #[test]
    fn rejects_missing_date() {
        let err = Article::new(raw_with_body(
            None,
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
    fn rejects_missing_slug() {
        let err = Article::new(raw_with_body(
            Some("2026-08-23"),
            None,
            vec!["rust"],
            Some("Title"),
            None,
            None,
        ))
        .unwrap_err();
        assert!(matches!(err, ArticleError::Slug(_)));
    }

    #[test]
    fn rejects_missing_tags() {
        let err = Article::new(raw_with_body(
            Some("2026-08-23"),
            Some("valid-slug"),
            vec![],
            Some("Title"),
            None,
            None,
        ))
        .unwrap_err();
        assert!(matches!(err, ArticleError::Tags(_)));
    }

    #[test]
    fn rejects_missing_title() {
        let err = Article::new(raw_with_body(
            Some("2026-08-23"),
            Some("valid-slug"),
            vec!["rust"],
            None,
            None,
            None,
        ))
        .unwrap_err();
        assert!(matches!(err, ArticleError::Title(_)));
    }

    #[test]
    fn rejects_missing_body() {
        let err = Article::new(raw(
            Some("2026-08-23"),
            Some("valid-slug"),
            vec!["rust"],
            Some("Title"),
            None,
            None,
            None,
        ))
        .unwrap_err();
        assert!(matches!(err, ArticleError::Body(_)));
    }

    // Component: Article — costruzione rifiutata (campo obbligatorio malformato), AC-2..AC-5

    #[test]
    fn rejects_malformed_date() {
        let err = Article::new(raw_with_body(
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
        let err = Article::new(raw_with_body(
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
        let err = Article::new(raw_with_body(
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
        let err = Article::new(raw_with_body(
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
