use bevy::prelude::*;
use crate::app_states::AppState::PlanetView;
use crate::planet_view::systems::*;

pub struct PlanetViewPlugin;

impl Plugin for PlanetViewPlugin {
    fn build(&self, app: &mut App) {
        app
        // OnEnter systems
        .add_systems(OnEnter(PlanetView), (
            refresh_planet_view, // Needed for: PlanetView -> Esc -> Esc to redraw
        ))
        // Update systems
        .add_systems(Update, (
            refresh_planet_view,
            animate_corner_planet,
        ).run_if(in_state(PlanetView)))
        // OnExit Systems
        .add_systems(OnExit(PlanetView), cleanup)
        ;
    }
}