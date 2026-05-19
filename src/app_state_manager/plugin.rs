use bevy::asset::meta::Settings;
use bevy::prelude::*;

use crate::app_states::*;
use super::messages::*;
use super::systems::*;

pub struct AppStateManagerPlugin;

impl Plugin for AppStateManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Messages init
            .add_message::<SetupSimulationCompleted>()
            .add_message::<SetupOrchestratorCompleted>()
            .add_message::<PlayPressed>()
            .add_message::<PausePressed>()
            .add_message::<GalaxyViewPressed>()
            .add_message::<PlanetViewPressed>()
            .add_message::<SimulationCompleted>()
            .add_message::<ActiveManualMode>()
            .add_message::<ActiveAutomaticMode>()
            // ===== Init State =====
            // .init_state::<AppState>() // Initialize AppStates : Default (SetUpSimulation)

            // Update systems executed in every AppState
            .add_systems(Update, (
                handle_setup_simulation_completed,
                handle_setup_orchestrator_completed,
                handle_play_pressed,
                handle_pause_pressed,
                handle_galaxy_view_pressed,
                handle_planet_view_pressed,
                handle_simulation_completed,
                handle_manual_mode,
                handle_automatic_mode,
            ));
    }
}