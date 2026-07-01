use bevy::prelude::*;
use crate::simulation_end::systems::*;
use crate::app_states::AppState::*;

pub struct SimulationEndPlugin;

impl Plugin for SimulationEndPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SimulationEnd), end_simulation)
        ;
    }
}