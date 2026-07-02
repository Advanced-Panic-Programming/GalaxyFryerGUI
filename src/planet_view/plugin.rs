use bevy::prelude::*;

use crate::app_states::AppState::PlanetView;
use crate::planet_view::systems::*;

/// This plugin handles the 'PlanetView' state
pub struct PlanetViewPlugin;

impl Plugin for PlanetViewPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter
            .add_systems(
                OnEnter(PlanetView),
                (
                    immediately_ask_planet_state,
                    cleanup,
                    spawn_corner_planet_system,
                    spawn_planet_view_system,
                ).chain(), // chain() guarantees cleanup → spawn_corner_planet → spawn_planet_view in that exact order.
            )
            // Update
            .add_systems(
                Update,
                (
                    // Full respawn when the selected planet changes.
                    // Must run before the other systems so they don't try to query
                    // entities that were just despawned.
                    on_planet_changed,
                    // These systems react to data changes on the current planet.
                    update_terrain_and_rocket,
                    update_energy_cells,
                    update_explorers,
                    // Animation runs every frame regardless of resource changes.
                    animate_corner_planet,
                    // System to periodically update SelectedPlanet PlanetState
                    periodically_ask_planet_state,
                    update_terrain_size_on_resize,
                ).run_if(in_state(PlanetView))
                    // Ensure on_planet_changed always completes before the other
                    // systems run, in order to never operate on stale entities.
                    .chain(),
            )
            // OnExit
            .add_systems(OnExit(PlanetView), cleanup);
    }
}