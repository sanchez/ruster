use std::panic::AssertUnwindSafe;
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;

use crate::{structures::LinearData, Dispatcher};

/// A worker thread that executes jobs from a shared job queue.
///
/// Workers run in their own thread and continuously process jobs until dropped.
/// Each worker has a unique ID and maintains its own thread handle.
///
/// # Panic Safety
///
/// Workers catch panics from job execution to prevent them from crashing the thread.
/// This allows the worker to continue processing other jobs even if one job panics.
pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,

    is_canceled: Arc<Mutex<bool>>,
    is_processing: Arc<Mutex<bool>>,
}

impl Drop for Worker {
    fn drop(&mut self) {
        println!("[Worker {}] Dropped, cancelling thread.", self.id);
        {
            // Signal the worker to stop running
            let mut is_canceled = self.is_canceled.lock().unwrap();
            *is_canceled = true;
        }

        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}

impl Worker {
    /// Creates a new worker with the specified ID that processes jobs from a shared receiver.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this worker
    /// * `receiver` - Shared receiver for jobs
    ///
    /// # Returns
    ///
    /// A new `Worker` instance
    pub fn new<T, F>(id: usize, mut receiver: Dispatcher<T>, handler: F) -> Self
    where
        T: Send + 'static,
        F: Fn(T) + Send + 'static,
    {
        let is_canceled = Arc::new(Mutex::new(false));
        let is_processing = Arc::new(Mutex::new(false));

        let thread_is_canceled = is_canceled.clone();
        let thread_is_processing = is_processing.clone();

        let thread = thread::spawn(move || loop {
            // Check if the worker has been signaled to stop
            let is_cancelled = {
                let guard = thread_is_canceled.lock().unwrap();
                *guard
            };
            if is_cancelled {
                println!("[Worker {}] Is cancelled.", id);
                break;
            }

            if let Some(payload) = receiver.pop() {
                // Set the processing flag to true
                {
                    let mut processing = thread_is_processing.lock().unwrap();
                    *processing = true;
                }

                _ = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    handler(payload);
                }));

                {
                    let mut processing = thread_is_processing.lock().unwrap();
                    *processing = false;
                }
            }

            // Sleep for a short duration to prevent busy waiting
            sleep(Duration::from_millis(10));
        });

        Self {
            id,
            thread: Some(thread),
            is_canceled,
            is_processing,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    /// Checks if the worker's thread has completed.
    ///
    /// # Returns
    ///
    /// `true` if the worker thread has finished, `false` otherwise.
    pub fn is_finished(&self) -> bool {
        if let Some(t) = &self.thread {
            return t.is_finished();
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
        println!("[Worker {}] Got request to cancel.", self.id);
        let mut is_canceled = self.is_canceled.lock().unwrap();
        *is_canceled = true;
    }
}
