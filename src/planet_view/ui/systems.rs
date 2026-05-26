use bevy::prelude::*;
use crate::legend::systems::spawn_legend;
use crate::setup_orchestrator::resources::CurrentOrchestratorMode;
use super::utils::*;
use super::components::*;

pub fn spawn_planet_view_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_mode: ResMut<CurrentOrchestratorMode>,
) {
    commands.spawn(
        (Node { // Transparent full block
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
         PlanetViewUI,
         PlanetViewUiRoot,
        )
    ).with_children(|ui| {
        ui.spawn((Node {
            position_type: PositionType::Absolute,
            top: Val::Px(LEGEND_TOP_POSITION),
            right: Val::Px(LEGEND_RIGHT_POSITION),
            ..default()
        },
        PlanetViewUI
        )).with_children(|legend_box| {
            (spawn_legend(legend_box, &asset_server, current_mode), PlanetViewUI);
        });
    });
}

pub fn despawn_planet_view_ui(
    mut commands: Commands,
    query: Query<Entity, With<PlanetViewUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}