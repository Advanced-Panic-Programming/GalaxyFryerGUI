use bevy::prelude::*;
use bevy::window::WindowResized;
use crate::app_states::AppState;
use crate::app_states::AppState::{GalaxyView, PlanetView, SetupOrchestrator};
use crate::setup_simulation::utils::*;
use crate::setup_simulation::resources::*;
use crate::app_state_manager::messages::SetupSimulationCompleted;

// ======================
// === SetUp Systems ===
// ======================

pub fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
}

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Sprite {
        image: asset_server.load(SPACE_BACKGROUND_PATH),
        custom_size: Some(Vec2::new(1920.0, 1080.0)),
        ..default()
    },
                    Transform::from_translation(Vec3::new(0.0, 0.0, -100.0)),
    ));
}

pub fn init_galaxy_orbit(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.single().unwrap();
    let orbit = compute_orbit_from_window(window);
    commands.insert_resource(orbit);
}

fn compute_orbit_from_window(window: &Window) -> GalaxyOrbit {
    compute_orbit_from_size(window.width(), window.height())
}

fn compute_orbit_from_size(width: f32, _height: f32) -> GalaxyOrbit {

    let total_galaxy_width = 2.0 * ORBIT_A + 175.0;
    let orbit_center_x = - (width / 2.0) + (total_galaxy_width / 2.0) + LEFT_MARGIN;

    // Clamp if the screen is too small — prevent planets from going off-screen
    // let min_x = (-width / 2.0 + LEFT_MARGIN)/2.0;
    // let max_x = 0.0; // don’t move past screen center
    // let safe_center_x = center_x.clamp(min_x, max_x);

    GalaxyOrbit {
        center: Vec3::new(orbit_center_x, 0.0, 0.0),
        a: ORBIT_A,
        b: ORBIT_B,
    }
}

pub fn init_planets_resources(
    mut planets_data: ResMut<PlanetsData>,
) {
    let planets_names = vec![
        "Mercury",
        "Venus",
        "Earth",
        "Mars",
        "Jupiter",
        "Saturn",
        "Uranus",
    ];

    let planets_sprites = vec![
        "planets/planet1.png",
        "planets/planet2.png",
        "planets/planet3.png",
        "planets/planet4.png",
        "planets/planet5.png",
        "planets/planet6.png",
        "planets/planet7.png",
    ];

    let destroyed_planets_sprites = vec![
        "destroyedPlanets/destroyedPlanet1.png",
        "destroyedPlanets/destroyedPlanet2.png",
        "destroyedPlanets/destroyedPlanet3.png",
        "destroyedPlanets/destroyedPlanet4.png",
        "destroyedPlanets/destroyedPlanet5.png",
        "destroyedPlanets/destroyedPlanet6.png",
        "destroyedPlanets/destroyedPlanet7.png",
    ];


    let angles_deg = [90.0, 38.58, 347.16, 295.74, 244.32, 192.9, 141.48];
    let angles: Vec<f32> = angles_deg.iter().map(|deg| (*deg as f32).to_radians()).collect();

    for (i, &angle) in angles.iter().enumerate() {
        {
            planets_data.planets.push(PlanetInfo {
                name: planets_names[i].to_string(),
                index: i,
                alive: true,
                angle,
                speed: 0.5,
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                sprite_path: planets_sprites[i].to_string(),
                destroyed_sprite_path: destroyed_planets_sprites[i].to_string(),
            })
        }
    }
}

pub fn init_explorers_resource(
    mut explorers_data: ResMut<ExplorersData>,
) {
    explorers_data.explorer1 = Explorer::new(0);
    explorers_data.explorer2 = Explorer::new(4);
}

pub fn init_selected_planet_resource(
    mut selected_planet: ResMut<SelectedPlanet>,
) {
    selected_planet.clear();
}

// ======================
// === Update Systems ===
// ======================

pub fn update_orbit_on_window_resized(
    mut resize_events: MessageReader<WindowResized>,
    mut orbit: ResMut<GalaxyOrbit>,
) {
    if !resize_events.is_empty() {
        for event in resize_events.read() {
            let new_orbit = compute_orbit_from_size(event.width, event.height);
            // Keep the same ellipse shape ratio, but update center
            orbit.center = new_orbit.center;
            orbit.a = new_orbit.a;
            orbit.b = new_orbit.b;
        }
    }
}

pub fn finish_simulation_setup(
    mut writer: MessageWriter<SetupSimulationCompleted>,
) {
    writer.write(SetupSimulationCompleted);
}