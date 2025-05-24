pub trait LinearData<T> {
    /// Adds an item to the data structure.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to add
    fn push(&mut self, item: T);

    /// Removes and returns an item from the data structure.
    ///
    /// The exact item removed depends on the specific implementation
    /// (e.g., LIFO for Stack, FIFO for Queue).
    ///
    /// # Returns
    ///
    /// * `Some(T)` - The removed item if the structure is not empty
    /// * `None` - If the structure is empty
    fn pop(&mut self) -> Option<T>;

    /// Returns a reference to the next item to be popped without removing it.
    ///
    /// # Returns
    ///
    /// * `Some(&T)` - Reference to the next item if the structure is not empty
    /// * `None` - If the structure is empty
    fn peek(&mut self) -> Option<&T>;

    /// Checks if the data structure is empty.
    ///
    /// # Returns
    ///
    /// `true` if the structure contains no elements, `false` otherwise.
    fn is_empty(&self) -> bool;
}
