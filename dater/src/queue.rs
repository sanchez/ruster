/// A generic implementation of a Queue data structure using two stacks.
///
/// The Queue follows First-In-First-Out (FIFO) principles where elements
/// are added to the back and removed from the front.
use crate::{LinearData, Stack};

/// A Queue data structure that uses two stacks to achieve FIFO behavior.
/// The 'items' stack contains elements in the correct order for dequeuing,
/// while the 'backlog' stack stores newly pushed elements.
pub struct Queue<T> {
    items: Stack<T>,
    backlog: Stack<T>,
}

impl<T> Queue<T> {
    /// Creates a new empty queue.
    ///
    /// # Returns
    ///
    /// A new `Queue<T>` instance.
    pub fn new() -> Self {
        Queue {
            items: Stack::new(),
            backlog: Stack::new(),
        }
    }
}

impl<T> LinearData<T> for Queue<T> {
    /// Adds an item to the back of the queue.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to add to the queue
    fn push(&mut self, item: T) {
        self.backlog.push(item);
    }

    /// Removes and returns the item at the front of the queue.
    ///
    /// # Returns
    ///
    /// * `Some(T)` - The front item if the queue is not empty
    /// * `None` - If the queue is empty
    fn pop(&mut self) -> Option<T> {
        if let Some(item) = self.items.pop() {
            return Some(item);
        }

        // The items stack is empty, so we need to flush the backlog into the items stack
        while let Some(item) = self.backlog.pop() {
            self.items.push(item);
        }

        self.items.pop()
    }

    /// Returns a reference to the item at the front of the queue without removing it.
    ///
    /// # Returns
    ///
    /// * `Some(&T)` - Reference to the front item if the queue is not empty
    /// * `None` - If the queue is empty
    fn peek(&mut self) -> Option<&T> {
        if self.items.is_empty() {
            // The items stack is empty, so we need to flush the backlog into the items stack
            while let Some(item) = self.backlog.pop() {
                self.items.push(item);
            }
        }

        self.items.peek()
    }

    /// Checks if the queue is empty.
    ///
    /// # Returns
    ///
    /// `true` if the queue contains no elements, `false` otherwise.
    fn is_empty(&self) -> bool {
        self.items.is_empty() && self.backlog.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Queue::new()
    }
}

impl<T> Iterator for Queue<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_queue_is_empty() {
        let queue: Queue<i32> = Queue::new();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_push_and_pop() {
        let mut queue = Queue::new();

        // Test pushing elements
        queue.push(1);
        queue.push(2);
        queue.push(3);

        // Test popping elements (FIFO order)
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut queue = Queue::new();

        // Peek on empty queue
        assert_eq!(queue.peek(), None);

        // Push and peek
        queue.push(1);
        assert_eq!(queue.peek(), Some(&1));

        // Ensure peek doesn't remove the element
        assert_eq!(queue.peek(), Some(&1));
        queue.push(2);
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.peek(), Some(&2));
    }

    #[test]
    fn test_iterator() {
        let mut queue = Queue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);

        let items: Vec<i32> = queue.into_iter().collect();
        assert_eq!(items, vec![1, 2, 3]);
    }

    #[test]
    fn test_mixed_operations() {
        let mut queue = Queue::new();

        // Push some elements
        queue.push(1);
        queue.push(2);

        // Pop one
        assert_eq!(queue.pop(), Some(1));

        // Push more
        queue.push(3);
        queue.push(4);

        // Check remaining elements come out in order
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(4));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_default() {
        let queue: Queue<i32> = Queue::default();
        assert!(queue.is_empty());
    }
}
