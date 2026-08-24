/// Named `article_abstract` (not `abstract`, a reserved keyword) at the file
/// and module level; the type itself is `Abstract` — `abstract` is only
/// reserved as an identifier, not as a type name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abstract(String);

impl Abstract {
    /// Normalizes rather than rejects: a blank or absent abstract is a
    /// legitimate "the author didn't write one" state, not a parse failure
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

    // Component: Abstract — normalizzazione, EP-001-UC-001-S003

    #[test]
    fn parses_non_blank_abstract() {
        let value = Abstract::parse(Some("An abstract")).unwrap();
        assert_eq!(value.as_str(), "An abstract");
    }

    #[test]
    fn normalizes_absent_to_none() {
        assert_eq!(Abstract::parse(None), None);
    }

    #[test]
    fn normalizes_blank_to_none() {
        assert_eq!(Abstract::parse(Some("   ")), None);
    }
}
