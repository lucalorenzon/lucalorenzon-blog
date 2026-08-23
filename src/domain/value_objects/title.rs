#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Title(String);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TitleError {
    #[error("title is missing")]
    Missing,
    #[error("invalid title {raw:?}: must be non-blank, single-line text")]
    Malformed { raw: String },
}

impl Title {
    pub fn parse(raw: Option<&str>) -> Result<Self, TitleError> {
        let raw = raw
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or(TitleError::Missing)?;

        if raw.chars().any(|c| c.is_control()) {
            return Err(TitleError::Malformed { raw: raw.to_string() });
        }
        Ok(Self(raw.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
