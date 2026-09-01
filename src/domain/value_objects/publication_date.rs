#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PublicationDate {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PublicationDateError {
    #[error("publication date is missing")]
    Missing,
    #[error("invalid publication date {raw:?}: expected YYYY-MM-DD, a real calendar date")]
    Malformed { raw: String },
}

impl PublicationDate {
    pub fn parse(raw: Option<&str>) -> Result<Self, PublicationDateError> {
        let raw = raw
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or(PublicationDateError::Missing)?;

        let malformed = || PublicationDateError::Malformed {
            raw: raw.to_string(),
        };

        let parts: Vec<&str> = raw.split('-').collect();
        let [y, m, d] = parts.as_slice() else {
            return Err(malformed());
        };
        if y.len() != 4 || m.len() != 2 || d.len() != 2 {
            return Err(malformed());
        }
        let (year, month, day) = (
            y.parse::<u16>().map_err(|_| malformed())?,
            m.parse::<u8>().map_err(|_| malformed())?,
            d.parse::<u8>().map_err(|_| malformed())?,
        );
        if !(1..=12).contains(&month) {
            return Err(malformed());
        }
        if day < 1 || day > days_in_month(year, month) {
            return Err(malformed());
        }
        Ok(Self { year, month, day })
    }
}

impl std::fmt::Display for PublicationDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

fn is_leap_year(year: u16) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => unreachable!("month already validated to 1..=12"),
    }
}
