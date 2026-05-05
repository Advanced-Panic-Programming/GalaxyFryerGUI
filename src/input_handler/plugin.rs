use bevy::prelude::*;
use super::systems::*;
pub struct InputHandlerPlugin;

impl Plugin for InputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                change_app_state,
            ))
        ;
    }
}