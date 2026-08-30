use crate::domain::ports::{ContentSource, FetchError};
use crate::domain::value_objects::image_path::ImagePath;

/// The outcome of resolving an `Article`'s image against the content
/// repo's actual files. No error variant for a missing file — a
/// referenced-but-absent image is a legitimate outcome, not a construction
/// failure. Carries no brand knowledge (the fallback asset path is a
/// presentation constant, `src/pages/view_model.rs`). See
/// docs/design/ssg-page-generation.md.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedImage {
    Own(ImagePath),
    Fallback { attempted: Option<ImagePath> },
}

/// Resolves an `Article`'s image against the content repo's actual files,
/// mediated by `ContentSource::image_exists` — never raw I/O directly, same
/// discipline as `ensure_slug_is_unique`. `None` (no image was ever set)
/// never touches the port. A missing file is not an error, it becomes
/// `Fallback`; a genuine I/O failure still propagates, not swallowed.
/// [S003, residuality extension — corrected 2026-08-31, see
/// docs/architecture/hexagonal.md]
pub fn resolve_image(
    source: &impl ContentSource,
    image: Option<&ImagePath>,
) -> Result<ResolvedImage, FetchError> {
    let Some(path) = image else {
        return Ok(ResolvedImage::Fallback { attempted: None });
    };
    if source.image_exists(path)? {
        Ok(ResolvedImage::Own(path.clone()))
    } else {
        Ok(ResolvedImage::Fallback {
            attempted: Some(path.clone()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::secondary::content_source::fake::InMemoryContentSource;

    // Component: resolve_image, AT-EP-001-UC-001-S003

    #[test]
    fn falls_back_with_no_attempt_when_no_image_was_ever_set() {
        let source = InMemoryContentSource::new(vec![]);

        let resolved = resolve_image(&source, None).expect("resolution should not fail");

        assert_eq!(resolved, ResolvedImage::Fallback { attempted: None });
    }

    #[test]
    fn resolves_to_own_when_the_referenced_image_exists() {
        let source = InMemoryContentSource::new(vec![]).with_existing_image("cover.webp");
        let image = ImagePath::parse(Some("cover.webp")).unwrap();

        let resolved = resolve_image(&source, Some(&image)).expect("resolution should not fail");

        assert_eq!(resolved, ResolvedImage::Own(image));
    }

    #[test]
    fn falls_back_with_the_attempted_path_when_the_referenced_image_is_missing() {
        let source = InMemoryContentSource::new(vec![]);
        let image = ImagePath::parse(Some("missing.webp")).unwrap();

        let resolved = resolve_image(&source, Some(&image)).expect("resolution should not fail");

        assert_eq!(
            resolved,
            ResolvedImage::Fallback {
                attempted: Some(image)
            }
        );
    }
}
