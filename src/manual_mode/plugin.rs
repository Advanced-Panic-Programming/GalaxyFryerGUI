use bevy::prelude::*;
use crate::app_states::AppState;
use crate::app_states::AppState::*;
use crate::manual_mode::resources::*;
use crate::manual_mode::systems::*;
use crate::manual_mode::buttons_logic::*;
use crate::manual_mode::gui_updates::*;

pub struct ManualModePlugin;

impl Plugin for ManualModePlugin {
    fn build(&self, app: &mut App) {
    app
        // Resources
            .init_resource::<ManualModePanel>()
            .init_resource::<PlanetSpinner>()
            .init_resource::<GeneratableResourcesOnPlanet>()
            .init_resource::<CombinableResourcesOnPlanet>()
            // Manual Mode Panel Spawn/Despawn
            .add_systems(OnEnter(GalaxyView), spawn_manual_mode_panel)
            .add_systems(OnExit(GalaxyView), despawn_manual_mode_panel)
            // Tab Handlers
            .add_systems(Update, (
                update_panel_visibility,
                update_selected_tab,
                update_tab_visibility,
                update_tab_selector_colors,
                update_orbit_for_manual_mode,
                ).run_if(in_state(GalaxyView).and(resource_exists::<ManualModePanel>))
            )
            // Galaxy Tab button logic
            .add_systems(Update, (
                    handle_planet_spinner_dec,
                    handle_planet_spinner_inc,
                    handle_send_sunray,
                    handle_send_asteroid,
                ).run_if(in_state(GalaxyView).and(resource_exists::<ManualModePanel>))
            )
            // Galaxy Tab Data Update
            .add_systems(Update, (
                update_planet_spinner_value,
                ).run_if(in_state(GalaxyView).and(resource_exists::<ManualModePanel>))
            )
            // Explorers Tab button logic
            .add_systems(Update, (
                handle_move_to_planet_button,
                handle_ask_generate_button,
                handle_generate_button,
                handle_ask_combine_button,
                handle_combine_button,
                handle_start_ai_button,
                handle_stop_ai_button,
                ).run_if(in_state(GalaxyView).and(resource_exists::<ManualModePanel>))
            )
            // Explorers Data Update
            .add_systems(Update, (
                update_generate_spinner_value,
                update_combine_spinner_value,
                update_explorers_info,
                ).run_if(in_state(GalaxyView))
            )
        ;
    }
}