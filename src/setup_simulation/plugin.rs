use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};
use crate::app_states::AppState;
use crate::app_states::AppState::*;
use crate::setup_simulation::resources::*;
use crate::setup_simulation::systems::*;

pub struct SetupSimulationPlugin;

impl Plugin for SetupSimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            // ===== Plugins ===== (MUST be before everything else)
            .add_plugins(DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::Windowed,
                        resolution: WindowResolution::new(1920, 1080),
                        title: "AirFryer".to_string(),
                        ..default()
                    }),
                    ..default()
                })
            )
            // ===== Init State =====
            .init_state::<AppState>() // Initialize AppStates : Default -> Setup
            // ===== Resources =====
            .insert_resource(PlanetsData { planets: Vec::new() })
            .insert_resource(Galaxy::default())
            .insert_resource(GalaxyOrbit::default())
            .insert_resource(ExplorersData::default())
            .insert_resource(SelectedPlanet::default())
            .insert_resource(LastState::default())
            // ===== OnEnter Setup =====
            .add_systems(OnEnter(SetupSimulation), (
                    spawn_camera,
                    spawn_background,
                    init_galaxy_orbit,
                    init_planets_resources,
                    init_explorers_resource,
                    init_selected_planet_resource,
                )
            )
            // ===== When Setup finishes -> go to GalaxyView ===== 
            .add_systems(Update, set_galaxy_view_state.run_if(in_state(SetupSimulation))
            )
            // ===== Normal Update systems =====
            .add_systems(Update, update_orbit_on_window_resized) // Runs even if the app is in a different state
        ;
    }
}