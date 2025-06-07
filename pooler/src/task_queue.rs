use crate::Task;
use dater::ArcQueue;

/// A concurrent task queue that processes items of type `T` using a fixed pool of worker tasks.
///
/// # Purpose
/// `TaskQueue` is designed to handle parallel processing of items using a fixed number of
/// worker tasks. This is useful for scenarios where you need to process items concurrently
/// but want to limit the number of simultaneous operations.
///
/// # Type Parameters
/// * `T` - The type of items to be processed. Must be `Send + Sync + 'static`
///
/// # Examples
/// ```
/// use pooler::TaskQueue;
/// use std::sync::{Arc, Mutex};
///
/// // Create a shared counter
/// let counter = Arc::new(Mutex::new(0));
/// let counter_clone = counter.clone();
///
/// // Create a task queue that increments the counter for each item
/// let queue = TaskQueue::new(2, move |num: i32| {
///     let mut count = counter_clone.lock().unwrap();
///     *count += num;
/// });
///
/// // Push items to process
/// queue.push(1);
/// queue.push(2);
///
/// // Wait for all items to be processed
/// queue.wait();
///
/// assert_eq!(*counter.lock().unwrap(), 3);
/// ```
pub struct TaskQueue<T> {
    tasks: Vec<Task>,
    backlog: ArcQueue<T>,
}

impl<T> TaskQueue<T>
where
    T: Send + Sync + 'static,
{
    /// Creates a new `TaskQueue` with the specified number of worker tasks.
    ///
    /// # Arguments
    /// * `task_count` - The number of worker tasks to create
    /// * `handler` - The function that processes each item. Must be `Send + Sync + 'static`
    ///
    /// # Returns
    /// A new `TaskQueue` instance ready to process items
    ///
    /// # Thread Safety
    /// The handler function will be called from multiple threads simultaneously, so it must
    /// be thread-safe. The provided handler is wrapped in an Arc for safe sharing.
    pub fn new<F>(task_count: usize, handler: F) -> Self
    where
        F: Fn(T) + Send + Sync + 'static,
    {
        let backlog = ArcQueue::new();
        let handler = std::sync::Arc::new(handler);

        let mut tasks = Vec::with_capacity(task_count);
        for _ in 0..task_count {
            let thread_backlog = backlog.clone();
            let handler = handler.clone();

            tasks.push(Task::new(move || {
                if let Some(item) = thread_backlog.pop() {
                    handler(item);
                }

                true
            }));
        }
        Self { tasks, backlog }
    }

    /// Pushes a new item to the task queue for processing.
    ///
    /// The item will be processed asynchronously by one of the worker tasks.
    ///
    /// # Arguments
    /// * `item` - The item to be processed
    pub fn push(&self, item: T) {
        self.backlog.push(item);
    }

    /// Checks if the task queue's backlog is empty.
    ///
    /// # Returns
    /// `true` if there are no items waiting to be processed, `false` otherwise
    ///
    /// # Note
    /// This does not indicate whether all processing is complete, as items may still
    /// be actively processing. Use `wait()` to ensure all processing is complete.
    pub fn is_empty(&self) -> bool {
        self.backlog.is_empty()
    }

    /// Checks if any worker tasks are currently processing items.
    ///
    /// # Returns
    /// `true` if any worker tasks are busy processing items, `false` otherwise
    pub fn is_busy(&self) -> bool {
        self.tasks.iter().any(|task| task.is_busy())
    }

    /// Waits for all items to be processed.
    ///
    /// This method blocks until:
    /// 1. The backlog is empty (no more items to process)
    /// 2. No tasks are currently busy processing items
    ///
    /// This ensures that all pushed items have been fully processed before returning.
    pub fn wait(&self) {
        while !self.is_empty() || self.is_busy() {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
