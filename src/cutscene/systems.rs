use bevy::prelude::{MessageReader, Res, ResMut, Resource, Time, Timer};
use crate::cutscene::messages::Cutscene;
use crate::cutscene::CutSceneType;
use crate::cutscene::resources::ActiveCutscene;

#[derive(Resource)]
pub struct CutsceneTimer(pub Timer);

pub fn handle_cutscene(
    mut messages: MessageReader<Cutscene>,
    mut active_cutscene: ResMut<ActiveCutscene>,
) {
    for message in messages.read() {

        // Ignore new cutscenes if one is already playing
        if active_cutscene.current.is_some() {
            return;
        }

        active_cutscene.current = Some(message.0);
    }
}

pub fn update_cutscene(
    time: Res<Time>,
    mut timer: ResMut<CutsceneTimer>,
    mut active_cutscene: ResMut<ActiveCutscene>,
) {
    let Some(cutscene) = active_cutscene.current else {
        return;
    };

    timer.0.tick(time.delta());

    match cutscene {

        CutSceneType::PlanetDestroyed(id) => {
            // animation/effects here
        }

        CutSceneType::AsteroidDestroyed => {
            // animation/effects here
        }
    }

    if timer.0.is_finished() {

        active_cutscene.current = None;

        timer.0.reset();
    }
}

pub fn no_active_cutscene(
    cutscene: Res<ActiveCutscene>,
) -> bool {
    cutscene.current.is_none()
}