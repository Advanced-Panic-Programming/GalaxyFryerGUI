use std::time::Duration;
use bevy::prelude::*;

#[derive(Component)]
pub struct SpawnedByGalaxyView;

#[derive(Component)]
pub struct Planet {
    pub index: usize,
}

#[derive(Component)]
pub struct ExplorerArrow {
    pub planet_index: usize,
    pub explorer_id: usize, // 1 or 2
}

#[derive(Component)]
pub struct AnimationConfig {
    pub(crate) first_sprite_index: usize,
    pub(crate) last_sprite_index: usize,
    pub(crate) frame_timer: Timer,
}

impl AnimationConfig {
    pub(crate) fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Repeating)
    }
}
