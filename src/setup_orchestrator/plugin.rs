use bevy::ecs::system::command::insert_resource;
use bevy::prelude::*;
use crate::app_states::AppState;
use crate::app_states::AppState::*;
use crate::setup_orchestrator::systems::*;
use crate::setup_orchestrator::resources::*;

pub struct SetupOrchestratorPlugin;

impl Plugin for SetupOrchestratorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SetupOrchestrator),
                    setup_orchestrator
            )
        // .insert_resource(ToOrchestrator)
        // .insert_resource(FromOrchestrator)
        ;
    }
}