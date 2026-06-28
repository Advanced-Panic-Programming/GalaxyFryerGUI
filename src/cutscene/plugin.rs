use bevy::prelude::*;
use crate::app_states::AppState::{GalaxyView, PlanetView};
use crate::cutscene::messages::Cutscene;
use crate::cutscene::resources::{ActiveCutscene, CutsceneTimer};
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
            // Systems (we need chain() because the order of execution matters)
            // 1. Receive message → set ActiveCutscene
            // 2. Spawn visuals once (guarded by ActiveCutscene::spawned)
            // 3. Animate every frame
            // 4. Tick timer, despawn when finished
            .add_systems(
                Update,
                (
                    handle_cutscene,
                    spawn_cutscene_visuals,
                    animate_cutscene,
                    update_cutscene,
                )
                    .chain()
                    .run_if(in_state(GalaxyView).or(in_state(PlanetView))),
            )
            // To clean the screen if the AppState changes
            .add_systems(OnExit(GalaxyView), despawn_cutscene)
            .add_systems(OnExit(PlanetView), despawn_cutscene)
        ;
    }
}