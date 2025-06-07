//! A thread-safe reactive state container that supports change notifications.
//!
//! This module provides a `Signal<T>` type that implements a reactive state pattern.
//! It allows safe state management across multiple threads with callback support
//! for reacting to state changes.

use std::sync::{Arc, Mutex};

/// A thread-safe container for reactive state management.
///
/// `Signal<T>` provides a way to manage state that can be safely shared across threads
/// while allowing interested parties to react to state changes through callbacks.
///
/// # Type Parameters
///
/// * `T` - The type of value to be stored. Must implement `Send + Sync + 'static`
///
/// # Examples
///
/// ```
/// use pooler::Signal;
///
/// let counter = Signal::new(0);
/// counter.listen(|value| println!("Counter changed to: {}", value));
/// counter.set(42); // Will trigger the callback
/// assert_eq!(counter.get(), 42);
/// ```
pub struct Signal<T> {
    value: Arc<Mutex<T>>,
    callbacks: Arc<Mutex<Vec<Box<dyn Fn(&T) + Send>>>>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Signal {
            value: self.value.clone(),
            callbacks: self.callbacks.clone(),
        }
    }
}

impl<T> Default for Signal<T>
where
    T: Default + Send + Sync + 'static,
{
    fn default() -> Self {
        Signal::new(T::default())
    }
}

impl<T> Signal<T>
where
    T: Send + Sync + 'static,
{
    /// Creates a new `Signal` with the given initial value.
    ///
    /// # Arguments
    ///
    /// * `initial_value` - The initial state value
    ///
    /// # Examples
    ///
    /// ```
    /// use pooler::Signal;
    ///
    /// let signal = Signal::new(String::from("Hello"));
    /// ```
    pub fn new(initial_value: T) -> Self {
        Signal {
            value: Arc::new(Mutex::new(initial_value)),
            callbacks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Updates the signal's value and notifies all registered callbacks.
    ///
    /// This method acquires locks on both the value and callbacks, updates the value,
    /// then notifies all registered callbacks with the new value.
    ///
    /// # Arguments
    ///
    /// * `new_value` - The new state value
    ///
    /// # Examples
    ///
    /// ```
    /// use pooler::Signal;
    ///
    /// let signal = Signal::new(0);
    /// signal.listen(|value| println!("New value: {}", value));
    /// signal.set(42); // Will trigger the callback
    /// ```
    pub fn set(&self, new_value: T) {
        let mut value = self.value.lock().unwrap();
        *value = new_value;

        let callbacks = self.callbacks.lock().unwrap();
        for callback in callbacks.iter() {
            callback(&*value);
        }
    }

    /// Retrieves a clone of the current value.
    ///
    /// # Type Constraints
    ///
    /// The stored type `T` must implement `Clone`.
    ///
    /// # Returns
    ///
    /// A clone of the current value.
    ///
    /// # Examples
    ///
    /// ```
    /// use pooler::Signal;
    ///
    /// let signal = Signal::new(42);
    /// assert_eq!(signal.get(), 42);
    /// ```
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        let value = self.value.lock().unwrap();
        value.clone()
    }

    /// Registers a callback function to be called when the signal's value changes.
    ///
    /// # Arguments
    ///
    /// * `callback` - A function that takes an immutable reference to the signal's value.
    ///               Must implement `Send` to ensure thread safety.
    ///
    /// # Examples
    ///
    /// ```
    /// use pooler::Signal;
    ///
    /// let signal = Signal::new(0);
    /// signal.listen(|value| println!("Value changed to: {}", value));
    /// signal.set(42); // Will trigger the callback
    /// ```
    pub fn listen<F>(&self, callback: F)
    where
        F: Fn(&T) + Send + 'static,
    {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.push(Box::new(callback));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::thread;

    #[test]
    fn test_new_signal() {
        let signal = Signal::new(42);
        assert_eq!(signal.get(), 42);
    }

    #[test]
    fn test_signal_set_get() {
        let signal = Signal::new(0);
        signal.set(42);
        assert_eq!(signal.get(), 42);
    }

    #[test]
    fn test_signal_callback() {
        let signal = Signal::new(0);
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        signal.listen(move |value| {
            assert_eq!(*value, 42);
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        signal.set(42);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_multiple_callbacks() {
        let signal = Signal::new(0);
        let counter = Arc::new(AtomicU32::new(0));

        for _ in 0..3 {
            let counter_clone = counter.clone();
            signal.listen(move |_| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }

        signal.set(42);
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_thread_safety() {
        let signal = Signal::new(0);
        let counter = Arc::new(AtomicU32::new(0));
        let thread_count = 10;
        let mut handles = vec![];

        // Add callbacks from different threads
        for _ in 0..thread_count {
            let signal = signal.clone();
            let counter = counter.clone();
            let handle = thread::spawn(move || {
                signal.listen(move |_| {
                    counter.fetch_add(1, Ordering::SeqCst);
                });
            });
            handles.push(handle);
        }

        // Wait for all callbacks to be registered
        for handle in handles {
            handle.join().unwrap();
        }

        // Trigger callbacks
        signal.set(42);
        assert_eq!(counter.load(Ordering::SeqCst), thread_count);
    }

    #[test]
    fn test_signal_clone() {
        let signal1 = Signal::new(0);
        let signal2 = signal1.clone();

        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        signal1.listen(move |value| {
            assert_eq!(*value, 42);
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Setting through cloned signal should trigger callback
        signal2.set(42);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        assert_eq!(signal1.get(), 42);
        assert_eq!(signal2.get(), 42);
    }

    #[test]
    fn test_signal_default() {
        let signal: Signal<i32> = Signal::default();
        assert_eq!(signal.get(), 0);
    }
}
