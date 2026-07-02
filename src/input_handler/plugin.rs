use bevy::prelude::*;
use crate::setup_orchestrator::resources::ToOrchestrator;
use super::systems::*;
pub struct InputHandlerPlugin;

/// This plugin handles all the inputs. When a button is pressed, if certain conditions are met, generates the corresponding event handled in the specific module
impl Plugin for InputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                app_states_affected_inputs,
                (
                    // Systems that depend on ToOrchestrator Resource:
                    // Since the resource is not init but is created from a system, we need to check if it exists before using it
                    menu_updates_inputs,
                    game_related_inputs,
                ).run_if(resource_exists::<ToOrchestrator>)
            ))
        ;
    }
}