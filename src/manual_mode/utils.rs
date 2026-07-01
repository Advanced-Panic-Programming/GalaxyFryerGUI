use bevy::prelude::*;
use common_game::components::resource::{BasicResourceType, ComplexResourceType};

// Fonts
pub const FONT: &str = "fonts/FiraMono-Medium.ttf";
pub const FONT_BOLD: &str = "fonts/FiraSans-Bold.ttf";
pub const FS_NM: f32 = 14.0;
pub const FS_LG: f32 = 15.0;

// Panel
pub const PANEL_HEIGHT: f32 = 340.0;
pub const PANEL_WIDTH: f32 = 60.0; // Percent
pub const TAB_HEIGHT: f32 = 36.0; // Px
pub const PANEL_BG: Color = Color::srgba(0.05, 0.05, 0.08, 0.80);
pub const PANEL_BORDER_RADIUS: f32 = 12.0;
pub const PANEL_BORDER_COLOR: Color = Color::srgba(0.7, 0.7, 0.7, 0.9);
pub const PANEL_PADDING: f32 = 20.0;
pub const BORDER_WIDTH: f32 = 2.0;
pub const TAB_ACTIVE_BG: Color = Color::srgb(0.35, 0.24, 0.52); // Color::srgb(0.16, 0.28, 0.52);
pub const TAB_INACTIVE_BG: Color = Color::srgb(0.22, 0.17, 0.32); // Color::srgba(0.18, 0.08, 0.18, 1.0);
pub const TAB_BORDER: Color = Color::srgba(0.7, 0.7, 0.7, 0.9); // Color::srgb(0.25, 0.38, 0.65);
pub const TAB_PADDING: f32 = 2.0;
pub const TAB_WIDTH: f32 = 170.0;

pub const EXPLOER_TAB_COLUMNS_GAP: f32 = 250.0; // px

pub const BTN_BG: Color = Color::srgb(0.26, 0.18, 0.40); // Color::srgb(0.12, 0.22, 0.42);
pub const BTN_HOVER_BG: Color = Color::srgb(0.30, 0.22, 0.44); // Color::srgb(0.42, 0.18, 0.42);
pub const BTN_BORDER: Color = Color::WHITE; // Color::srgb(0.35, 0.55, 0.85);

// Text
pub const TEXT_WHITE: Color = Color::WHITE;
pub const TEXT_LABEL: Color = Color::WHITE; // Color::srgb(0.65, 0.82, 1.0);
pub const TEXT_VALUE: Color = Color::WHITE; // Color::srgb(1.0, 0.88, 0.45);

// Others
pub const ORBIT_Y_SHIFT_ACTIVE: f32 = 130.0; // When the panel is active, we shift the planets' orbit up so that they're not covered

pub fn basic_resource_type_to_string(res: Option<&BasicResourceType>) -> String {
    match res {
        Some(BasicResourceType::Silicon) => "Silicon".to_string(),
        Some(BasicResourceType::Oxygen) => "Oxygen".to_string(),
        Some(BasicResourceType::Hydrogen) => "Hydrogen".to_string(),
        Some(BasicResourceType::Carbon) => "Carbon".to_string(),
        None => "".to_string(),
    }
}

pub fn complex_resource_type_to_string(res: Option<&ComplexResourceType>) -> String {
    match res {
        Some(ComplexResourceType::Diamond) => "Diamond".to_string(),
        Some(ComplexResourceType::Water) => "Water".to_string(),
        Some(ComplexResourceType::Life) => "Life".to_string(),
        Some(ComplexResourceType::Robot) => "Robot".to_string(),
        Some(ComplexResourceType::Dolphin) => "Dolphin".to_string(),
        Some(ComplexResourceType::AIPartner) => "AIPartner".to_string(),
        None => "".to_string(),
    }
}