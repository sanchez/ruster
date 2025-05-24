use crate::Domain;

impl Domain {
    /// Returns the length of the domain.
    ///
    /// # Examples
    /// ```
    /// use geomagic::data::Domain;
    ///
    /// let domain : Domain = Domain::new(0.0, 10.0);
    /// assert_eq!(domain.length(), 10.0);
    /// ```
    pub fn length(&self) -> f64 {
        self.end - self.start
    }

    /// Returns if the domain is increasing.
    ///
    /// # Examples
    /// ```
    /// use geomagic::data::Domain;
    ///
    /// let domain : Domain = Domain::new(0.0, 10.0);
    /// domain.is_increasing();
    /// ```
    pub fn is_increasing(&self) -> bool {
        self.end > self.start
    }

    /// Returns if the domain is increasing.
    ///
    /// # Examples
    /// ```
    /// use geomagic::data::Domain;
    ///
    /// let domain : Domain = Domain::new(10.0, 0.0);
    /// domain.is_decreasing();
    /// ```
    pub fn is_decreasing(&self) -> bool {
        self.start < self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let domain = Domain::new(0.0, 10.0);
        assert_eq!(domain.length(), 10.0);
        let domain = Domain::new(-5.0, 5.0);
        assert_eq!(domain.length(), 10.0);
    }

    #[test]
    fn test_is_increasing() {
        let domain = Domain::new(0.0, 1.0);
        assert!(domain.is_increasing());
        let domain = Domain::new(1.0, 0.0);
        assert!(!domain.is_increasing());
        let domain = Domain::new(0.0, 0.0);
        assert!(!domain.is_increasing());
    }

    #[test]
    fn test_is_decreasing() {
        let domain = Domain::new(0.0, 1.0);
        assert!(!domain.is_decreasing());
        let domain = Domain::new(1.0, 0.0);
        assert!(domain.is_decreasing());
        let domain = Domain::new(0.0, 0.0);
        assert!(!domain.is_decreasing());
    }
}
