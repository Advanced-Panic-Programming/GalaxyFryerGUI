use bevy::math::Vec2;

pub const ANIMATION_FPS: u8 = 6;
pub const PLANET_INITIAL_SPLAT: f32 = 3.0;

pub const EXPLORER_ARROW_VERTICAL_OFFSET: f32 = 45.0;
pub const EXPLORER_ARROW_HORIZONTAL_OFFSET: f32 = 10.0; //Applied only if 2 explorer are on the same planet

pub const EXPLORER_ARROW_DIMENSION: f32 = 72.0;
pub const EXPLORER_INITIAL_SPLAT: f32 = 0.5;

pub const GALAXY_MAP_SPRITE_PATH: &str = "galaxy/galaxy_map.png";

// Original 1020x1020
pub const GALAXY_MAP_SIZE: Vec2 = Vec2::new(244.0, 244.0); // Referred to 1920x1080
pub const GALAXY_MAP_X: f32 = 250.0;
pub const GALAXY_MAP_Y: f32 = 400.0;
pub const GALAXY_MAP_Z: f32 = 50.0;