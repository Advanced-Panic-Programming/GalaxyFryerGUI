use bevy::prelude::*;
use crate::log::resources::*;
use crate::log::systems::*;
use crate::log::ui::plugin::LogUIPlugin;

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