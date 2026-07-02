use bevy::prelude::*;
use crate::app_states::AppState::*;
use crate::setup_orchestrator::systems::*;
use crate::setup_orchestrator::resources::*;

/// This module is responsible for the initialization of the orchestrator and the communication channels. It runs after 'SetupSimulation' and before the actual simulation
/// ensuring that the communication channels and the actual galaxy logic (handled in the orchestrator) is up and running for the GUI systems to interact with.
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