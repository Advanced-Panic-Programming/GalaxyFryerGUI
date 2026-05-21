use bevy::prelude::*;
use crate::app_states::AppState::PlanetView;
use super::systems::*;

pub struct PlanetViewUiPlugin;

impl Plugin for PlanetViewUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter systems
            .add_systems(OnEnter(PlanetView), spawn_planet_view_ui)
            .add_systems(OnExit(PlanetView), despawn_planet_view_ui)
        ;
    }
}