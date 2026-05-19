use bevy::prelude::*;

use crate::cutscene::CutSceneType;

#[derive(Resource, Default)]
pub struct ActiveCutscene {
    pub current: Option<CutSceneType>,
}