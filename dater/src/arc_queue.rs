use std::sync::{Arc, Mutex};

use crate::{LinearData, Queue};

/// A thread-safe queue implementation using `Arc` and `Mutex`.
///
/// This data structure provides a FIFO (First-In-First-Out) queue that can be safely shared
/// across multiple threads. It wraps a standard `Queue<T>` with atomic reference counting
/// and mutual exclusion to ensure thread safety.
///
/// # Type Parameters
///
/// * `T` - The type of elements stored in the queue. Must implement `Send` to be thread-safe.
///
/// # Examples
///
/// ```
/// use dater::ArcQueue;
///
/// let queue = ArcQueue::new();
/// queue.push(1);
/// assert_eq!(queue.pop(), Some(1));
/// assert!(queue.is_empty());
/// ```
pub struct ArcQueue<T> {
    items: Arc<Mutex<Queue<T>>>,
}

impl<T> Clone for ArcQueue<T> {
    fn clone(&self) -> Self {
        ArcQueue {
            items: Arc::clone(&self.items),
        }
    }
}

impl<T> ArcQueue<T> {
    pub fn new() -> Self {
        ArcQueue {
            items: Arc::new(Mutex::new(Queue::new())),
        }
    }

    /// Adds an item to the back of the queue.
    ///
    /// This operation acquires a mutex lock on the underlying queue. If the lock
    /// cannot be acquired (e.g., due to poison), this method will panic.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to add to the queue
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned.
    pub fn push(&self, item: T) {
        let mut guard = self.items.lock().expect("Mutex was poisoned");
        guard.push(item);
    }

    /// Removes and returns the item at the front of the queue.
    ///
    /// This operation acquires a mutex lock on the underlying queue. If the lock
    /// cannot be acquired (e.g., due to poison), this method will panic.
    ///
    /// # Returns
    ///
    /// * `Some(T)` - The front item if the queue is not empty
    /// * `None` - If the queue is empty
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned.
    pub fn pop(&self) -> Option<T> {
        let mut guard = self.items.lock().expect("Mutex was poisoned");
        guard.pop()
    }

    /// Checks if the queue is empty.
    ///
    /// This operation acquires a mutex lock on the underlying queue. If the lock
    /// cannot be acquired (e.g., due to poison), this method will panic.
    ///
    /// # Returns
    ///
    /// `true` if the queue contains no elements, `false` otherwise.
    ///
    /// # Panics
    ///
    /// Panics if the mutex is poisoned.
    pub fn is_empty(&self) -> bool {
        let guard = self.items.lock().expect("Mutex was poisoned");
        guard.is_empty()
    }
}

impl<T> Default for ArcQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_basic_operations() {
        let queue = ArcQueue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_is_empty() {
        let queue = ArcQueue::new();
        assert!(queue.is_empty());

        queue.push(1);
        assert!(!queue.is_empty());

        queue.pop();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_thread_safety() {
        let queue = ArcQueue::new();
        let queue_clone = queue.clone();

        // Producer thread
        let producer = thread::spawn(move || {
            for i in 0..100 {
                queue_clone.push(i);
            }
        });

        // Consumer thread
        let consumer = thread::spawn(move || {
            let mut sum = 0;
            let mut count = 0;
            while count < 100 {
                if let Some(value) = queue.pop() {
                    sum += value;
                    count += 1;
                }
            }
            sum
        });

        producer.join().unwrap();
        let sum = consumer.join().unwrap();

        // Sum of numbers 0..100
        assert_eq!(sum, (0..100).sum());
    }
}
