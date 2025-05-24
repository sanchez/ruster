/// Type alias for message handlers that can be registered with the queue.
/// The handler receives a message of type T and returns a boolean indicating whether
/// message propagation should stop (true) or continue (false).
type Callback<T> = Box<dyn Fn(T) -> bool + Send + 'static>;

/// A thread-safe message queue that supports multiple handlers with controlled message propagation.
///
/// `MessageQueue` allows registration of multiple callback handlers that process messages of type T.
/// Each handler can control whether subsequent handlers should process the message by returning
/// a boolean value.
///
/// # Type Parameters
///
/// * `T` - The type of messages that can be sent through the queue. Must implement Copy.
///
/// # Examples
///
/// ```
/// use signaler::MessageQueue;
///
/// let mut queue = MessageQueue::new();
/// queue.add_handler(|msg: i32| {
///     println!("Received: {}", msg);
///     false // allow message to propagate
/// });
/// queue.notify(42);
/// ```
#[derive(Default)]
pub struct MessageQueue<T> {
    handlers: Vec<Callback<T>>,
}

impl<T> MessageQueue<T>
where
    T: Copy,
{
    /// Creates a new empty message queue with no handlers.
    ///
    /// # Returns
    ///
    /// A new `MessageQueue` instance ready to accept handlers and messages.
    pub fn new() -> Self {
        MessageQueue {
            handlers: Vec::new(),
        }
    }

    /// Registers a new message handler with this queue.
    ///
    /// # Parameters
    ///
    /// * `handler` - A function that will be called when messages are sent through the queue.
    ///   The handler should return `true` to stop message propagation, or `false` to allow
    ///   the message to continue to subsequent handlers.
    ///
    /// # Type Parameters
    ///
    /// * `F` - A function type that implements `Fn(T) -> bool` and can be sent between threads safely.
    pub fn add_handler<F>(&mut self, handler: F)
    where
        F: Fn(T) -> bool + Send + 'static,
    {
        self.handlers.push(Box::new(handler));
    }

    /// Sends a message to all registered handlers in order of registration.
    ///
    /// Message propagation continues until either all handlers have processed the message
    /// or a handler returns true to stop propagation.
    ///
    /// # Parameters
    ///
    /// * `message` - The message to send to all registered handlers.
    pub fn notify(&self, message: T) {
        for handler in &self.handlers {
            if handler(message) {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_new_queue_is_empty() {
        let queue: MessageQueue<i32> = MessageQueue::new();
        assert!(queue.handlers.is_empty());
    }

    #[test]
    fn test_add_handler() {
        let mut queue = MessageQueue::new();
        queue.add_handler(|_: i32| false);
        assert_eq!(queue.handlers.len(), 1);
    }

    #[test]
    fn test_notify_single_handler() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let mut queue = MessageQueue::new();
        queue.add_handler(move |_: i32| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            false
        });

        queue.notify(42);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_notify_multiple_handlers() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut queue = MessageQueue::new();

        // Add three handlers that increment the counter
        for _ in 0..3 {
            let counter_clone = counter.clone();
            queue.add_handler(move |_: i32| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                false
            });
        }

        queue.notify(42);
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_handler_stops_propagation() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut queue = MessageQueue::new();

        // First handler increments and stops propagation
        let counter_clone = counter.clone();
        queue.add_handler(move |_: i32| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            true // stop propagation
        });

        // Second handler should never be called
        let counter_clone = counter.clone();
        queue.add_handler(move |_: i32| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            false
        });

        queue.notify(42);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_copy_type_constraint() {
        #[derive(Clone, Copy)]
        struct Message(i32);

        let mut queue = MessageQueue::new();
        queue.add_handler(|msg: Message| {
            assert_eq!(msg.0, 42);
            false
        });
        queue.notify(Message(42));
    }
}
