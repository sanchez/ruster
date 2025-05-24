use crate::Domain;

/// Trait for types that can be clamped to a given domain.
///
/// Implementing this trait allows a type to be restricted to the bounds
/// of a specified domain using the `domain_clamp` method.
pub trait DomainClamp {
    /// Clamps the value to be within the specified domain.
    ///
    /// # Arguments
    /// * `domain` - The domain to clamp the value to
    ///
    /// # Returns
    /// A new value of the same type that lies within the domain bounds
    fn domain_clamp(&self, domain: &Domain) -> Self;
}

impl Domain {
    /// Clamps a value to be within this domain.
    ///
    /// # Examples
    /// ```
    /// use mather::Domain;
    ///
    /// let domain = Domain::new(0.0, 10.0);
    /// assert_eq!(domain.clamp(-5.0), 0.0);
    /// assert_eq!(domain.clamp(5.0), 5.0);
    /// assert_eq!(domain.clamp(15.0), 10.0);
    /// ```
    pub fn clamp<T>(&self, value: T) -> T
    where
        T: DomainClamp,
    {
        value.domain_clamp(self)
    }
}

impl DomainClamp for f64 {
    fn domain_clamp(&self, domain: &Domain) -> f64 {
        match *self {
            value if value < domain.start => domain.start,
            value if value > domain.end => domain.end,
            value => value,
        }
    }
}

impl DomainClamp for Domain {
    fn domain_clamp(&self, domain: &Domain) -> Domain {
        Domain::new(
            self.start.domain_clamp(domain),
            self.end.domain_clamp(domain),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        let domain = Domain::new(0.0, 10.0);
        assert_eq!(domain.clamp(-5.0), 0.0);
        assert_eq!(domain.clamp(0.0), 0.0);
        assert_eq!(domain.clamp(5.0), 5.0);
        assert_eq!(domain.clamp(10.0), 10.0);
        assert_eq!(domain.clamp(15.0), 10.0);
    }

    #[test]
    fn test_clamp_negative_domain() {
        let domain = Domain::new(-10.0, -5.0);
        assert_eq!(domain.clamp(-15.0), -10.0);
        assert_eq!(domain.clamp(-7.5), -7.5);
        assert_eq!(domain.clamp(0.0), -5.0);
    }

    #[test]
    fn test_domain_clamp_domain() {
        let outer_domain = Domain::new(0.0, 10.0);

        // Test inner domain getting clamped to outer bounds
        let inner_domain = Domain::new(-5.0, 15.0);
        let clamped = outer_domain.clamp(inner_domain);
        assert_eq!(clamped.start, 0.0);
        assert_eq!(clamped.end, 10.0);

        // Test partial overlap
        let partial_domain = Domain::new(5.0, 15.0);
        let clamped = outer_domain.clamp(partial_domain);
        assert_eq!(clamped.start, 5.0);
        assert_eq!(clamped.end, 10.0);
    }
}
