use bevy::prelude::*;

#[derive(Component)]
pub struct PauseMenuUI;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct AnimatedLogo;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);