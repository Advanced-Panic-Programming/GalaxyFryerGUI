use bevy::prelude::*;
use crate::galaxy_view::messages::{ReceivedAutomaticModeAck, ReceivedManualModeAck};
use crate::setup_orchestrator::resources::{CurrentOrchestratorMode, OrchestratorMode};

/// When the GUI asks to change to manual/automatic mode, it sends a GUIToOrchestrator message
/// The orchestrator responds with a OrchestratorToGUI message (ManualModeAck/AutomaticModeAck)
/// The response is captured in the 'communication' module that creates the corresponding event (ReceivedManualModeAck/ReceivedAutomaticModeAck)
/// This event is then captured here and the change to the actual 'OrchestratorMode' resource is done.
///
/// NB: The actual change is the game is handled by the 'game_mode' module that waits for a 'CurrentOrchestratorMode' resource update.
pub fn handle_orchestrator_mode(
    mut orchestrator_mode: ResMut<CurrentOrchestratorMode>,
    mut manual_mode_reader: MessageReader<ReceivedManualModeAck>,
    mut automatic_mode_reader: MessageReader<ReceivedAutomaticModeAck>,
) {
    if !manual_mode_reader.is_empty() {
        manual_mode_reader.clear();
        orchestrator_mode.mode = OrchestratorMode::ManualMode;
    }
    if !automatic_mode_reader.is_empty() {
        automatic_mode_reader.clear();
        orchestrator_mode.mode = OrchestratorMode::AutomaticMode;
    }
}