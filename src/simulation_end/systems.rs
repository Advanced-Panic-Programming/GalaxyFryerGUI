use bevy::prelude::*;
use crate::galaxy_view::messages::ReceivedSimulationEnd;

/// When the orchestrator sends the SimulationEnd message to the GUI, the app_state_manager system will
/// generate the corresponding 'ReceivedSimulationEnd' event.
/// Once generated, this function will receive it and terminate the application.
/// 
/// I decided to create a state for this for both clearance and to avoid to terminate the application while other systems were running 
/// (bc I would have been in another AppState)
pub fn end_simulation(
    mut simulation_end_reader: MessageReader<ReceivedSimulationEnd>,
    mut exit: MessageWriter<AppExit> // Bevy event to terminate app
) {
    if !simulation_end_reader.is_empty() {
        simulation_end_reader.clear();
        exit.write(AppExit::Success); // Bevy condition to terminate app safely
    }
}