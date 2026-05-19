use bevy::prelude::*;
use super::systems::*;
pub struct InputHandlerPlugin;

impl Plugin for InputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                app_states_affected_inputs,
                menu_updates_inputs,
                game_related_inputs, //.run_if(no_active_cutscene),
            ))
        ;
    }
}