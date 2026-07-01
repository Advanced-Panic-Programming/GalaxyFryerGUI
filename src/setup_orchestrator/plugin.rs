use bevy::prelude::*;
use crate::app_states::AppState::*;
use crate::setup_orchestrator::systems::*;
use crate::setup_orchestrator::resources::*;

pub struct SetupOrchestratorPlugin;

impl Plugin for SetupOrchestratorPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(CurrentOrchestratorMode::default())
            // OnEnter systems
            .add_systems(OnEnter(SetupOrchestrator),
                    setup_orchestrator,
            )
            // Update systems
            .add_systems(Update, setup_orchestrator_completed.run_if(in_state(SetupOrchestrator)))
        ;
    }
}