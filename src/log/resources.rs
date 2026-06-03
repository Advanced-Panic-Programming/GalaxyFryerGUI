use std::collections::VecDeque;
use bevy::prelude::*;

#[derive(Message)]
pub struct LogMessage {
    pub level: LogLevel,
    pub message: String,
}

// Still don't know if useful
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

#[derive(Clone)]
pub struct LogEntry {
    pub timestamp: f32, // simulation time
    pub level: LogLevel,
    pub message: Box<str>, // saves 8 byte per entry
}

#[derive(Resource)]
pub struct LogStore {
    entries: VecDeque<LogEntry>,
    max_entries: usize,
}

impl LogStore {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(max_entries),
            max_entries,
        }
    }

    pub(crate) fn push(&mut self, entry: LogEntry) {
        if self.entries.len() >= self.max_entries {
            self.entries.pop_front(); // deletes the oldest
        }
        self.entries.push_back(entry);
    }

    /// Iterator from the newest entry
    pub fn iter_newest_first(&self) -> impl Iterator<Item = &LogEntry> {
        self.entries.iter().rev()
    }

    /// Current entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}