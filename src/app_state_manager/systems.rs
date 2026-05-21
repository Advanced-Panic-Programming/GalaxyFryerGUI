use bevy::app::ctrlc::SignalType::Termination;
use bevy::prelude::*;

use super::messages::*;
use crate::app_states::*;
use crate::AppState::*;
use crate::galaxy_view::messages::ReceivedSimulationEnd;
use crate::setup_orchestrator::resources::ToOrchestrator;

pub fn handle_setup_simulation_completed(
    mut message: MessageReader<SetupSimulationCompleted>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !message.is_empty() {
        message.clear(); // Remove SetupSimulationCompleted message from queue
        info!("State transition: SetupSimulation -> SetupOrchestrator");
        next_state.set(SetupOrchestrator);
    }
}
pub fn handle_setup_orchestrator_completed(
    mut message: MessageReader<SetupOrchestratorCompleted>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !message.is_empty() {
        message.clear();
        info!("State transition: SetupOrchestrator -> Pause Menu");
        next_state.set(PauseMenu);
    }
}
pub fn handle_play_pressed(
    mut message: MessageReader<PlayPressed>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !message.is_empty() {
        message.clear();
        info!("State transition: Pause Menu -> GalaxyView");
        next_state.set(GalaxyView);
    }
}
pub fn handle_pause_pressed(
    mut message: MessageReader<PausePressed>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !message.is_empty() {
        message.clear();
        info!("State transition: {:?} -> PauseMenu", next_state);
        next_state.set(PauseMenu);
    }
}

pub fn handle_exit_pressed(
    mut message: MessageReader<ExitPressed>,
    mut next_state: ResMut<NextState<AppState>>,
    // mut gui_to_orch: ResMut<ToOrchestrator>,
) {
    if !message.is_empty() {
        message.clear();
        info!("Starting termination process"); // Send termination command to orchestrator and waits for response
        //TODO! gui_to_orch.0.send(GuiToOrchestrator::EndSimulation)
        next_state.set(SimulationEnd);
    }
}

pub fn handle_galaxy_view_pressed(
    mut message: MessageReader<GalaxyViewPressed>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !message.is_empty() {
        message.clear();
        info!("State transition: {:?} -> GalaxyView", next_state);
        next_state.set(GalaxyView);

    }
}
pub fn handle_planet_view_pressed(
    mut message: MessageReader<PlanetViewPressed>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !message.is_empty() {
        message.clear();
        info!("State transition: {:?} -> Pause Menu", next_state);
        next_state.set(PlanetView);
    }
}
pub fn handle_simulation_completed( //TODO! Ridondante, definito anche in communication/systems. Rimuovere
    mut message: MessageReader<SimulationCompleted>,
    mut writer: MessageWriter<ReceivedSimulationEnd>,
) {
    if !message.is_empty() {
        message.clear();
        writer.write(ReceivedSimulationEnd);
    }
}

pub fn handle_manual_mode(
    mut message: MessageReader<ActiveManualMode>,
) {
    if !message.is_empty() {
        message.clear();
        //TODO attivare menu manual mode (nuovo plugin)
    }
}

pub fn handle_automatic_mode(
    mut message: MessageReader<ActiveManualMode>,
) {
    if !message.is_empty() {
        message.clear();
        //TODO disattivare menu manual mode
    }
}