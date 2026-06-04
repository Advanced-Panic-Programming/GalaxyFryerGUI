use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DisplayedLogEntries {
    pub count: usize,
    // pub last_rendered: usize,
}

#[derive(Resource)]
pub struct LogAutoScroll {
    pub enabled: bool,
}

impl Default for LogAutoScroll {
    fn default() -> Self { Self { enabled: true } }
}

#[derive(Resource, Default)]
pub struct PreviousPosition {
    pub previous: f32,
}