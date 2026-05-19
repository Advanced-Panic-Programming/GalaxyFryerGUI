use bevy::prelude::*;
use crate::communication::systems::receive_from_orchestrator;

pub struct CommunicationPlugin;

impl Plugin for CommunicationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Update systems
            .add_systems(Update,
                receive_from_orchestrator,
            )
        ;
    }
}