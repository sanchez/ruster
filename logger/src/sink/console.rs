use crate::{sink::LoggerSink, LogMessage, LogMessagePayload};

pub struct ConsoleSink {}

impl LoggerSink for ConsoleSink {
    fn write(&self, message: LogMessagePayload) {
        match message.message {
            LogMessage::Debug(msg) => println!("[DEBUG] {}", msg),
            LogMessage::Verbose(msg) => println!("[VERBOSE] {}", msg),
            LogMessage::Info(msg) => println!("[INFO] {}", msg),
            LogMessage::Success(msg) => println!("[SUCCESS] {}", msg),
            LogMessage::Warning(msg) => println!("[WARNING] {}", msg),
            LogMessage::Error(msg) => println!("[ERROR] {}", msg),
        }
    }
}
