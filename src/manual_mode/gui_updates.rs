use bevy::prelude::*;
use crate::manual_mode::components::{BagViewMarker, CombineResourceSpinnerValue, ExplorerCurrentPlanetMarker, ExplorerSpriteMarker, GenerateResourceSpinnerValue, ManualModePanelRoot, PlanetSpinnerValue, TabButton, TabContent};
use crate::manual_mode::resources::{CombinableResourcesOnPlanet, GeneratableResourcesOnPlanet, ManualModePanel, PlanetSpinner};
use crate::manual_mode::utils::*;
use crate::setup_simulation::resources::{ExplorersData, GalaxyOrbit};
// This module contains the systems that update the graphics part

/// Shows/hides the manual mode panel according to `ManualModePanel.visible`,
/// toggled by the M/A key bindings in `input_handler`.
pub fn update_panel_visibility(
    manual_mode_panel: Res<ManualModePanel>,
    mut query: Query<&mut Visibility, With<ManualModePanelRoot>>,
) {
    if manual_mode_panel.is_changed() {
        for mut visibility in &mut query {
            *visibility = if manual_mode_panel.visible {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

/// Shows the content node that matches the active tab; hides the rest.
/// Uses `Visibility::Inherited` (not `Visible`) for the active tab so that it
/// still inherits `Hidden` from the parent panel when manual mode is off.
pub fn update_tab_visibility(
    manual_mode_panel: Res<ManualModePanel>,
    mut tab_q: Query<(&TabContent, &mut Visibility)>,
) {
    if !manual_mode_panel.is_changed() {
        return;
    }

    for (tab, mut vis) in &mut tab_q {
        *vis = if tab.tab == manual_mode_panel.active_tab {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

pub fn update_tab_selector_colors(
    state: Res<ManualModePanel>,
    mut query: Query<(&Interaction, &mut BackgroundColor, &TabButton)>,
) {
    for (interaction, mut bg, btn) in &mut query {
        *bg = BackgroundColor(match interaction {
            Interaction::Hovered | Interaction::Pressed => BTN_HOVER_BG,
            Interaction::None => {
                if btn.tab == state.active_tab { TAB_ACTIVE_BG } else { TAB_INACTIVE_BG }
            }
        });
    }
}

pub fn update_orbit_for_manual_mode(
    state: Res<ManualModePanel>,
    mut orbit: ResMut<GalaxyOrbit>,
) {
    if !state.is_changed() {
        return;
    }
    orbit.center.y = if state.visible { ORBIT_Y_SHIFT_ACTIVE } else { 0.0 };
}

/// Updates the planet index shown according to the PlanetSpinner Resource
pub fn update_planet_spinner_value(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    mut query: Query<&mut Text, With<PlanetSpinnerValue>>,
    planet_spinner: Res<PlanetSpinner>,
) {

    if check_root.is_empty() {
        return;
    }
    
    if planet_spinner.is_changed() {
        for mut planet_index_text in query.iter_mut() {
            *planet_index_text = Text::new((planet_spinner.get_current_value() + 1).to_string());
        }
    }
}

/// Updates the planet index shown according to the Generate Resource
pub fn update_generate_spinner_value(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    mut query: Query<(&mut Text, &GenerateResourceSpinnerValue), With<GenerateResourceSpinnerValue>>,
    generate_resource_on_planet: Res<GeneratableResourcesOnPlanet>,
    explorers_data: Res<ExplorersData>,
) {

    if check_root.is_empty() {
        return;
    }
    
    if !generate_resource_on_planet.is_changed() {
        return;
    }

    for (mut text, value) in query.iter_mut() {
        let current_planet = match value.explorer_id {
            0 => {
                explorers_data.explorer1.get_current_planet_index()
            }
            1 => {
                explorers_data.explorer2.get_current_planet_index()
            }
            _ => {panic!("Explorer Index out of bounds!")}
        };
        *text = Text::new(basic_resource_type_to_string(generate_resource_on_planet.get_generate(current_planet as u32).get_current_value()));
    }
}

pub fn update_combine_spinner_value(
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    mut query: Query<(&mut Text, &CombineResourceSpinnerValue), With<CombineResourceSpinnerValue>>,
    combine_resource_on_planet: Res<CombinableResourcesOnPlanet>,
    explorers_data: Res<ExplorersData>,
) {

    if check_root.is_empty() {
        return;
    }
    
    if !combine_resource_on_planet.is_changed() {
        return;
    }

    for (mut text, value) in query.iter_mut() {
        let current_planet = match value.explorer_id {
            // This code needs to be duplicated because each query updates the value in both the explorers
            // so the value of the resource cannot be checked outside the loop or in another function
            0 => {
                explorers_data.explorer1.get_current_planet_index()
            }
            1 => {
                explorers_data.explorer2.get_current_planet_index()
            }
            _ => {panic!("Explorer Index out of bounds!")}
        };
        *text = Text::new(complex_resource_type_to_string(combine_resource_on_planet.get_combine(current_planet as u32).get_current_value()));
    }
}

pub fn update_explorers_info(
    asset_server: Res<AssetServer>,
    explorers_data: Res<ExplorersData>,
    check_root: Query<Entity, With<ManualModePanelRoot>>,
    mut queries: ParamSet<(
        Query<
            (&mut Text, &ExplorerCurrentPlanetMarker),
            With<ExplorerCurrentPlanetMarker>
        >,
        Query<
            (&mut Text, &BagViewMarker),
            With<BagViewMarker>
        >,
        Query<
            (&mut ImageNode, &ExplorerSpriteMarker),
            With<ExplorerSpriteMarker>
        >,
    )>,
) {

    if check_root.is_empty() {
        return;
    }

    if !explorers_data.is_changed() {
        return;
    }

    // Update ExplorerCurrentPlanetMarker
    for (mut text, marker) in queries.p0().iter_mut() {
        match marker.explorer_id {
            0 => {
                *text = Text::new((explorers_data.explorer1.get_current_planet_index() + 1).to_string());
            }
            1 => {
                *text = Text::new((explorers_data.explorer2.get_current_planet_index() + 1).to_string());
            }
            _ => { panic!("Explorer Index out of bounds!") }
        }
    }

    // Update BagViewMarker
    for (mut text, marker) in queries.p1().iter_mut() {
        match marker.explorer_id {
            0 => {
                *text = Text::new(explorers_data.explorer1.get_bag().to_string());
            }
            1 => {
                *text = Text::new(explorers_data.explorer2.get_bag().to_string());
            }
            _ => { panic!("Explorer Index out of bounds!") }
        }
    }

    // Update ExplorerSpriteMarker
    for (mut sprite, marker) in queries.p2().iter_mut() {
        match marker.explorer_id {
            0 => {
                if explorers_data.explorer1.is_alive() {
                    sprite.image = asset_server.load(EXPLORER1_ALIVE_PATH)
                } else {
                    sprite.image = asset_server.load(EXPLORER1_DEAD_PATH)
                }
            }
            1 => {
                if explorers_data.explorer2.is_alive() {
                    sprite.image = asset_server.load(EXPLORER2_ALIVE_PATH)
                } else {
                    sprite.image = asset_server.load(EXPLORER2_DEAD_PATH)
                }
            }
            _ => { panic!("Explorer Index out of bounds!") }
        }
    }
}