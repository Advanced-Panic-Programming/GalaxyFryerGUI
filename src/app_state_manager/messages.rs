use bevy::prelude::*;
use crate::cutscene::CutSceneType;

// AppStates
#[derive(Message)]
pub struct SetupSimulationCompleted;
#[derive(Message)]
pub struct SetupOrchestratorCompleted;
#[derive(Message)]
pub struct PlayPressed;
#[derive(Message)]
pub struct PausePressed;
#[derive(Message)]
pub struct ExitPressed;
#[derive(Message)]
pub struct GalaxyViewPressed;
#[derive(Message)]
pub struct PlanetViewPressed;
#[derive(Message)]
pub struct ActiveManualMode;
#[derive(Message)]
pub struct ActiveAutomaticMode;