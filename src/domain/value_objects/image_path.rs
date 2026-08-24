#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImagePath(String);

impl ImagePath {
    /// Normalizes rather than rejects: a blank or absent image is a
    /// legitimate "the author didn't set one" state, not a parse failure
    /// — so this returns `Option`, never `Result`.
    pub fn parse(raw: Option<&str>) -> Option<Self> {
        let raw = raw.map(str::trim).filter(|s| !s.is_empty())?;
        Some(Self(raw.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Component: ImagePath — normalizzazione, EP-001-UC-001-S003

    #[test]
    fn parses_non_blank_image_path() {
        let value = ImagePath::parse(Some("image.webp")).unwrap();
        assert_eq!(value.as_str(), "image.webp");
    }

    #[test]
    fn normalizes_absent_to_none() {
        assert_eq!(ImagePath::parse(None), None);
    }

    #[test]
    fn normalizes_blank_to_none() {
        assert_eq!(ImagePath::parse(Some("  ")), None);
    }
}
