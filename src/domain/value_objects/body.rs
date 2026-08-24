#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Body(String);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BodyError {
    #[error("article body is missing or empty")]
    Missing,
}

impl Body {
    pub fn parse(raw: Option<&str>) -> Result<Self, BodyError> {
        let raw = raw
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or(BodyError::Missing)?;

        Ok(Self(raw.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Component: Body — costruzione, EP-001-UC-001-S003

    #[test]
    fn parses_non_blank_body() {
        let body = Body::parse(Some("Some content.\n\nMore content.")).unwrap();
        assert_eq!(body.as_str(), "Some content.\n\nMore content.");
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let body = Body::parse(Some("  \nContent.\n  ")).unwrap();
        assert_eq!(body.as_str(), "Content.");
    }

    #[test]
    fn rejects_missing_body() {
        assert_eq!(Body::parse(None).unwrap_err(), BodyError::Missing);
    }

    #[test]
    fn rejects_blank_body() {
        assert_eq!(
            Body::parse(Some("   \n  ")).unwrap_err(),
            BodyError::Missing
        );
    }
}
