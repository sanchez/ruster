/// Extends iterators with the ability to extract a single element if the iterator contains exactly one item.
///
/// This trait is automatically implemented for all types that implement [`Iterator`].
///
/// # Examples
///
/// ```
/// use dater::SingleIterator;
///
/// let vec = vec![1];
/// assert_eq!(vec.into_iter().single(), Some(1));
///
/// let empty: Vec<i32> = vec![];
/// assert_eq!(empty.into_iter().single(), None);
///
/// let multiple = vec![1, 2];
/// assert_eq!(multiple.into_iter().single(), None);
/// ```
pub trait SingleIterator: Iterator {
    /// Returns the single element of the iterator, or `None` if the iterator is empty or has multiple elements.
    ///
    /// This method consumes the iterator to determine if it contains exactly one element.
    ///
    /// # Examples
    ///
    /// With a single element:
    /// ```
    /// use dater::SingleIterator;
    ///
    /// let vec = vec![42];
    /// assert_eq!(vec.into_iter().single(), Some(42));
    /// ```
    ///
    /// With an empty iterator:
    /// ```
    /// use dater::SingleIterator;
    ///
    /// let empty: Vec<i32> = vec![];
    /// assert_eq!(empty.into_iter().single(), None);
    /// ```
    ///
    /// With multiple elements:
    /// ```
    /// use dater::SingleIterator;
    ///
    /// let multiple = vec![1, 2, 3];
    /// assert_eq!(multiple.into_iter().single(), None);
    /// ```
    ///    /// # Returns
    ///
    /// - Returns `Some(item)` if the iterator contains exactly one element
    /// - Returns `None` if the iterator is empty or contains more than one element
    #[inline]
    fn single(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let first = self.next();
        if self.next().is_some() {
            None
        } else {
            first
        }
    }
}

impl<T: ?Sized> SingleIterator for T where T: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_with_one_element() {
        let vec = vec![42];
        assert_eq!(vec.into_iter().single(), Some(42));
    }

    #[test]
    fn test_single_with_empty_iterator() {
        let vec: Vec<i32> = vec![];
        assert_eq!(vec.into_iter().single(), None);
    }

    #[test]
    fn test_single_with_multiple_elements() {
        let vec = vec![1, 2, 3];
        assert_eq!(vec.into_iter().single(), None);
    }

    #[test]
    fn test_single_with_strings() {
        let vec = vec!["hello".to_string()];
        assert_eq!(vec.into_iter().single(), Some("hello".to_string()));
    }

    #[test]
    fn test_single_with_reference_iterator() {
        let vec = vec![1, 2, 3];
        assert_eq!(vec.iter().filter(|&&x| x == 2).single(), Some(&2));
    }
}
