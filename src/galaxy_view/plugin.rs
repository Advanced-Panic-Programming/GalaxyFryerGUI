use bevy::prelude::*;
use galaxy_view::systems::*;
use crate::app_states::AppState::GalaxyView;
use crate::galaxy_view;
use crate::galaxy_view::messages::*;
use crate::setup_simulation::resources::ExplorersData;

pub struct GalaxyViewPlugin;

impl Plugin for GalaxyViewPlugin {
    fn build(&self, app: &mut App) {
        app
            // Messages init
            .add_message::<ReceivedPlanetDestroyed>()
            .add_message::<PlanetDestroyedCutscene>()
            .add_message::<ReceivedAsteroidDestroyed>()
            .add_message::<AsteroidDestroyedCutscene>()
            .add_message::<ReceivedPlanetState>()
            .add_message::<ReceivedPlanetGenerate>()
            .add_message::<ReceivedPlanetCombine>()
            .add_message::<ReceivedExplorerPosition>()
            .add_message::<ReceivedExplorerMove>()
            .add_message::<ReceivedExplorerBag>()
            .add_message::<ReceivedKilledExplorer>()
            .add_message::<ReceivedManualModeAck>()
            .add_message::<ReceivedAutomaticModeAck>()
            .add_message::<ReceivedSimulationEnd>()
            
            // OnEnter Systems
            .add_systems(OnEnter(GalaxyView), (
                spawn_planets,
                setup_explorer_arrow_atlas,
            ))
            // Update Systems in_state
            .add_systems(Update, (
                update_planets_sprites,
                execute_animations,
            ).run_if(in_state(GalaxyView)))
            .add_systems(Update, (
                spawn_explorer_arrows,
                update_explorer_arrow_binding,
                update_explorer_arrow_offsets,
                change_dead_explorers_arrows,
            ).run_if(in_state(GalaxyView))
            )
            // Update Systems (always)
            .add_systems(Update, (
                update_planets_data,
                update_explorer_data
            ))
            // OnExit Systems
            .add_systems(OnExit(GalaxyView), cleanup)
        ;
    }
}