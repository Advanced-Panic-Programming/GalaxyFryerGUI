use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    SetupOrchestrator,
    SetupSimulation,
    GalaxyView,
    PlanetView,
    PauseMenu,
    CutScene(CutSceneType, u32),
    SimulationEnd,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CutSceneType {
    PlanetDestroyed,
    AsteroidDestroyed,
}