use bevy::prelude::*;
use crate::log::resources::*;

/// This functions reads all the LogMessage events and adds them to the Log. This way, we only have 1 function responsible for adding messages to the log
pub(super) fn collect_log_events(
    mut events: MessageReader<LogMessage>,
    mut store: ResMut<LogStore>,
    time: Res<Time>,
) {
    for event in events.read() {
        store.push(LogEntry {
            timestamp: time.elapsed_secs(),
            level: event.level.clone(),
            message: event.message.as_str().into(), // String → Box<str>
        });
    }
}