use bevy::prelude::*;
use crate::simulation_end::systems::*;
use crate::app_states::AppState::*;

/// This plugin ensures that the application is terminated correctly by using the default 'AppExit' event in Bevy.
/// When you send an AppExit event, Bevy halts the engine and drops the ECS World entirely, which automatically triggers
/// the Drop trait for all managed resources, assets, and components.
pub struct SimulationEndPlugin;

impl Plugin for SimulationEndPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SimulationEnd), end_simulation)
        ;
    }
}