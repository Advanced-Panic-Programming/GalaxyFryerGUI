use bevy::prelude::*;
use galaxy_fryer::app::gui_protocol::GUIToOrchestrator::{EndSimulation, PauseSimulation, ResumeSimulationFromAutomatic, ResumeSimulationFromManual};
use crate::app_state_manager::resources::{CurrentMode, Mode};
use super::messages::*;
use crate::app_states::*;
use crate::AppState::*;
use crate::galaxy_view::messages::{ReceivedAutomaticModeAck, ReceivedManualModeAck, ReceivedSimulationEnd};
use crate::setup_orchestrator::resources::{CurrentOrchestratorMode, OrchestratorMode, ToOrchestrator};

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
    gui_to_orchestrator: Res<ToOrchestrator>,
    current_mode: Res<CurrentMode>,
) {
    if !message.is_empty() {
        message.clear();
        info!("State transition: Pause Menu -> GalaxyView");
        next_state.set(GalaxyView);

        if current_mode.current == Mode::Automatic {
            let _ = gui_to_orchestrator.0.send(ResumeSimulationFromAutomatic);
        } else if current_mode.current == Mode::Manual {
            let _ = gui_to_orchestrator.0.send(ResumeSimulationFromManual);
        }
    }
}
pub fn handle_pause_pressed(
    mut message: MessageReader<PausePressed>,
    mut next_state: ResMut<NextState<AppState>>,
    gui_to_orchestrator: Res<ToOrchestrator>,
) {
    if !message.is_empty() {
        message.clear();
        info!("State transition: {:?} -> PauseMenu", next_state);
        next_state.set(PauseMenu);

        let _ = gui_to_orchestrator.0.send(PauseSimulation);
    }
}

pub fn handle_exit_pressed(
    mut message: MessageReader<ExitPressed>,
    mut next_state: ResMut<NextState<AppState>>,
    gui_to_orch: Res<ToOrchestrator>,
) {
    if !message.is_empty() {
        message.clear();
        info!("Starting termination process"); // Send termination command to orchestrator and waits for response
        let _ = gui_to_orch.0.send(EndSimulation);
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
pub fn handle_simulation_completed(
    mut simulation_end_reader: MessageReader<ReceivedSimulationEnd>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !simulation_end_reader.is_empty() {
        simulation_end_reader.clear();
        info!("State transition: {:?} -> SimulationCompleted", next_state);
        next_state.set(SimulationEnd);
        
    }
}

/// The current flow to set manual mode:
/// 1) GUI sends 'ManualMode' message to the orchestrator
/// 2) Orchestrator sends 'ManualModeAck' to the GUI
/// 3) GUI generates 'ReceivedManualModeAck' event and sets the 'CurrentOrchestratorMode' resource to ManualMode
/// 4) The event ReceivedManualModeAck is here handled:
///         - another event is generated: 'ActiveManualMode'
///         - and the 'CurrentOrchestratorMode' resource is set to ManualMode
/// 5) The new 'ActiveManualMode' event is handled in the game systems in order to enable the manual mode menu
pub fn handle_manual_mode(
    mut message: MessageReader<ReceivedManualModeAck>,
    mut manual_mode_writer: MessageWriter<ActiveManualMode>,
    mut orchestrator_mode: ResMut<CurrentOrchestratorMode>,
) {
    if !message.is_empty() {
        message.clear();
        manual_mode_writer.write(ActiveManualMode);
        orchestrator_mode.mode = OrchestratorMode::ManualMode;
        //TODO! enable menu manual mode (new plugin)
    }
}

/// The current flow to set automatic mode:
/// 1) GUI sends 'AutomaticMode' message to the orchestrator
/// 2) Orchestrator sends 'AutomaticModeAck' to the GUI
/// 3) GUI generates 'ReceivedAutomaticModeAck' event
/// 4) The event ReceivedAutomaticModeAck is here handled:
///         - another event is generated: 'ActiveAutomaticMode'
///         - and the 'CurrentOrchestratorMode' resource is set to AutomaticMode
/// 5) The new 'ActiveAutomaticMode' event is handled in the game systems in order to disable the manual mode menu
pub fn handle_automatic_mode(
    mut message: MessageReader<ReceivedAutomaticModeAck>,
    mut automatic_mode_writer: MessageWriter<ActiveAutomaticMode>,
    mut orchestrator_mode: ResMut<CurrentOrchestratorMode>,
) {
    if !message.is_empty() {
        message.clear();
        automatic_mode_writer.write(ActiveAutomaticMode);
        orchestrator_mode.mode = OrchestratorMode::AutomaticMode;
        //TODO! disable manual mode menu (new plugin)
    }
}