use bevy::log::tracing::Instrument;
use bevy::prelude::*;
use crate::simulation_end::systems::*;
use crate::app_states::AppState::*;

pub struct SimulationEndPlugin;

impl Plugin for SimulationEndPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, end_simulation.run_if(in_state(SimulationEnd)))
        ;
    }
}