use crate::Domain;

/// Implements conversion from a tuple of (f64, f64) to Domain
///
/// # Arguments
/// * `domain` - A tuple containing (start, end) values for the domain
///
/// # Example
/// ```
/// use mather::Domain;
/// let domain = Domain::from((0.0, 1.0));
/// assert_eq!(domain.start, 0.0);
/// assert_eq!(domain.end, 1.0);
/// ```
impl From<(f64, f64)> for Domain {
    fn from(domain: (f64, f64)) -> Self {
        Domain {
            start: domain.0,
            end: domain.1,
        }
    }
}
