use bevy::prelude::*;
use crate::app_states::AppState::*;
use crate::manual_mode::resources::*;
use crate::manual_mode::systems::*;

pub struct ManualModePlugin;

impl Plugin for ManualModePlugin {
    fn build(&self, app: &mut App) {
    app
        // Resources
            .init_resource::<GeneratableResourcesOnPlanet>()
            .init_resource::<CombinableResourcesOnPlanet>()
        // Update Systems (only in GalaxyView)
        // .add_systems(Update, (
        //
        //     //).run_if(in_state(GalaxyView))
        // )
        ;
    }
}