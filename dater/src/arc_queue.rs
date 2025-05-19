use std::sync::{Arc, Mutex};

use crate::{LinearData, Queue};

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
}

impl<T> Default for ArcQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ArcQueue<T> {
    pub fn push(&self, item: T) {
        let mut items = self.items.lock().unwrap();
        items.push(item);
    }

    pub fn pop(&self) -> Option<T> {
        let mut items = self.items.lock().unwrap();
        items.pop()
    }

    fn is_empty(&self) -> bool {
        let items = self.items.lock().unwrap();
        items.is_empty()
    }
}
