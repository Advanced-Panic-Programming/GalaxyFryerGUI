use bevy::prelude::*;
use crate::app_states::AppState::PlanetView;
use crate::planet_view::systems::*;

pub struct PlanetViewPlugin;

impl Plugin for PlanetViewPlugin {
    fn build(&self, app: &mut App) {
        app
            // ── OnEnter ──────────────────────────────────────────────────────────
            // `cleanup` runs first so re-entering PlanetView (e.g. Esc → Esc) never
            // leaves stale entities. No `.run_if` here: OnEnter fires exactly once on
            // state transition and Bevy silently ignores run conditions on schedule
            // labels like OnEnter/OnExit.
            .add_systems(
                OnEnter(PlanetView),
                (cleanup, spawn_corner_planet, spawn_planet_view_ui).chain(),
            )
            // ── Update ───────────────────────────────────────────────────────────
            // `respawn_on_planet_change` handles mid-state planet switches; it contains
            // its own `is_changed` guard so it is cheap when nothing has changed.
            .add_systems(
                Update,
                (
                    animate_corner_planet,
                    respawn_on_planet_change,
                )
                    .run_if(in_state(PlanetView)),
            )
            // ── OnExit ───────────────────────────────────────────────────────────
            .add_systems(OnExit(PlanetView), cleanup);
    }
}