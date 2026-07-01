use bevy::math::Vec2;

pub const SPACE_BACKGROUND_PATH: &str = "backgrounds/space_bg.png";
pub const ALIVE_PLANET_TERRAIN_SPRITE_PATH: &str = "backgrounds/planet_terrain.png";
pub const DESTROYED_PLANET_TERRAIN_SPRITE_PATH: &str = "backgrounds/destroyed_planet_terrain.png";
pub const EXPLORER1_ALIVE_PATH: &str = "explorers/explorer1_alive.png";
pub const EXPLORER1_DEAD_PATH: &str = "explorers/explorer1_dead_with_rocks.png";
pub const EXPLORER1_ALIVE_ARROW_PATH: &str = "explorer_arrows/red_explorer_arrow.png";
pub const EXPLORER1_DEAD_ARROW_PATH: &str = "explorer_arrows/red_dead_explorer_arrow.png";
pub const EXPLORER2_ALIVE_PATH: &str = "explorers/explorer2_alive.png";
pub const EXPLORER2_DEAD_PATH: &str = "explorers/explorer2_dead_with_rocks.png";
pub const EXPLORER2_ALIVE_ARROW_PATH: &str = "explorer_arrows/green_explorer_arrow.png";
pub const EXPLORER2_DEAD_ARROW_PATH: &str = "explorer_arrows/green_dead_explorer_arrow.png";

pub const EMPTY_BASE_ROCKET_SPRITE_PATH: &str = "rocket/rocket_base.png";
pub const FULL_BUILT_ROCKET_SPRITE_PATH: &str = "rocket/full_built_rocket.png";
pub const FLYING_ROCKET_SPIRTE_PATH: &str = "rocket/flying_rocket.png";

pub const EMPTY_ENERGY_CELL_SPRITE_PATH: &str = "energy_cell/empty_energy_cell.png";
pub const CHARGED_ENERGY_CELL_SPRITE_PATH: &str = "energy_cell/charged_energy_cell.png";

pub const ORBIT_A: f32 = 350.0; // value = 400 -> standard. Tweak to change orbit width.
pub const ORBIT_B: f32 = 220.0; // Tweak to change orbit height

pub const LEFT_MARGIN: f32 = 30.0;

// CONSTS FOR IMAGE SCALING ON DIFFRENT RESOLUTIONS
pub const DESIGN_WIDTH: f32 = 1920.0;
pub const DESIGN_HEIGHT: f32 = 1080.0;
/// Bumps menus/text up a bit further so they read comfortably on a large monitor
pub const UI_SCALE_BOOST: f32 = 1.15; // Change this if the menus feel too small/large
pub fn visible_world_size(window_width: f32, window_height: f32) -> Vec2 {
    let design_aspect = DESIGN_WIDTH / DESIGN_HEIGHT;
    let real_aspect = window_width / window_height;
    if real_aspect >= design_aspect {
        Vec2::new(DESIGN_HEIGHT * real_aspect, DESIGN_HEIGHT)
    } else {
        Vec2::new(DESIGN_WIDTH, DESIGN_WIDTH / real_aspect)
    }
}