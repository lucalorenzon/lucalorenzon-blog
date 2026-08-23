#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Slug(String);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SlugError {
    #[error("slug is missing")]
    Missing,
    #[error("invalid slug {raw:?}: expected lowercase kebab-case (a-z0-9, single hyphens)")]
    Malformed { raw: String },
}

impl Slug {
    pub fn parse(raw: Option<&str>) -> Result<Self, SlugError> {
        let raw = raw
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or(SlugError::Missing)?;

        if is_kebab_case(raw) {
            Ok(Self(raw.to_string()))
        } else {
            Err(SlugError::Malformed { raw: raw.to_string() })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Shared by Slug and Tag: lowercase ASCII kebab-case,
/// no leading/trailing/doubled hyphens.
pub(crate) fn is_kebab_case(s: &str) -> bool {
    !s.is_empty()
        && s.split('-').all(|segment| {
            !segment.is_empty() && segment.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        })
}

impl std::fmt::Display for Slug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
