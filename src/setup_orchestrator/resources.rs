use bevy::prelude::*;
use crossbeam_channel::{Sender, Receiver};
use galaxy_fryer::app::orchestrator::{GUIToOrchestrator, OrchestratorToGUI};

#[derive(Resource)]
pub struct ToOrchestrator(pub Sender<GUIToOrchestrator>);
#[derive(Resource)]
pub struct FromOrchestrator(pub Receiver<OrchestratorToGUI>);
#[derive(Default)]
pub enum OrchestratorMode {
    #[default]
    ManualMode,
    AutomaticMode,
}
#[derive(Resource, Default)]
pub struct CurrentOrchestratorMode { pub(crate) mode: OrchestratorMode }