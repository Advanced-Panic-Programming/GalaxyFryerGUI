use bevy::prelude::*;
use crate::app_states::AppState::*;
use crate::manual_mode::resources::*;
use crate::manual_mode::systems::*;

pub struct ManualModePlugin;

impl Plugin for ManualModePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ManualModeState>()
            // Spawn / despawn the panel together with GalaxyView
            .add_systems(OnEnter(GalaxyView), spawn_manual_mode_panel)
            .add_systems(OnExit(GalaxyView), despawn_manual_mode_panel)
            // Panel visibility & tab switching
            .add_systems(
                Update,
                (
                    update_panel_visibility,
                    update_tab_visibility,
                    handle_tab_selector_buttons,
                    update_tab_selector_colors,
                )
                    .run_if(in_state(GalaxyView)),
            )
            // Galaxy tab
            .add_systems(
                Update,
                (
                    handle_galaxy_target_dec,
                    handle_galaxy_target_inc,
                    handle_send_sunray,
                    handle_send_asteroid,
                )
                    .run_if(in_state(GalaxyView)),
            )
            // Explorer tab – Move
            .add_systems(
                Update,
                (
                    handle_move_target_dec,
                    handle_move_target_inc,
                    handle_move_btn,
                )
                    .run_if(in_state(GalaxyView)),
            )
            // Explorer tab – Info stubs
            .add_systems(
                Update,
                (
                    handle_ask_current_planet,
                    handle_ask_energy_cells,
                    handle_refresh_bag,
                )
                    .run_if(in_state(GalaxyView)),
            )
            // Explorer tab – Resource stubs
            .add_systems(
                Update,
                (
                    handle_ask_supported_resources,
                    handle_prev_resource,
                    handle_next_resource,
                    handle_generate_resource,
                )
                    .run_if(in_state(GalaxyView)),
            )
            // Explorer tab – Combination stubs
            .add_systems(
                Update,
                (
                    handle_ask_supported_combinations,
                    handle_prev_combination,
                    handle_next_combination,
                    handle_combine_resource,
                )
                    .run_if(in_state(GalaxyView)),
            )
            // Other tab stubs + label refresh
            .add_systems(
                Update,
                (
                    handle_stop_explorer_ai,
                    handle_start_explorer_ai,
                    update_labels,
                )
                    .run_if(in_state(GalaxyView)),
            )
            // Orbit shift when panel opens/closes
            .add_systems(
                Update,
                adjust_orbit_for_manual_mode.run_if(in_state(GalaxyView)),
            )
            // React to mode ACKs from orchestrator
            .add_systems(Update, (
                handle_active_manual_mode,
                handle_active_automatic_mode,
            ));
    }
}