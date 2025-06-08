use crate::DateTime;

pub struct SystemTime;

impl SystemTime {
    /// Returns the current system time as a Unix timestamp in milliseconds.
    pub fn now() -> DateTime {
        // Use the standard library's SystemTime to get the current time
        let now = std::time::SystemTime::now();
        // Convert to UNIX timestamp in milliseconds
        let timestamp = now
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;
        todo!()
    }
}
