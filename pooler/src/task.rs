use std::{
    panic::AssertUnwindSafe,
    sync::{Arc, Mutex},
    thread,
};

pub struct Task {
    thread: Option<thread::JoinHandle<()>>,

    is_canceled: Arc<Mutex<bool>>,
    is_processing: Arc<Mutex<bool>>,
}

impl Drop for Task {
    fn drop(&mut self) {
        {
            // Signal the thread to stop running
            let mut is_canceled = self.is_canceled.lock().unwrap();
            *is_canceled = true;
        }

        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}

impl Task {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn() -> bool + Send + 'static,
    {
        let is_canceled = Arc::new(Mutex::new(false));
        let is_processing = Arc::new(Mutex::new(false));

        let thread_is_canceled = is_canceled.clone();
        let thread_is_processing = is_processing.clone();

        let thread = thread::spawn(move || loop {
            // Check if the thread has been signaled to stop
            let is_cancelled = {
                let guard = thread_is_canceled.lock().unwrap();
                *guard
            };
            if is_cancelled {
                break;
            }

            // Set the processing flag to true
            {
                let mut processing = thread_is_processing.lock().unwrap();
                *processing = true;
            }

            _ = std::panic::catch_unwind(AssertUnwindSafe(|| {
                let result = handler();

                if result {
                    // Signal that the thread is no longer processing
                    let mut processing = thread_is_processing.lock().unwrap();
                    *processing = false;

                    // If the handler returns true, we can break the loop
                    return;
                }
            }));

            // Reset the processing flag to false
            {
                let mut processing = thread_is_processing.lock().unwrap();
                *processing = false;
            }

            // Sleep for a short duration to avoid busy-waiting
            thread::sleep(std::time::Duration::from_millis(10));
        });

        Self {
            thread: Some(thread),
            is_canceled,
            is_processing,
        }
    }

    pub fn is_finished(&self) -> bool {
        if let Some(thread) = &self.thread {
            return thread.is_finished();
        }

        true
    }

    pub fn is_busy(&self) -> bool {
        let is_processing = self.is_processing.lock().unwrap();
        *is_processing
    }

    pub fn is_canceled(&self) -> bool {
        let is_canceled = self.is_canceled.lock().unwrap();
        *is_canceled
    }

    pub fn cancel(&self) {
        let mut is_canceled = self.is_canceled.lock().unwrap();
        *is_canceled = true;
    }
}
