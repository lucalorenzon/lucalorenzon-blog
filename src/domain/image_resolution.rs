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
