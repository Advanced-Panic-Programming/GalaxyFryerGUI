use bevy::prelude::*;
use common_game::utils::ID;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    SetupSimulation,
    SetupOrchestrator,
    PauseMenu,
    GalaxyView,
    PlanetView,
    SimulationEnd, // App termination
}