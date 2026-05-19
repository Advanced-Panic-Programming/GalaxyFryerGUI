use bevy::prelude::*;
use galaxy_fryer::orchestrator::{GUIToOrchestrator, OrchestratorToGUI};
use crate::setup_orchestrator::resources::{FromOrchestrator, ToOrchestrator};
use crate::galaxy_view::messages::*;

pub fn receive_from_orchestrator(
    receiver: Res<FromOrchestrator>,
    mut planet_destroyed_writer: MessageWriter<ReceivedPlanetDestroyed>,
    // ...
){
    while let Ok(msg) = receiver.0.try_recv() {
        match msg {
            OrchestratorToGUI::DefaultMessage => {}
            OrchestratorToGUI::SendAsteroidDestroyed{p_id} => {
                
            }
            _ => {}
        }
    }
}