use bevy::prelude::*;
use galaxy_fryer::orchestrator::GUIToOrchestrator::*;
use crate::app_state_manager::messages::*;
use crate::app_states::AppState;
use crate::setup_simulation::resources::{Explorer, ExplorersData, SelectedPlanet};
use crate::setup_orchestrator::resources::*;

fn digit_to_index(key: &KeyCode) -> Option<usize> {
    match key {
        KeyCode::Digit1 => Some(0),
        KeyCode::Digit2 => Some(1),
        KeyCode::Digit3 => Some(2),
        KeyCode::Digit4 => Some(3),
        KeyCode::Digit5 => Some(4),
        KeyCode::Digit6 => Some(5),
        KeyCode::Digit7 => Some(6),
        _ => None
    }
}

pub fn app_states_affected_inputs(
    current_state: ResMut<State<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut selected_planet: ResMut<SelectedPlanet>,
    mut galaxy_view_writer: MessageWriter<GalaxyViewPressed>,
    mut planet_view_writer: MessageWriter<PlanetViewPressed>,
) {
    // GalaxyView State
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if *current_state.get() != AppState::GalaxyView && *current_state.get() != AppState::PauseMenu {
            galaxy_view_writer.write(GalaxyViewPressed);
        }
    }
    // PlanetView State
    if let Some(key) = keyboard_input.get_just_pressed().next() && *current_state.get() != AppState::PauseMenu {
        if let Some(index) = digit_to_index(key) {
            let _ = selected_planet.set(index);
            planet_view_writer.write(PlanetViewPressed);
        }
    }
}

pub fn menu_updates_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut pause_menu_writer: MessageWriter<PausePressed>,
) {
    // Pause Pressed
    if keyboard_input.just_pressed(KeyCode::Escape) {
        pause_menu_writer.write(PausePressed);
    }
    // Play Pressed handled in app_state_manager since the Message is sent as the 'play' button is pressed
    
}

pub fn game_related_inputs(
    current_state: ResMut<State<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // gui_to_orch: ResMut<ToOrchestrator>,
    mut explorer_data: ResMut<ExplorersData>,
    mut manual_mode_writer: MessageWriter<ActiveManualMode>,
    mut automatic_mode_writer: MessageWriter<ActiveAutomaticMode>,
    mut planet_view_writer: MessageWriter<PlanetViewPressed>, 
) {
    // Handle Manual Mode
    if keyboard_input.just_pressed(KeyCode::KeyM) && *current_state.get() != AppState::PauseMenu {
        // let _ = gui_to_orch.0.send(ManualMode); // sends ManualMode message to orchestrator
        manual_mode_writer.write(ActiveManualMode);
    }
    // Handle Automatic Mode
    if keyboard_input.just_pressed(KeyCode::KeyA) && *current_state.get() != AppState::PauseMenu {
        // let _ = gui_to_orch.0.send(AutomaticMode); // sends ManualMode message to orchestrator
        automatic_mode_writer.write(ActiveAutomaticMode);
    }
    // Pause Simulation
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        // gui_to_orch.0.send(PauseSimulation);
    }
    // Cycle Explorer -> sets PlanetView
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        if *current_state.get() != AppState::PauseMenu {
            explorer_data.switch();
            planet_view_writer.write(PlanetViewPressed);
        }
    }
}