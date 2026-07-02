use bevy::prelude::*;
use crate::log::resources::*;
use crate::log::systems::*;
use crate::log::ui::plugin::LogUIPlugin;

/// This plugin is responsible for showing the events in the log panel. Each system is responsible to generate its own events 'LogMessage'.
/// This plugin then reads the events and shows them in the log.
pub struct LogPlugin;

impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<LogMessage>()
            .insert_resource(LogStore::new(2000))
            .add_plugins(LogUIPlugin)
            .add_systems(Update, collect_log_events); // It's ok for it to run at everytime
    }
}