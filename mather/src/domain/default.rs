use crate::Domain;

/// Provides a default implementation for Domain
///
/// Creates a Domain with:
/// - start: 0.0
/// - end: 1.0
///
/// This represents a standard unit interval [0,1]
impl Default for Domain {
    fn default() -> Self {
        Domain {
            start: 0.0,
            end: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let domain = Domain::default();
        assert_eq!(domain.start, 0.0);
        assert_eq!(domain.end, 1.0);
    }

    #[test]
    fn test_default_is_valid_interval() {
        let domain = Domain::default();
        assert!(domain.start < domain.end);
    }
}
