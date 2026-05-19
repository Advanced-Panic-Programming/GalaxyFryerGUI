use bevy::prelude::Message;
use crate::cutscene::CutSceneType;

#[derive(Message)]
pub struct Cutscene(pub CutSceneType);