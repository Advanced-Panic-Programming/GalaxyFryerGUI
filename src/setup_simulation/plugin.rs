use bevy::prelude::*;
use crate::app_states::AppState::*;
use crate::setup_simulation::resources::*;
use crate::setup_simulation::systems::*;

pub struct SetupSimulationPlugin;

impl Plugin for SetupSimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            // ===== Resources =====
            .insert_resource(PlanetsSpritesData { planets: Vec::new() })
            .insert_resource(PlanetsData { planets: Vec::new() })
            .insert_resource(Galaxy::default())
            .insert_resource(GalaxyOrbit::default())
            .insert_resource(ExplorersData::new())
            .insert_resource(SelectedPlanet::default())
            // ===== OnEnter Setup =====
            .add_systems(OnEnter(SetupSimulation), (
                    spawn_camera,
                    spawn_background,
                    init_galaxy_orbit,
                    init_ui_scale,
                    init_planets_sprites_data_resource,
                    init_planets_data_resource,
                    init_selected_planet_resource,
                    init_planet_terrain_sprites_resource,
                    init_explorer_sprites_resource,
                    init_rocket_sprites_resource,
                    init_energy_cell_sprites_resource,
                    init_button_click_sound_resource,
                    init_explosion_sound_resource,
                    play_background_music,
                ).chain()
            )
            // Update system: sends SetupSimulationCompleted Message -> AppStateManager will change app state
            // Bevy guarantees that "Update" systems will be executed only AFTER the "OnEnter" systems 
            // ===== Update systems =====
            .add_systems(Update, finish_simulation_setup.run_if(in_state(SetupSimulation)))
            // This systems will run even after exiting SetupSimulation
            .add_systems(Update, (
                    update_ui_scale_on_resize,
                    update_background_size_on_resize,
                    play_sound_on_button_press,
                )
            )
        ;
    }
}