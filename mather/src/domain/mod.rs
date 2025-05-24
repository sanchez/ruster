mod clamp;
mod converters;
mod default;
mod from;
mod operators;
mod projection;

/// Represents a continuous domain between two f64 values.
///
/// # Examples
/// ```
/// use mather::Domain;
///
/// let domain : Domain = Domain::new(0.0, 1.0);
/// assert_eq!(domain.start, 0.0);
/// assert_eq!(domain.end, 1.0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Domain {
    /// The starting value of the domain
    pub start: f64,
    /// The ending value of the domain
    pub end: f64,
}

impl Domain {
    /// Creates a new Domain with the given start and end values.
    ///
    /// # Arguments
    /// * `start` - The starting value of the range
    /// * `end` - The ending value of the range
    pub fn new(start: f64, end: f64) -> Self {
        Domain { start, end }
    }

    /// Creates a new Unit Domain [0,1]
    pub fn unit_domain() -> Self {
        Domain::new(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let domain: Domain = Domain::new(1.0, 2.0);
        assert_eq!(domain.start, 1.0);
        assert_eq!(domain.end, 2.0);
    }
}
