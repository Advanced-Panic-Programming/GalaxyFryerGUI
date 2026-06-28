//! PlanetViewPlugin — registers all systems with correct scheduling.
//!
//! System ordering guarantees:
//! • OnEnter: `cleanup` always runs before spawn systems (chain).
//! • Update:  patch systems are independent and run in parallel.
//!            `on_planet_changed` is ordered before the patch systems so a
//!            full respawn on the same frame is not immediately overwritten.

use bevy::prelude::*;

use crate::app_states::AppState::PlanetView;
use crate::planet_view::systems::*;

pub struct PlanetViewPlugin;

impl Plugin for PlanetViewPlugin {
    fn build(&self, app: &mut App) {
        app
            // ── OnEnter ───────────────────────────────────────────────────────
            // chain() guarantees cleanup → spawn_corner_planet → spawn_planet_view
            // in that exact order.
            .add_systems(
                OnEnter(PlanetView),
                (
                    cleanup,
                    spawn_corner_planet_system,
                    spawn_planet_view_system,
                )
                    .chain(),
            )
            // ── Update ────────────────────────────────────────────────────────
            .add_systems(
                Update,
                (
                    // Full respawn when the selected planet changes.
                    // Must run before patch systems so they don't try to query
                    // entities that were just despawned.
                    on_planet_changed,
                    // Patch systems — react to data changes on the current planet.
                    // These are no-ops when their respective resource is unchanged.
                    update_terrain_and_rocket,
                    update_energy_cells,
                    update_explorers,
                    // Animation runs every frame regardless of resource changes.
                    animate_corner_planet,
                )
                    .run_if(in_state(PlanetView))
                    // Ensure on_planet_changed always completes before the patch
                    // systems run, so patches never operate on stale entities.
                    .chain(),
            )
            // ── OnExit ────────────────────────────────────────────────────────
            .add_systems(OnExit(PlanetView), cleanup);
    }
}