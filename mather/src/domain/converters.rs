use crate::Domain;
use std::num::ParseFloatError;
use std::str::FromStr;

/// Errors that can occur when converting strings to Domain
#[derive(Debug)]
pub enum ConvertError {
    /// Error parsing float values
    ParseFloatError(ParseFloatError),
    /// Missing opening or closing parentheses
    MissingParentheses,
    /// Invalid format (should be "(start,end)")
    InvalidFormat,
}

impl FromStr for Domain {
    type Err = ConvertError;

    /// Converts a string in the format "(start,end)" to a Domain.
    ///
    /// # Examples
    /// ```
    /// use mather::Domain;
    /// use std::str::FromStr;
    ///
    /// let domain = Domain::from_str("(0.0,10.0)").unwrap();
    /// assert_eq!(domain.start, 0.0);
    /// assert_eq!(domain.end, 10.0);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('(') || !s.ends_with(')') {
            return Err(ConvertError::MissingParentheses);
        }

        let s = s.trim_start_matches('(').trim_end_matches(')');

        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(ConvertError::InvalidFormat);
        }

        let start = parts[0]
            .parse()
            .map_err(|x| ConvertError::ParseFloatError(x))?;
        let end = parts[1]
            .parse()
            .map_err(|x| ConvertError::ParseFloatError(x))?;
        Ok(Domain { start, end })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_string() {
        let range = Domain::from_str("(0.0,10.0)").unwrap();
        assert_eq!(range.start, 0.0);
        assert_eq!(range.end, 10.0);
    }

    #[test]
    fn test_missing_parentheses() {
        assert!(matches!(
            Domain::from_str("0.0,10.0"),
            Err(ConvertError::MissingParentheses)
        ));
    }

    #[test]
    fn test_invalid_number() {
        assert!(matches!(
            Domain::from_str("(invalid,10.0)"),
            Err(ConvertError::ParseFloatError(_))
        ));
    }

    #[test]
    fn test_invalid_format() {
        assert!(matches!(
            Domain::from_str("(0.0)"),
            Err(ConvertError::InvalidFormat)
        ));
    }
}
