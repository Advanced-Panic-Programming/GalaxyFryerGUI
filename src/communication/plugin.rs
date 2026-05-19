use bevy::prelude::*;
use crate::communication::systems::receive_from_orchestrator;
use crate::setup_orchestrator::resources::FromOrchestrator;

/// This plugin manages the messages received from the orchestrators and generates the corresponding Bevy events.
pub struct CommunicationPlugin;

impl Plugin for CommunicationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Update systems
            .add_systems(Update, receive_from_orchestrator.run_if(resource_exists::<FromOrchestrator>));
    }
}