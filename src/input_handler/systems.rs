use bevy::prelude::*;
use galaxy_fryer::app::gui_protocol::GUIToOrchestrator::*;
use crate::app_state_manager::messages::*;
use crate::app_state_manager::resources::{CurrentMode, Mode};
use crate::app_states::AppState;
use crate::manual_mode::resources::ManualModeState;
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
    gui_to_orch: ResMut<ToOrchestrator>,
    mut pause_menu_writer: MessageWriter<PausePressed>,
) {
    // Pause Pressed
    if keyboard_input.just_pressed(KeyCode::Escape) {
        pause_menu_writer.write(PausePressed);
        let _ = gui_to_orch.0.send(PauseSimulation);
    }
    // Play Pressed handled in app_state_manager since the Message is sent as the 'play' button is pressed
    
}

pub fn game_related_inputs(
    current_state: ResMut<State<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    gui_to_orch: ResMut<ToOrchestrator>,
    mut explorer_data: ResMut<ExplorersData>,
    mut planet_view_writer: MessageWriter<PlanetViewPressed>, 
    mut selected_planet: ResMut<SelectedPlanet>,
    mut manual_mode: Option<ResMut<ManualModeState>>,
    mut current_mode: ResMut<CurrentMode>,
) {
    // Set Manual Mode
    if keyboard_input.just_pressed(KeyCode::KeyM) && *current_state.get() != AppState::PauseMenu && current_mode.current == Mode::Automatic {
        let _ = gui_to_orch.0.send(ManualMode); // sends ManualMode message to orchestrator
        // Active Manual Mode Panel
        if let Some(ref mut mm) = manual_mode {
            mm.active = !mm.active;
        }
        // Update Resource
        current_mode.current = Mode::Manual;

    }
    // Set Automatic Mode
    if keyboard_input.just_pressed(KeyCode::KeyA) && *current_state.get() != AppState::PauseMenu && current_mode.current == Mode::Manual {
        let _ = gui_to_orch.0.send(AutomaticMode); // sends ManualMode message to orchestrator
        // Update Resource
        current_mode.current = Mode::Automatic;
    }
    // Pause Simulation
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        let _ = gui_to_orch.0.send(PauseSimulation);
    }
    // Resume Simulation
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        let _ = gui_to_orch.0.send(ResumeSimulation);
    }
    // Cycle Explorer -> sets PlanetView
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        if *current_state.get() != AppState::PauseMenu {
            explorer_data.switch_last_cycle();
            planet_view_writer.write(PlanetViewPressed);
            if explorer_data.get_last_cycle() {
                match selected_planet.set(explorer_data.explorer1.get_current_planet_index()) {
                    Ok(_) => {}
                    Err(_) => {
                        panic!("Out of bounds planet index");
                    }
                }
            } else {
                match selected_planet.set(explorer_data.explorer2.get_current_planet_index()) {
                    Ok(_) => {}
                    Err(_) => {
                        panic!("Out of bounds planet index");
                    }
                }
            }
        }
    }
}