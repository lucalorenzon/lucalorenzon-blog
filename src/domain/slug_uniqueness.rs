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
