use chrono::{DateTime, Utc};
use log::{Level, Metadata, Record};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMetadata {
    pub level: Level,
    pub target: String,
}

impl From<&Metadata<'_>> for LogMetadata {
    fn from(value: &Metadata<'_>) -> Self {
        Self {
            level: value.level(),
            target: value.target().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRecord {
    pub ts: DateTime<Utc>,
    pub msg: String,
    pub metadata: LogMetadata,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
}

impl From<&Record<'_>> for LogRecord {
    fn from(record: &Record) -> Self {
        let msg = format!("{}", record.args());
        Self {
            ts: Utc::now(),
            msg,
            metadata: LogMetadata::from(record.metadata()),
            module_path: record.module_path().map(ToString::to_string),
            file: record.file().map(ToString::to_string),
            line: record.line(),
        }
    }
}
