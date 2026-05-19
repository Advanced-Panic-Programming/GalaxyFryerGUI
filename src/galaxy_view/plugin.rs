use bevy::prelude::*;
use galaxy_view::ui::plugin::GalaxyViewUiPlugin;
use galaxy_view::systems::*;
use crate::app_states::AppState::GalaxyView;
use crate::galaxy_view;
use crate::galaxy_view::messages::*;

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
            .add_message::<ReceivedExplorerMove>()
            .add_message::<ReceivedExplorerBag>()
            .add_message::<ReceivedKilledExplorer>()
            .add_message::<ReceivedSimulationEnd>()

            // Plugins
            .add_plugins(GalaxyViewUiPlugin)
            // OnEnter Systems
            .add_systems(OnEnter(GalaxyView), (
                spawn_planets,
            ))
            // Update Systems
            .add_systems(Update, (
                update_planets,
                execute_animations,
                bound_explorer_arrows,
                // update_explorer_arrows_planet_binding,
                // animate_explorer_arrows,
            ).run_if(in_state(GalaxyView)))
            // OnExit Systems
            .add_systems(OnExit(GalaxyView), cleanup)
        ;
    }
}