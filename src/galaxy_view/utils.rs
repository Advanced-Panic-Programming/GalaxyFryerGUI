use bevy::prelude::Res;
use common_game::components::resource::{BasicResourceType, ComplexResourceType};
use common_game::utils::ID;
use galaxy_fryer::app::orchestrator::{GUIToOrchestrator, Orchestrator};
use galaxy_fryer::orchestrator::GUIToOrchestrator::*;
use crate::setup_orchestrator::resources::ToOrchestrator;

pub const ANIMATION_FPS: u8 = 6;
pub const PLANET_INITIAL_SPLAT: f32 = 3.0;

pub const EXPLORER_ARROW_VERTICAL_OFFSET: f32 = 45.0;
pub const EXPLORER_ARROW_HORIZONTAL_OFFSET: f32 = 10.0; //Applied only if 2 explorer are on the same planet

pub const EXPLORER_ARROW_DIMENSION: f32 = 72.0;
pub const EXPLORER_INITIAL_SPLAT: f32 = 0.5;
pub const EXPLORER_1_SPRITE_PATH: &str = "explorer_arrows/red_explorer_arrow.png";
pub const EXPLORER_2_SPRITE_PATH: &str = "explorer_arrows/green_explorer_arrow.png";


// ============================
// === Send to Orchestrator ===
// ============================

//TODO this functions should be declared inside the manual mode menu. Move them when needed
fn set_automatic_mode(gui_to_orch: Res<ToOrchestrator>) {
    let _ = gui_to_orch.0.send(AutomaticMode);
}
fn set_manual_mode(gui_to_orch: Res<ToOrchestrator>) {
    let _ = gui_to_orch.0.send(ManualMode);
}
fn pause_simulation(gui_to_orch: Res<ToOrchestrator>) {
    let _ = gui_to_orch.0.send(PauseSimulation);
}
fn resume_simulation(gui_to_orch: Res<ToOrchestrator>) {
    let _ = gui_to_orch.0.send(ResumeSimulation);
}
fn send_sunray(gui_to_orch: Res<ToOrchestrator>, planet_id: ID) {
    let _ = gui_to_orch.0.send(SendSunray {planet_id});
}
fn send_asteroid(gui_to_orch: Res<ToOrchestrator>, planet_id: ID) {
    let _ = gui_to_orch.0.send(SendAsteroid {planet_id});
}
fn ask_planet_state(gui_to_orch: Res<ToOrchestrator>, planet_id: ID) { 
    let _ = gui_to_orch.0.send(AskPlanetState {planet_id});
}
fn move_explorer(gui_to_orch: Res<ToOrchestrator>, explorer_id: ID, planet_id: ID) {
    let _ = gui_to_orch.0.send(MoveExplorer {explorer_id, planet_id});
}
fn start_explorer_ai(gui_to_orch: Res<ToOrchestrator>, explorer_id: ID) {
    let _ = gui_to_orch.0.send(StartExplorerAI {explorer_id});
}
fn stop_explorer_ai(gui_to_orch: Res<ToOrchestrator>, explorer_id: ID) {
    let _ = gui_to_orch.0.send(StopExplorerAI {explorer_id});
}
fn ask_available_generate(gui_to_orch: Res<ToOrchestrator>, explorer_id: ID) {
    let _ = gui_to_orch.0.send(AskAvailableGenerate {explorer_id});
}
fn ask_available_combine(gui_to_orch: Res<ToOrchestrator>, explorer_id: ID) {
    let _ = gui_to_orch.0.send(AskAvailableCombine {explorer_id});
}
fn ask_to_generate(gui_to_orch: Res<ToOrchestrator>, explorer_id: ID, resource: BasicResourceType) {
    let _ = gui_to_orch.0.send(AskToGenerate {explorer_id, resource});
}
fn ask_to_combine(gui_to_orch: Res<ToOrchestrator>, explorer_id: ID, combine: ComplexResourceType) {
    let _ = gui_to_orch.0.send(AskToCombine {explorer_id, combine});
}