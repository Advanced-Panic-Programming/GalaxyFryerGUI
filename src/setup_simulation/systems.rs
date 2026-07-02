use std::collections::HashSet;
use bevy::prelude::*;
use bevy::camera::ScalingMode;
use bevy::window::WindowResized;
use crate::setup_simulation::utils::*;
use crate::setup_simulation::resources::*;
use crate::app_state_manager::messages::SetupSimulationCompleted;
use crate::setup_simulation::components::SpaceBackground;
// ======================
// === Camera Systems ===
// ======================

/// Spawns the main 2D camera with a fixed virtual resolution
/// (`DESIGN_WIDTH` x `DESIGN_HEIGHT`). `ScalingMode::AutoMin` shows *at least*
/// that much world space, scaled uniformly to fit any window size
pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: DESIGN_WIDTH,
                min_height: DESIGN_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

/// Sets the initial UI scale from the window at startup. Needed in addition to
/// `update_ui_scale_on_resize` because `main.rs` switches to fullscreen in
/// `PostStartup`, i.e. after this runs — see that system for why UI needs its
/// own scale (unlike the 2D world, it isn't covered by the camera projection).
pub fn init_ui_scale(windows: Query<&Window>, mut ui_scale: ResMut<UiScale>) {
    let Ok(window) = windows.single() else {
        return;
    };
    ui_scale.0 = ui_scale_for_size(window.width(), window.height());
}

/// Keeps the UI scale in sync with the window.
///
/// `bevy_ui` is a render layer entirely separate from the 2D world camera, so
/// the `ScalingMode::AutoMin` projection above has no effect on it: panel,
/// text and button sizes are plain pixel values and stay visually tiny on a
/// big monitor/fullscreen unless something scales them. `UiScale` is Bevy's
/// global multiplier for the whole UI tree; this system keeps it proportional
/// to how much bigger the real window is than the design resolution.
pub fn update_ui_scale_on_resize(
    mut resize_events: MessageReader<WindowResized>,
    mut ui_scale: ResMut<UiScale>,
) {
    let Some(event) = resize_events.read().last() else {
        return;
    };
    ui_scale.0 = ui_scale_for_size(event.width, event.height);
}

/// Uses the smaller of the two axis ratios (like `ScalingMode::AutoMin` would)
/// so the UI never overflows the window on either axis, then applies
/// `UI_SCALE_BOOST` on top so menus read comfortably rather than just barely
/// fitting.
fn ui_scale_for_size(width: f32, height: f32) -> f32 {
    (width / DESIGN_WIDTH).min(height / DESIGN_HEIGHT) * UI_SCALE_BOOST
}

// ======================
// === SetUp Systems ===
// ======================

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>, windows: Query<&Window>){
    let size = windows.single().map(|w| visible_world_size(w.width(), w.height())).unwrap_or(Vec2::new(DESIGN_WIDTH, DESIGN_HEIGHT));

    commands.spawn(
        (
            Sprite {
                image: asset_server.load(SPACE_BACKGROUND_PATH),
                custom_size: Some(size),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, -100.0)),
            SpaceBackground,
        )
    );
}

/// Keeps the space background sized to fully cover the camera's visible area
pub fn update_background_size_on_resize(
    mut resize_events: MessageReader<WindowResized>,
    mut background_query: Query<&mut Sprite, With<SpaceBackground>>,
) {
    let Some(event) = resize_events.read().last() else {
        return;
    };
    let size = visible_world_size(event.width, event.height);
    for mut sprite in background_query.iter_mut() {
        sprite.custom_size = Some(size);
    }
}

pub fn init_galaxy_orbit(mut commands: Commands) {
    commands.insert_resource(compute_orbit_from_size(DESIGN_WIDTH));
}
fn compute_orbit_from_size(width: f32) -> GalaxyOrbit {
    let total_galaxy_width = 2.0 * ORBIT_A + 175.0;
    let orbit_center_x = - (width / 2.0) + (total_galaxy_width / 2.0) + LEFT_MARGIN;
    GalaxyOrbit {
        center: Vec3::new(orbit_center_x, 0.0, 0.0),
        a: ORBIT_A,
        b: ORBIT_B,
    }
}

pub fn init_planets_sprites_data_resource(
    mut planets_data: ResMut<PlanetsSpritesData>,
    asset_server: Res<AssetServer>,
) {
    let planets_sprites = [
        "planets/planet1.png",
        "planets/planet2.png",
        "planets/planet3.png",
        "planets/planet4.png",
        "planets/planet5.png",
        "planets/planet6.png",
        "planets/planet7.png"];

    let destroyed_planets_sprites = [
        "destroyedPlanets/destroyedPlanet1.png",
        "destroyedPlanets/destroyedPlanet2.png",
        "destroyedPlanets/destroyedPlanet3.png",
        "destroyedPlanets/destroyedPlanet4.png",
        "destroyedPlanets/destroyedPlanet5.png",
        "destroyedPlanets/destroyedPlanet6.png",
        "destroyedPlanets/destroyedPlanet7.png"];


    let angles_deg = [90.0, 38.58, 347.16, 295.74, 244.32, 192.9, 141.48];
    let angles: Vec<f32> = angles_deg.iter().map(|deg| (*deg as f32).to_radians()).collect();

    for (i, &angle) in angles.iter().enumerate() {
        {
            planets_data.planets.push(PlanetSpriteInfo {
                index: i,
                alive: true,
                angle,
                speed: 0.5,
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                alive_sprite: asset_server.load(planets_sprites[i]),
                destroyed_sprite: asset_server.load(destroyed_planets_sprites[i]),
            })
        }
    }
}

/// Initializes the actual planets' data. Hard coded as in the orchestrator
pub fn init_planets_data_resource(
    mut planets_data: ResMut<PlanetsData>,
) {
    // Planet 1 - Rustrelli - D
    let rustrelli = PlanetInfo::new(true, Vec::new(),0, false, false, HashSet::new(), HashSet::new());
    planets_data.planets.push(rustrelli);
    
    // Planet 2 - Houston we have a borrow
    let huston_we_have_a_borrow = PlanetInfo::new(true, Vec::new(),0, true, false, HashSet::new(), HashSet::new());
    planets_data.planets.push(huston_we_have_a_borrow);
    
    // Planet 3 - Enterprise - C
    let entrerprise = PlanetInfo::new(true, Vec::new(),0, true, false, HashSet::new(), HashSet::new());
    planets_data.planets.push(entrerprise);
    
    // Planet 4 - One-Million-Crabs - D
    let one_million_crabs = PlanetInfo::new(true, Vec::new(),0, false, false, HashSet::new(), HashSet::new());
    planets_data.planets.push(one_million_crabs);
    
    // Planet 5 - Rusty Crab - C
    let rusty_crab = PlanetInfo::new(true, Vec::new(),0, true, false, HashSet::new(), HashSet::new());
    planets_data.planets.push(rusty_crab);
    
    // Planet 6 - Orbitron - D
    let orbitron = PlanetInfo::new(true, Vec::new(),0, false, false, HashSet::new(), HashSet::new());
    planets_data.planets.push(orbitron);
    
    // Planet 7 - Trip - A
    let trip = PlanetInfo::new(true, Vec::new(),0, true, false, HashSet::new(), HashSet::new());
    planets_data.planets.push(trip);
}

pub fn init_selected_planet_resource(
    mut selected_planet: ResMut<SelectedPlanet>,
) {
    selected_planet.clear();
}

pub fn init_planet_terrain_sprites_resource(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let terrain = PlanetTerrainSpriteInfo {
        alive: asset_server.load(ALIVE_PLANET_TERRAIN_SPRITE_PATH),
        destroyed: asset_server.load(DESTROYED_PLANET_TERRAIN_SPRITE_PATH) 
    };
    
    commands.insert_resource(PlanetTerrainSpriteData {
        terrain,
    })
}

// ==============================
// === Explorer Resource Init ===
// ==============================

pub fn init_explorer_sprites_resource(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let explorer1 = ExplorerSpriteInfo {
        alive_sprite: asset_server.load(EXPLORER1_ALIVE_PATH),
        dead_sprite: asset_server.load(EXPLORER1_DEAD_PATH),
        alive_arrow_sprite: asset_server.load(EXPLORER1_ALIVE_ARROW_PATH),
        dead_arrow_sprite: asset_server.load(EXPLORER1_DEAD_ARROW_PATH),
    };

    let explorer2 = ExplorerSpriteInfo {
        alive_sprite: asset_server.load(EXPLORER2_ALIVE_PATH),
        dead_sprite: asset_server.load(EXPLORER2_DEAD_PATH),
        alive_arrow_sprite: asset_server.load(EXPLORER2_ALIVE_ARROW_PATH),
        dead_arrow_sprite: asset_server.load(EXPLORER2_DEAD_ARROW_PATH),
    };

    commands.insert_resource(ExplorerSpriteData {
        explorer1,
        explorer2,
    }); // HERE the resource is added to the system
}

pub fn init_rocket_sprites_resource(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let rocket = RocketSpriteInfo {
        full_built_rocket: asset_server.load(FULL_BUILT_ROCKET_SPRITE_PATH),
        empty_rocket_base: asset_server.load(EMPTY_BASE_ROCKET_SPRITE_PATH),
        flying_rocket: asset_server.load(FLYING_ROCKET_SPIRTE_PATH),
    };
    commands.insert_resource(RocketSpritesData {
        rocket,
    })
}

pub fn init_energy_cell_sprites_resource(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let energy_cell = EnergyCellsSpriteInfo {
        empty: asset_server.load(EMPTY_ENERGY_CELL_SPRITE_PATH),
        charged: asset_server.load(CHARGED_ENERGY_CELL_SPRITE_PATH),
    };
    commands.insert_resource(EnergyCellsSpritesData {
        energy_cell,
    })
}

pub fn play_background_music(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn((
        AudioPlayer::new(asset_server.load(SOUNDTRACK_PATH)),
        PlaybackSettings::LOOP,
    ));
}

// SetupSimulationEnd
pub fn finish_simulation_setup(
    mut writer: MessageWriter<SetupSimulationCompleted>,
) {
    writer.write(SetupSimulationCompleted);
}