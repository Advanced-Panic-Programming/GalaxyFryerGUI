use std::collections::VecDeque;
use bevy::prelude::*;

use crate::cutscene::CutSceneType;

#[derive(Resource, Default)]
pub struct ActiveCutscene {
    pub current: Option<CutSceneType>,
    /// Whether the visual entities for the active cutscene have already been spawned.
    pub spawned: bool,
    /// Cutscene events queued while another is already playing.
    pub pending: VecDeque<CutSceneType>,
    pub explosion_sound_played: bool,
}

#[derive(Resource)]
pub struct CutsceneTimer(pub Timer);