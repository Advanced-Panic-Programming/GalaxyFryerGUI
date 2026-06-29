use bevy::prelude::*;

// Viewport

pub const TERRAIN_SIZE: Vec2 = Vec2::new(1920.0, 1080.0);

// Z-layers (Higher values render on top)

pub const Z_TERRAIN: f32 = -80.0;
pub const Z_ROCKET: f32 = 1.0;
pub const Z_CELLS: f32 = 1.0;
pub const Z_EXPLORER: f32 = 2.0;
pub const Z_BAG_LABEL: f32 = 3.0;

// Corner planet

/// Position of the animated planet in the top-right corner.
pub const CORNER_PLANET_X: f32 = -800.0;
pub const CORNER_PLANET_Y: f32 = 420.0;
pub const CORNER_PLANET_Z: f32 = -50.0;
/// Uniform scale applied to the 72 px atlas frame.
pub const CORNER_PLANET_SCALE: f32 = 3.0;
pub const ANIMATION_FPS: u8 = 12;

// Rocket

// Native sprite: 1024×1024 px (too big).
pub const ROCKET_SIZE: Vec2 = Vec2::new(600.0, 600.0);
pub const ROCKET_POS: Vec3 = Vec3::new(-680.0, -350.0, Z_ROCKET);

// Energy cells

/// Native sprite: 300×300 px. Display at 70×70 px.
pub const CELL_SIZE: Vec2 = Vec2::new(70.0, 70.0);
pub const CELLS_START_X: f32 = -420.0;
/// Cell centre-to-centre distance (display width + 10 px gap).
pub const CELLS_SPACING: f32 = 60.0;
pub const CELLS_Y: f32 = -350.0;

/// X position of the i-th cell.
#[inline]
pub fn cell_x(index: usize) -> f32 {
    CELLS_START_X + index as f32 * CELLS_SPACING
}

//Explorers

pub const EXPLORERS_BAG_FONT_PATH: &str = "fonts/FiraMono-Medium.ttf";

/// Native sprite: 288×288 px. Display at 160×160 px.
pub const EXPLORER_SIZE: Vec2 = Vec2::new(160.0, 160.0);
pub const EXPLORER1_POS: Vec3 = Vec3::new(0.0, -320.0, Z_EXPLORER);
pub const EXPLORER2_POS: Vec3 = Vec3::new(350.0, -320.0, Z_EXPLORER);

/// Vertical offset from sprite centre to bag-label centre (pixels).
pub const BAG_LABEL_OFFSET_Y: f32 = 100.0;
pub const BAG_FONT_SIZE: f32 = 24.0;

/// Position of the bag label given the explorer's world position.
#[inline]
pub fn bag_label_pos(explorer_pos: Vec3) -> Vec3 {
    Vec3::new(
        explorer_pos.x,
        explorer_pos.y + BAG_LABEL_OFFSET_Y,
        Z_BAG_LABEL,
    )
}