use bevy::prelude::*;
use crate::cutscene::messages::Cutscene;
use crate::cutscene::resources::ActiveCutscene;
use crate::cutscene::systems::*;

pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app
            // Message
            .add_message::<Cutscene>()
            // Resources
            .init_resource::<ActiveCutscene>()
            .insert_resource(
                CutsceneTimer(
                    Timer::from_seconds(
                        3.0,
                        TimerMode::Once,
                    )
                )
            )
            // Systems
            .add_systems(Update, (
                handle_cutscene,
                update_cutscene,
            ));
    }
}