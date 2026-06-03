use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DisplayedLogEntries {
    pub count: usize,
    // pub last_rendered: usize,
}

#[derive(Resource, Default)]
pub struct LogAutoScroll {
    pub enabled: bool,
}

#[derive(Resource, Default)]
pub struct PreviousPosition {
    pub previous: f32,
}