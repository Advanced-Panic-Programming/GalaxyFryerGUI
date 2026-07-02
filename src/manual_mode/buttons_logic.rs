use bevy::prelude::*;
use common_game::utils::ID;
use galaxy_fryer::app::gui_protocol::GUIToOrchestrator;
use crate::manual_mode::components::{AskCombineButton, AskGenerateButton, CombinableResourceSpinnerDecrementButton, CombinableResourceSpinnerIncrementButton, CombineButton, GenerateButton, GenerateResourceSpinnerDecrementButton, GenerateResourceSpinnerIncrementButton, ManualModePanelRoot, MoveButton, PlanetSpinnerDecrementButton, PlanetSpinnerIncrementButton, SendAsteroidButton, SendSunrayButton, StartAIButton, StopAIButton, TabButton};
use crate::manual_mode::resources::{CombinableResourcesOnPlanet, GeneratableResourcesOnPlanet, ManualModePanel, PlanetSpinner};
use crate::setup_orchestrator::resources::ToOrchestrator;
use crate::setup_simulation::resources::ExplorersData;
// This module contains all the buttons logic

// ==========================
//      Galaxy Tab Buttons
// ==========================

pub fn handle_planet_spinner_dec(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    query: Query<&Interaction, (Changed<Interaction>, With<PlanetSpinnerDecrementButton>)>,
    mut planet_spinner: ResMut<PlanetSpinner>,
) {

    if check_root.is_empty() {
        return;
    }

    for interaction in &query {
        if *interaction == Interaction::Pressed {
            planet_spinner.decrease();
        }
    }
}

pub fn handle_planet_spinner_inc(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    query: Query<&Interaction, (Changed<Interaction>, With<PlanetSpinnerIncrementButton>)>,
    mut planet_spinner: ResMut<PlanetSpinner>,
) {

    if check_root.is_empty() {
        return;
    }

    for interaction in &query {
        if *interaction == Interaction::Pressed {
            planet_spinner.increase();
        }
    }
}

pub fn handle_send_sunray(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    query: Query<&Interaction, (Changed<Interaction>, With<SendSunrayButton>)>,
    sender: Option<Res<ToOrchestrator>>,
    planet_spinner: Res<PlanetSpinner>,
) {

    if check_root.is_empty() {
        return;
    }

    for interaction in &query {
        if *interaction == Interaction::Pressed && let Some(s) = &sender {
            let planet_id = ID::from(planet_spinner.get_current_value() as u32);
            let _ = s.0.send(GUIToOrchestrator::SendSunray { planet_id });
        }
    }
}

pub fn handle_send_asteroid(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    query: Query<&Interaction, (Changed<Interaction>, With<SendAsteroidButton>)>,
    sender: Option<Res<ToOrchestrator>>,
    planet_spinner: Res<PlanetSpinner>,
) {

    if check_root.is_empty() {
        return;
    }

    for interaction in &query {
        if *interaction == Interaction::Pressed && let Some(s) = &sender {
            let planet_id = ID::from(planet_spinner.get_current_value() as u32);
            let _ = s.0.send(GUIToOrchestrator::SendAsteroid { planet_id});
        }
    }
}

// =============================
//      Explorers Tab Buttons
// =============================
pub fn handle_move_to_planet_button(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    query: Query<(&Interaction, &MoveButton), (Changed<Interaction>, With<MoveButton>)>,
    sender: Option<Res<ToOrchestrator>>,
    planet_spinner: Res<PlanetSpinner>,
) {

    if check_root.is_empty() {
        return;
    }

    for (interaction, btn) in &query {
        if *interaction == Interaction::Pressed && let Some(s) = &sender {
            let planet_id = ID::from(planet_spinner.get_current_value() as u32);
            let _ = s.0.send(GUIToOrchestrator::MoveExplorer { planet_id, explorer_id: btn.explorer_id});
        }
    }
}

// planet spinner is already handled in galaxy section because the query didn't specify the tab !!!

pub fn handle_ask_generate_button(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    query: Query<(&Interaction, &AskGenerateButton), Changed<Interaction>>,
    sender: Option<Res<ToOrchestrator>>,
) {

    if check_root.is_empty() {
        return;
    }

    for (interaction, btn) in query.iter() {
        if *interaction == Interaction::Pressed && let Some(s) = &sender {
            let _ = s.0.send(GUIToOrchestrator::AskAvailableGenerate {explorer_id: btn.explorer_id});
        }
    }
}

pub fn handle_generate_button(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    query: Query<(&Interaction, &GenerateButton), Changed<Interaction>>,
    sender: Option<Res<ToOrchestrator>>,
    generate_spinner: Res<GeneratableResourcesOnPlanet>,
    explorers_data: Res<ExplorersData>,
) {

    if check_root.is_empty() {
        return;
    }

    for (interaction, btn) in query.iter() {
        if *interaction == Interaction::Pressed && let Some(s) = &sender {
            let current_planet = match btn.explorer_id {
                0 => {
                    explorers_data.explorer1.get_current_planet_index() as u32
                }
                1 => {
                    explorers_data.explorer2.get_current_planet_index() as u32
                }
                _ => { panic!("Explorer index out of bounds")}
            };

            if let Some(to_generate) =
                generate_spinner.get_generate(current_planet).get_current_value() {
                    let _ = s.0.send(GUIToOrchestrator::AskToGenerate {
                        explorer_id: btn.explorer_id, resource: *to_generate });
            }
        }
    }
}

pub fn handle_generate_increment_button(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    query: Query<(&Interaction, &GenerateResourceSpinnerIncrementButton), (Changed<Interaction>, With<GenerateResourceSpinnerIncrementButton>)>,
    explorers_data: Res<ExplorersData>,
    mut generatable_resources_on_planet: ResMut<GeneratableResourcesOnPlanet>,
) {
    if check_root.is_empty() {
        return;
    }
    
    for (interaction, btn) in query.iter() {
        if *interaction == Interaction::Pressed {
            let current_planet = match btn.explorer_id {
                0 => explorers_data.explorer1.get_current_planet_index() as ID,
                1 => explorers_data.explorer2.get_current_planet_index() as ID,
                _ => { panic!("Explorer index out of bounds")}
            };
            generatable_resources_on_planet.get_generate_mut(current_planet).increase();
        }
    }
}

pub fn handle_generate_decrement_button(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    query: Query<(&Interaction, &GenerateResourceSpinnerDecrementButton), (Changed<Interaction>, With<GenerateResourceSpinnerDecrementButton>)>,
    explorers_data: Res<ExplorersData>,
    mut generatable_resources_on_planet: ResMut<GeneratableResourcesOnPlanet>,
) {
    if check_root.is_empty() {
        return;
    }

    for (interaction, btn) in query.iter() {
        if *interaction == Interaction::Pressed {
            let current_planet = match btn.explorer_id {
                0 => explorers_data.explorer1.get_current_planet_index() as ID,
                1 => explorers_data.explorer2.get_current_planet_index() as ID,
                _ => { panic!("Explorer index out of bounds")}
            };
            generatable_resources_on_planet.get_generate_mut(current_planet).decrease();
        }
    }
}

pub fn handle_ask_combine_button(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    query: Query<(&Interaction, &AskCombineButton), Changed<Interaction>>,
    sender: Option<Res<ToOrchestrator>>,
){

    if check_root.is_empty() {
        return;
    }

    for (interaction, btn) in query.iter() {
        if *interaction == Interaction::Pressed && let Some(s) = &sender {
            let _ = s.0.send(GUIToOrchestrator::AskAvailableCombine {explorer_id: btn.explorer_id} );
        }
    }
}

pub fn handle_combine_button(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    query: Query<(&Interaction, &CombineButton), Changed<Interaction>>,
    sender: Option<Res<ToOrchestrator>>,
    combine_spinner: Res<CombinableResourcesOnPlanet>,
    explorers_data: Res<ExplorersData>,
) {

    if check_root.is_empty() {
        return;
    }

    for (interaction, btn) in query.iter() {
        if *interaction == Interaction::Pressed && let Some(s) = &sender {
            let current_planet = match btn.explorer_id {
                0 => {
                    explorers_data.explorer1.get_current_planet_index() as u32
                }
                1 => {
                    explorers_data.explorer2.get_current_planet_index() as u32
                }
                _ => { panic!("Explorer index out of bounds")}
            };

            if let Some(to_combine) =
                combine_spinner.get_combine(current_planet).get_current_value() {
                let _ = s.0.send(GUIToOrchestrator::AskToCombine {
                    explorer_id: btn.explorer_id, combine: *to_combine });
            }
        }
    }
}

pub fn handle_combine_increment_button(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    query: Query<(&Interaction, &CombinableResourceSpinnerIncrementButton), (Changed<Interaction>, With<CombinableResourceSpinnerIncrementButton>)>,
    explorers_data: Res<ExplorersData>,
    mut combinable_resources_on_planet: ResMut<CombinableResourcesOnPlanet>,
) {
    if check_root.is_empty() {
        return;
    }

    for (interaction, btn) in query.iter() {
        if *interaction == Interaction::Pressed {
            let current_planet = match btn.explorer_id {
                0 => explorers_data.explorer1.get_current_planet_index() as ID,
                1 => explorers_data.explorer2.get_current_planet_index() as ID,
                _ => { panic!("Explorer index out of bounds")}
            };
            combinable_resources_on_planet.get_combine_mut(current_planet).increase();
        }
    }
}

pub fn handle_combine_decrement_button(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    query: Query<(&Interaction, &CombinableResourceSpinnerDecrementButton), (Changed<Interaction>, With<CombinableResourceSpinnerDecrementButton>)>,
    explorers_data: Res<ExplorersData>,
    mut combinable_resources_on_planet: ResMut<CombinableResourcesOnPlanet>,
) {
    
    if check_root.is_empty() {
       return ; 
    }
    
    for (interaction, btn) in query.iter() {
        if *interaction == Interaction::Pressed {
            let current_planet = match btn.explorer_id {
                0 => explorers_data.explorer1.get_current_planet_index() as ID,
                1 => explorers_data.explorer2.get_current_planet_index() as ID,
                _ => { panic!("Explorer index out of bounds")}
            };
            combinable_resources_on_planet.get_combine_mut(current_planet).decrease();
        }
    }
}

pub fn handle_start_ai_button(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    query: Query<(&Interaction, &StartAIButton), Changed<Interaction>>,
    sender: Option<Res<ToOrchestrator>>,
) {

    if check_root.is_empty() {
        return;
    }

    for (interaction, btn) in query.iter() {
        if *interaction == Interaction::Pressed && let Some(s) = &sender {
            let _ = s.0.send(GUIToOrchestrator::StartExplorerAI {explorer_id: btn.explorer_id});
        }
    }
}

pub fn handle_stop_ai_button(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    query: Query<(&Interaction, &StopAIButton), Changed<Interaction>>,
    sender: Option<Res<ToOrchestrator>>,
) {

    if check_root.is_empty() {
        return;
    }

    for (interaction, btn) in query.iter() {
        if *interaction == Interaction::Pressed && let Some(s) = &sender {
            let _ = s.0.send(GUIToOrchestrator::StopExplorerAI {explorer_id: btn.explorer_id});
        }
    }
}

// ==========================
//    change tab handlers
// ==========================
pub fn update_selected_tab(
    query: Query<(&Interaction, &TabButton), Changed<Interaction>>,
    mut manual_mode_panel: ResMut<ManualModePanel>,
) {
    for (interaction, btn) in query.iter() {
        if *interaction == Interaction::Pressed {
            manual_mode_panel.active_tab = btn.tab;
        }
    }
}