use super::slug::is_kebab_case;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag(String);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TagError {
    #[error("invalid tag {raw:?}: expected lowercase kebab-case (a-z0-9, single hyphens)")]
    Malformed { raw: String },
}

impl Tag {
    pub fn parse(raw: &str) -> Result<Self, TagError> {
        let raw = raw.trim();
        if is_kebab_case(raw) {
            Ok(Self(raw.to_string()))
        } else {
            Err(TagError::Malformed {
                raw: raw.to_string(),
            })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tags(Vec<Tag>);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TagsError {
    #[error("at least one tag is required")]
    Empty,
    #[error(transparent)]
    InvalidTag(#[from] TagError),
}

impl Tags {
    pub fn parse(raw: Vec<String>) -> Result<Self, TagsError> {
        if raw.is_empty() {
            return Err(TagsError::Empty);
        }
        let tags = raw
            .iter()
            .map(|t| Tag::parse(t))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(tags))
    }

    pub fn as_slice(&self) -> &[Tag] {
        &self.0
    }
}
