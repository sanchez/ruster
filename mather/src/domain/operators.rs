//! Operator implementations for the Domain type.
//!
//! This module provides mathematical operators for Domain objects,
//! allowing them to be combined and manipulated algebraically.
//!
//! # Addition
//! The `+` operator combines two domains by taking the minimum start value
//! and maximum end value of both domains. This effectively creates a new domain
//! that spans the entire range covered by both input domains.
//!
//! # Examples
//! ```
//! use mather::Domain;
//!
//! let domain1 = Domain::new(1.0, 3.0);
//! let domain2 = Domain::new(2.0, 4.0);
//! let combined = domain1 + domain2;
//! assert_eq!(combined.start, 1.0);
//! assert_eq!(combined.end, 4.0);
//! ```

use super::Domain;
use std::ops::Add;

macro_rules! impl_domain_operators {
    ($(($lhs:ty, $rhs:ty)), *) => {
        $(
            impl Add<$rhs> for $lhs {
                type Output = Domain;

                fn add(self, other: $rhs) -> Self::Output {
                    Domain {
                        start: self.start.min(other.start),
                        end: self.end.max(other.end),
                    }
                }
            }
        )*
    };
}

impl_domain_operators!(
    (Domain, Domain),
    (Domain, &Domain),
    (&Domain, Domain),
    (&Domain, &Domain)
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_ranges() {
        let domain1 = Domain::new(1.0, 3.0);
        let domain2 = Domain::new(2.0, 4.0);
        let result = domain1 + domain2;
        assert_eq!(result.start, 1.0);
        assert_eq!(result.end, 4.0);
    }

    #[test]
    fn add_overlapping_ranges() {
        let domain1 = Domain::new(1.0, 5.0);
        let domain2 = Domain::new(2.0, 4.0);
        let result = domain1 + domain2;
        assert_eq!(result.start, 1.0);
        assert_eq!(result.end, 5.0);
    }

    #[test]
    fn add_identical_ranges() {
        let domain1 = Domain::new(1.0, 3.0);
        let domain2 = Domain::new(1.0, 3.0);
        let result = domain1 + domain2;
        assert_eq!(result.start, 1.0);
        assert_eq!(result.end, 3.0);
    }

    #[test]
    fn add_disjoint_ranges() {
        let domain1 = Domain::new(1.0, 2.0);
        let domain2 = Domain::new(3.0, 4.0);
        let result = domain1 + domain2;
        assert_eq!(result.start, 1.0);
        assert_eq!(result.end, 4.0);
    }

    #[test]
    fn add_nested_ranges() {
        let domain1 = Domain::new(0.0, 5.0);
        let domain2 = Domain::new(1.0, 4.0);
        let result = domain1 + domain2;
        assert_eq!(result.start, 0.0);
        assert_eq!(result.end, 5.0);
    }

    #[test]
    fn add_zero_length_ranges() {
        let domain1 = Domain::new(1.0, 1.0);
        let domain2 = Domain::new(2.0, 2.0);
        let result = domain1 + domain2;
        assert_eq!(result.start, 1.0);
        assert_eq!(result.end, 2.0);
    }

    #[test]
    fn add_negative_ranges() {
        let domain1 = Domain::new(-3.0, -1.0);
        let domain2 = Domain::new(-2.0, 0.0);
        let result = domain1 + domain2;
        assert_eq!(result.start, -3.0);
        assert_eq!(result.end, 0.0);
    }

    #[test]
    fn add_with_references() {
        let domain1 = Domain::new(1.0, 3.0);
        let domain2 = Domain::new(2.0, 4.0);
        let result = &domain1 + &domain2;
        assert_eq!(result.start, 1.0);
        assert_eq!(result.end, 4.0);
    }
}
