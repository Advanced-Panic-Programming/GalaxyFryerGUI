use bevy::prelude::*;
use crate::app_states::AppState;
use crate::setup_simulation::resources::{Explorer, ExplorersData, LastState, SelectedPlanet};

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

pub fn change_app_state(
    current_state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut selected_planet: ResMut<SelectedPlanet>,
    mut last_state: ResMut<LastState>,
    mut explorer_data: ResMut<ExplorersData>,
) {
    // GalaxyView State
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if *current_state.get() != AppState::GalaxyView && *current_state.get() != AppState::PauseMenu {
            last_state.state = current_state.clone();
        next_state.set(AppState::GalaxyView);
        }
    }
    // PlanetView State
    if let Some(key) = keyboard_input.get_just_pressed().next() && *current_state.get() != AppState::PauseMenu {
        if let Some(index) = digit_to_index(key) {
            let _ = selected_planet.set(index);
            last_state.state = current_state.clone();
            next_state.set(AppState::PlanetView);
        }
    }
    // Handle Manual Mode
    if keyboard_input.just_pressed(KeyCode::KeyM) && *current_state.get() != AppState::PauseMenu {

    }
    // Open/Close Menu
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if current_state.clone() == AppState::PauseMenu {
            next_state.set(last_state.state);
        } else {
            last_state.state = current_state.clone();
            next_state.set(AppState::PauseMenu);
        }
    }
    // Cycle Explorer
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        if *current_state.get() != AppState::PauseMenu {
            explorer_data.switch();
            last_state.state = current_state.clone();
            next_state.set(AppState::PlanetView);
        }
    }
    // Pause Simulation
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        
    }
}