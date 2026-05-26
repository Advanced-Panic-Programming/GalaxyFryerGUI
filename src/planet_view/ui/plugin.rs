use bevy::prelude::*;
use crate::app_states::AppState::{GalaxyView, PlanetView};
use crate::setup_orchestrator::resources::CurrentOrchestratorMode;
use super::systems::*;

pub struct PlanetViewUiPlugin;

impl Plugin for PlanetViewUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter systems
            .add_systems(OnEnter(PlanetView), spawn_planet_view_ui.run_if(resource_exists::<CurrentOrchestratorMode>))
            // Update systems
            .add_systems(Update, spawn_planet_view_ui.run_if(resource_changed::<CurrentOrchestratorMode>).run_if(in_state(PlanetView)))
            // OnExit systems
            .add_systems(OnExit(PlanetView), despawn_planet_view_ui)
        ;
    }
}