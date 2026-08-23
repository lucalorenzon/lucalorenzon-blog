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
