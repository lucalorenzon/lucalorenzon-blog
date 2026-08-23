use crate::domain::article::{Article, ArticleError};
use crate::domain::value_objects::slug::Slug;

#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    #[error("article not found")]
    NotFound,
    #[error("I/O error reading content source: {0}")]
    Io(#[from] std::io::Error),
    #[error("malformed article: {0}")]
    Malformed(#[from] ArticleError),
    #[error("operation not yet implemented")]
    NotImplemented,
}

pub trait ContentSource {
    fn get_by_slug(&self, slug: &Slug) -> Result<Article, FetchError>;
    fn list_published(&self) -> Result<Vec<Article>, FetchError>;
}
