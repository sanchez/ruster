use timer::DateTime;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogMessage {
    Debug(String),
    Verbose(String),
    Info(String),
    Success(String),
    Warning(String),
    Error(String),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogMessagePayload {
    pub message: LogMessage,
    pub timestamp: DateTime,
    pub module: String,
}
