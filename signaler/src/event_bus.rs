/// A type alias for event handler callbacks that can be safely shared between threads
type Callback<T> = Box<dyn Fn(T) + Send + 'static>;

/// An event bus that enables decoupled communication between components through a publish-subscribe pattern.
///
/// The `EventBus` provides a thread-safe mechanism for components to subscribe to and receive notifications
/// about specific events without direct coupling to event producers. This promotes modular design and
/// helps maintain separation of concerns.
///
/// # Type Parameters
///
/// * `T` - The type of event data that will be shared through the bus. Must implement `Copy`.
///
/// # Examples
///
/// ```
/// use signaler::EventBus;
///
/// // Create an event bus for i32 events
/// let mut bus = EventBus::new();
///
/// // Add a handler that processes events
/// bus.add_handler(|value| println!("Received: {}", value));
///
/// // Notify all handlers with an event
/// bus.notify(42);
/// ```
pub struct EventBus<T> {
    /// Collection of registered event handlers
    handlers: Vec<Callback<T>>,
}

impl<T> EventBus<T>
where
    T: Copy,
{
    /// Creates a new empty event bus.
    ///
    /// Returns an `EventBus` instance with no registered handlers.
    pub fn new() -> Self {
        EventBus {
            handlers: Vec::new(),
        }
    }

    /// Registers a new event handler function.
    ///
    /// The handler will be called whenever a new event is published through `notify`.
    /// Multiple handlers can be registered, and they will be called in registration order.
    ///
    /// # Arguments
    ///
    /// * `handler` - A function that processes events of type `T`. Must be `Send` and `'static`
    ///   to ensure thread safety and proper lifetime management.
    pub fn add_handler<F>(&mut self, handler: F)
    where
        F: Fn(T) + Send + 'static,
    {
        self.handlers.push(Box::new(handler));
    }

    /// Publishes an event to all registered handlers.
    ///
    /// Iterates through all registered handlers and calls each one with the provided event data.
    /// The handlers are called synchronously in their registration order.
    ///
    /// # Arguments
    ///
    /// * `message` - The event data to be passed to each handler.
    pub fn notify(&self, message: T) {
        for handler in &self.handlers {
            handler(message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_new_bus_has_no_handlers() {
        let bus: EventBus<i32> = EventBus::new();
        assert_eq!(bus.handlers.len(), 0);
    }

    #[test]
    fn test_add_handler_increases_handler_count() {
        let mut bus = EventBus::new();
        bus.add_handler(|_: i32| {});
        assert_eq!(bus.handlers.len(), 1);
    }

    #[test]
    fn test_notify_calls_all_handlers() {
        let mut bus = EventBus::new();
        let counter1 = Arc::new(Mutex::new(0));
        let counter2 = Arc::new(Mutex::new(0));

        let counter1_clone = counter1.clone();
        let counter2_clone = counter2.clone();

        bus.add_handler(move |_| {
            let mut count = counter1_clone.lock().unwrap();
            *count += 1;
        });

        bus.add_handler(move |_| {
            let mut count = counter2_clone.lock().unwrap();
            *count += 1;
        });

        bus.notify(42);

        assert_eq!(*counter1.lock().unwrap(), 1);
        assert_eq!(*counter2.lock().unwrap(), 1);
    }

    #[test]
    fn test_handler_receives_correct_value() {
        let mut bus = EventBus::new();
        let received = Arc::new(Mutex::new(None));
        let received_clone = received.clone();

        bus.add_handler(move |value: i32| {
            let mut received = received_clone.lock().unwrap();
            *received = Some(value);
        });

        bus.notify(42);

        assert_eq!(*received.lock().unwrap(), Some(42));
    }

    #[test]
    fn test_multiple_notifications() {
        let mut bus = EventBus::new();
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();

        bus.add_handler(move |_| {
            let mut count = counter_clone.lock().unwrap();
            *count += 1;
        });

        for _ in 0..3 {
            bus.notify(1);
        }

        assert_eq!(*counter.lock().unwrap(), 3);
    }
}
