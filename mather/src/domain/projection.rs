use crate::Domain;

impl Domain {
    /// Projects a value from [0,1] to this domain.
    ///
    /// # Examples
    /// ```
    /// use mather::Domain;
    ///
    /// let domain : Domain = Domain::new(0.0, 10.0);
    /// assert_eq!(domain.project(0.5), 5.0);
    /// ```
    pub fn project(&self, value: f64) -> f64 {
        let domain = self.end - self.start;
        value * domain + self.start
    }

    pub fn remap_value_from(&self, other: Domain, value: f64) -> f64 {
        // [a,b] -> [A,B]
        // let u be some parameter in [a,b]
        // (u - a) is the distance from a;
        // ratio = (u - a)/(b - a) is the legth of the domain
        // A + ratio * (B-A)
        other.start + (other.end - other.start) * (value - self.start) / (self.end - self.start)
    }

    pub fn remap_value_to(&self, other: Domain, value: f64) -> f64 {
        // [a,b] -> [A,B]
        // let u be some parameter in [a,b]
        // (u - a) is the distance from a;
        // ratio = (u - a)/(b - a) is the legth of the domain
        // A + ratio * (B-A)
        self.start + (self.end - self.start) * (value - other.start) / (other.end - other.start)
    }

    pub fn remap_domain_from(&self, other: Domain, value: Domain) -> Domain {
        let start: f64 = self.remap_value_from(other, value.start);
        let end: f64 = self.remap_value_from(other, value.end);
        Domain { start, end }
    }

    pub fn remap_domain_to(&self, other: Domain, value: Domain) -> Domain {
        let start: f64 = self.remap_value_to(other, value.start);
        let end: f64 = self.remap_value_to(other, value.end);
        Domain { start, end }
    }

    /// Projects a value from this domain to [0,1].
    ///
    /// # Examples
    /// ```
    /// use mather::Domain;
    ///
    /// let domain : Domain = Domain::new(0.0, 10.0);
    /// assert_eq!(domain.map(5.0), 0.5);
    /// ```
    pub fn map(&self, value: f64) -> f64 {
        let domain = self.end - self.start;
        (value - self.start) / domain
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    #[test]
    fn test_out() {
        let domain = Domain::new(0.0, 10.0);
        assert_eq!(domain.project(0.0), 0.0);
        assert_eq!(domain.project(0.5), 5.0);
        assert_eq!(domain.project(1.0), 10.0);
    }

    #[test]
    fn test_into() {
        let domain = Domain::new(0.0, 10.0);
        assert!((domain.map(0.0) - 0.0).abs() < EPSILON);
        assert!((domain.map(5.0) - 0.5).abs() < EPSILON);
        assert!((domain.map(10.0) - 1.0).abs() < EPSILON);
    }
}
