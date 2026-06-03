use bevy::prelude::*;
use bevy::tasks::futures_lite::StreamExt;
use crate::log::resources::*;

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
        // println!("{}", event.message); // Debug
    }
}