/// The site's full address, such as `https://www.lucalorenzon.com/`.
///
/// Built only through [`SiteAddress::parse`], so a value of this type is
/// already known to be a full `https` address with a host and a final `/`
/// (ADR-002). It keeps the full text, for the addresses that must be complete
/// (the canonical link), and its path part, for the pages served at any depth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiteAddress {
    full: String,
    path: String,
}

/// Why a text was refused as the site's address.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error(
    "invalid site address `{0}`: it must start with `https://`, have a host and end with `/` (for example `https://www.lucalorenzon.com/`)"
)]
pub struct InvalidSiteAddress(String);

impl SiteAddress {
    pub fn parse(text: &str) -> Result<Self, InvalidSiteAddress> {
        let refuse = || InvalidSiteAddress(text.to_owned());
        let rest = text.strip_prefix("https://").ok_or_else(refuse)?;
        if rest.chars().any(char::is_whitespace) {
            return Err(refuse());
        }
        let slash = rest.find('/').ok_or_else(refuse)?;
        if slash == 0 || !rest.ends_with('/') {
            return Err(refuse());
        }
        Ok(Self {
            full: text.to_owned(),
            path: rest[slash..].to_owned(),
        })
    }

    /// The part after the host: `/` or, behind a gateway, `/blog/`.
    /// Always starts and ends with `/`.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// The full address of a page, given its address relative to the site's
    /// root: `articles/a.html` becomes `https://host/blog/articles/a.html`.
    pub fn join(&self, relative: &str) -> String {
        format!("{}{relative}", self.full)
    }
}

#[cfg(test)]
mod tests {
    use super::SiteAddress;

    #[test]
    fn accepts_the_root_of_a_domain() {
        let address = SiteAddress::parse("https://www.lucalorenzon.com/").unwrap();
        assert_eq!(address.path(), "/");
    }

    #[test]
    fn accepts_an_address_under_a_prefix() {
        let address = SiteAddress::parse("https://host/blog/").unwrap();
        assert_eq!(address.path(), "/blog/");
    }

    #[test]
    fn joins_a_page_onto_the_root_of_a_domain() {
        let address = SiteAddress::parse("https://www.lucalorenzon.com/").unwrap();
        assert_eq!(
            address.join("articles/a.html"),
            "https://www.lucalorenzon.com/articles/a.html"
        );
    }

    #[test]
    fn joins_a_page_onto_an_address_under_a_prefix() {
        let address = SiteAddress::parse("https://example.com/blog/").unwrap();
        assert_eq!(
            address.join("articles/a.html"),
            "https://example.com/blog/articles/a.html"
        );
    }

    #[test]
    fn rejects_other_schemes() {
        assert!(SiteAddress::parse("ftp://host/").is_err());
        assert!(SiteAddress::parse("http://host/").is_err());
    }

    #[test]
    fn rejects_a_missing_scheme() {
        assert!(SiteAddress::parse("www.lucalorenzon.com/").is_err());
    }

    #[test]
    fn rejects_an_empty_host() {
        assert!(SiteAddress::parse("https:///").is_err());
        assert!(SiteAddress::parse("https:///blog/").is_err());
    }

    #[test]
    fn rejects_a_missing_final_slash() {
        assert!(SiteAddress::parse("https://host").is_err());
        assert!(SiteAddress::parse("https://host/blog").is_err());
    }

    #[test]
    fn rejects_whitespace_and_empty_text() {
        assert!(SiteAddress::parse("https://host/ blog/").is_err());
        assert!(SiteAddress::parse("").is_err());
    }

    #[test]
    fn the_message_names_the_value_and_the_rule() {
        let message = SiteAddress::parse("ftp://x").unwrap_err().to_string();
        assert!(message.contains("ftp://x"));
        assert!(message.contains("https://"));
    }
}
