use bevy::prelude::*;
use crate::app_states::AppState::GalaxyView;
use crate::galaxy_view::ui::systems::*;
use crate::setup_orchestrator::resources::CurrentOrchestratorMode;

pub struct GalaxyViewUiPlugin;

impl Plugin for GalaxyViewUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter systems
            .add_systems(OnEnter(GalaxyView), spawn_galaxy_view_ui.run_if(resource_exists::<CurrentOrchestratorMode>))
            // Update systems
            .add_systems(Update, spawn_galaxy_view_ui.run_if(resource_changed::<CurrentOrchestratorMode>).run_if(in_state(GalaxyView)))//TODO! Check correctness
            // OnExit systems
            .add_systems(OnExit(GalaxyView), despawn_galaxy_view_ui)
        ;
    }
}