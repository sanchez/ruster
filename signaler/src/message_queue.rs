type Callback<T> = Box<dyn Fn(T) -> bool + Send + 'static>;

pub struct MessageQueue<T> {
    handlers: Vec<Callback<T>>,
}

impl<T> MessageQueue<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        MessageQueue {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler<F>(&mut self, handler: F)
    where
        F: Fn(T) -> bool + Send + 'static,
    {
        self.handlers.push(Box::new(handler));
    }

    pub fn notify(&self, message: T) {
        for handler in &self.handlers {
            if handler(message) {
                break;
            }
        }
    }
}
