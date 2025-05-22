type Callback<T> = Box<dyn Fn(T) + Send + 'static>;

pub struct EventBus<T> {
    handlers: Vec<Callback<T>>,
}

impl<T> EventBus<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        EventBus {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler<F>(&mut self, handler: F)
    where
        F: Fn(T) + Send + 'static,
    {
        self.handlers.push(Box::new(handler));
    }

    pub fn notify(&self, message: T) {
        for handler in &self.handlers {
            handler(message);
        }
    }
}
