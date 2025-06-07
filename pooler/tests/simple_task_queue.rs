use pooler::TaskQueue;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn test_basic_task_execution() {
    let counter = Arc::new(Mutex::new(0));

    let handler_counter = counter.clone();
    let queue = TaskQueue::new(1, move |num: usize| {
        let mut count = handler_counter.lock().unwrap();
        *count += num;
    });

    for _ in 1..=5 {
        queue.push(1);
    }

    queue.wait();
    let result = counter.lock().unwrap();
    assert_eq!(
        *result, 5,
        "The task should have been executed 5 times, incrementing the counter by 1 each time."
    );
}

#[test]
fn test_multiple_workers() {
    let counter = Arc::new(Mutex::new(0));
    let handler_counter = counter.clone();

    // Create queue with 3 workers
    let queue = TaskQueue::new(3, move |num: usize| {
        let mut count = handler_counter.lock().unwrap();
        *count += num;
        // Add small delay to ensure concurrent execution
        thread::sleep(Duration::from_millis(10));
    });

    // Push 9 items
    for _ in 0..9 {
        queue.push(1);
    }

    queue.wait();
    assert_eq!(*counter.lock().unwrap(), 9);
}

#[test]
fn test_queue_empty_and_busy() {
    let counter = Arc::new(Mutex::new(0));
    let handler_counter = counter.clone();

    let queue = TaskQueue::new(1, move |num: usize| {
        thread::sleep(Duration::from_millis(50)); // Simulate work
        let mut count = handler_counter.lock().unwrap();
        *count += num;
    });

    assert!(queue.is_empty(), "Queue should be empty initially");
    assert!(!queue.is_busy(), "Queue should not be busy initially");

    queue.push(1);

    // Give the worker a moment to start
    thread::sleep(Duration::from_millis(10));
    assert!(queue.is_busy(), "Queue should be busy while processing");

    queue.wait();
    assert!(!queue.is_busy(), "Queue should not be busy after waiting");
    assert!(queue.is_empty(), "Queue should be empty after processing");
    assert_eq!(*counter.lock().unwrap(), 1);
}

#[test]
fn test_concurrent_pushes() {
    let counter = Arc::new(Mutex::new(0));
    let handler_counter = counter.clone();

    let queue = Arc::new(TaskQueue::new(2, move |num: usize| {
        let mut count = handler_counter.lock().unwrap();
        *count += num;
    }));

    let mut handles = vec![];

    // Spawn 5 threads that each push 10 items
    for _ in 0..5 {
        let queue = queue.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..10 {
                queue.push(1);
                thread::sleep(Duration::from_millis(1));
            }
        }));
    }

    // Wait for all pushes to complete
    for handle in handles {
        handle.join().unwrap();
    }

    queue.wait();
    assert_eq!(*counter.lock().unwrap(), 50);
}
