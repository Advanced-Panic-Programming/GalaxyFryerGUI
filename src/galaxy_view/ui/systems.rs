use bevy::prelude::*;
use crate::legend::systems::*;
use crate::galaxy_view::ui::components::*;
use crate::galaxy_view::ui::utils::*;

pub fn spawn_galaxy_view_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(
        (Node { // Transparent full block
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
        },
         GalaxyViewUI,
         GalaxyViewUiRoot,
        )
    ).with_children(|ui| {
        ui.spawn((Node {
            position_type: PositionType::Absolute,
            top: Val::Px(LEGEND_TOP_POSITION),
            right: Val::Px(LEGEND_RIGHT_POSITION),
            ..default()
        },
        GalaxyViewUI
        )).with_children(|legend_box| {
                (spawn_legend(legend_box, &asset_server), GalaxyViewUI);
            });
    });
}

pub fn despawn_galaxy_view_ui(
    mut commands: Commands,
    query: Query<Entity, With<GalaxyViewUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}