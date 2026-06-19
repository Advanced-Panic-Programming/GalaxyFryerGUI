use bevy::prelude::*;

pub const PANEL_HEIGHT: f32 = 340.0;
pub const TAB_HEIGHT: f32 = 36.0;
/// How many pixels up the orbit center shifts when the manual-mode panel is open.
pub const ORBIT_Y_SHIFT_ACTIVE: f32 = 110.0;

pub const PANEL_BG: Color = Color::srgba(0.04, 0.04, 0.13, 0.95);
pub const TAB_ACTIVE_BG: Color = Color::srgb(0.16, 0.28, 0.52);
pub const TAB_INACTIVE_BG: Color = Color::srgba(0.08, 0.08, 0.18, 1.0);
pub const TAB_BORDER: Color = Color::srgb(0.25, 0.38, 0.65);

pub const BTN_BG: Color = Color::srgb(0.12, 0.22, 0.42);
pub const BTN_HOVER_BG: Color = Color::srgb(0.22, 0.38, 0.65);
pub const BTN_BORDER: Color = Color::srgb(0.35, 0.55, 0.85);

pub const STUB_BTN_BG: Color = Color::srgb(0.18, 0.14, 0.08);
pub const STUB_BTN_BORDER: Color = Color::srgb(0.6, 0.45, 0.15);

pub const TEXT_WHITE: Color = Color::WHITE;
pub const TEXT_LABEL: Color = Color::srgb(0.65, 0.82, 1.0);
pub const TEXT_VALUE: Color = Color::srgb(1.0, 0.88, 0.45);
pub const TEXT_STUB: Color = Color::srgb(0.75, 0.60, 0.25);
pub const TEXT_SECTION: Color = Color::srgb(0.45, 0.60, 0.85);

pub const FONT_PATH: &str = "fonts/FiraMono-Medium.ttf";
pub const FONT_BOLD_PATH: &str = "fonts/FiraSans-Bold.ttf";
pub const FS_SM: f32 = 12.0;
pub const FS_NM: f32 = 14.0;
pub const FS_LG: f32 = 15.0;
