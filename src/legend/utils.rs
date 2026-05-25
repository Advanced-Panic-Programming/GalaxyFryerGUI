use bevy::prelude::*;
use bevy::color::Color;

pub const KEYCAP_BACKGROUND_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.4); // transparent
pub const KEYCAP_BORDER_COLOR: Color = Color::WHITE;
pub const KEYCAP_BORDER_RADIUS: f32 = 6.0;
pub const KEYCAP_BORDER_WIDTH: f32 = 2.0;
pub const KEYCAP_SIZE: f32 = 28.0; // It's a square
pub const FONT_PATH: &str = "fonts/MediumPixel.otf";