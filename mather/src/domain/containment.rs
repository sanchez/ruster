use crate::Domain;

impl Domain {
    /// Checks if a value is within this domain, inclusive.
    ///
    /// # Examples
    /// ```
    /// use geomagic::data::Domain;
    ///
    /// let domain = Domain::new(0.0, 10.0);
    /// assert!(domain.has(5.0));
    /// assert!(!domain.has(11.0));
    /// ```
    pub fn contains_value(&self, value: f64) -> bool {
        value >= self.start && value <= self.end
    }

    /// Checks if a value is within this domain, inclusive.
    ///
    /// # Examples
    /// ```
    /// use geomagic::data::Domain;
    ///
    /// let domain = Domain::new(0.0, 10.0);
    /// assert!(domain.has(5.0));
    /// assert!(!domain.has(11.0));
    /// ```
    pub fn strictly_contains_value(&self, value: f64) -> bool {
        value > self.start && value < self.end
    }

    /// Checks if this domain completely contains another domain.
    ///
    /// # Examples
    /// ```
    /// use geomagic::data::Domain;
    ///
    /// let domain1 = Domain::new(0.0, 10.0);
    /// let domain2 = Domain::new(2.0, 8.0);
    /// assert!(domain1.contains(domain2));
    /// ```
    pub fn contains_domain(&self, other: Domain) -> bool {
        self.contains_value(other.start) && self.contains_value(other.end)
    }

    /// Checks if this domain completely contains another domain.
    ///
    /// # Examples
    /// ```
    /// use geomagic::data::Domain;
    ///
    /// let domain1 = Domain::new(0.0, 10.0);
    /// let domain2 = Domain::new(2.0, 8.0);
    /// assert!(domain1.contains(domain2));
    /// ```
    pub fn strictly_contains_domain(&self, other: Domain) -> bool {
        self.strictly_contains_value(other.start) && self.strictly_contains_value(other.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    #[test]
    fn test_has() {
        let domain = Domain::new(0.0, 10.0);
        assert!(domain.contains_value(0.0));
        assert!(domain.contains_value(5.0));
        assert!(domain.contains_value(10.0));
        assert!(!domain.contains_value(-1.0));
        assert!(!domain.contains_value(11.0));
    }

    #[test]
    fn test_contains() {
        let domain = Domain::new(0.0, 10.0);
        assert!(domain.contains_domain(Domain::new(0.0, 10.0)));
        assert!(domain.contains_domain(Domain::new(2.0, 8.0)));
        assert!(!domain.contains_domain(Domain::new(-1.0, 5.0)));
        assert!(!domain.contains_domain(Domain::new(5.0, 11.0)));
    }

    #[test]
    fn test_length() {
        let domain = Domain::new(0.0, 10.0);
        assert_eq!(domain.length(), 10.0);
        let domain = Domain::new(-5.0, 5.0);
        assert_eq!(domain.length(), 10.0);
    }
}
