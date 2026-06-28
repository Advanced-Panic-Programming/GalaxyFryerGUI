use bevy::prelude::*;
use crate::app_states::AppState::{GalaxyView, PlanetView};
use crate::legend::systems::*;
use crate::setup_orchestrator::resources::CurrentOrchestratorMode;

pub struct LegendPlugin;

impl Plugin for LegendPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter systems
            .add_systems(
                OnEnter(GalaxyView), spawn_legend.run_if(resource_exists::<CurrentOrchestratorMode>))
            .add_systems(OnEnter(PlanetView), spawn_legend.run_if(resource_exists::<CurrentOrchestratorMode>))
            // Update systems
            .add_systems(Update, update_legend_mode.run_if(resource_exists::<CurrentOrchestratorMode>))
            // OnExit Systems
            .add_systems(OnExit(GalaxyView), cleanup_legend)
            .add_systems(OnExit(PlanetView), cleanup_legend)
        ;
    }
}