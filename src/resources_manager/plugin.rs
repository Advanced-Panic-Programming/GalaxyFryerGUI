use bevy::prelude::*;
use crate::resources_manager::systems::*;
use crate::setup_orchestrator::resources::CurrentOrchestratorMode;

pub struct ResourceManagerPlugin;

impl Plugin for ResourceManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, handle_orchestrator_mode.run_if(resource_exists::<CurrentOrchestratorMode>))
        ;
    }
}