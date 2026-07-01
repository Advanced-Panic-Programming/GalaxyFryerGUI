use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use crate::legend::utils::*;
use crate::legend::bundles::*;
use crate::legend::components::{LegendUI, ModeLegendKey, ModeLegendLabel};
use crate::setup_orchestrator::resources::{CurrentOrchestratorMode, OrchestratorMode};

// Builder functions
fn keycap() -> KeyCapBundle {

    KeyCapBundle {
        node: Node {
            width: Val::Px(KEYCAP_SIZE),
            height: Val::Px(KEYCAP_SIZE),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(KEYCAP_BORDER_WIDTH)),
            ..default()
        },
        background_color: BackgroundColor(KEYCAP_BACKGROUND_COLOR),
        border_color: BorderColor::all(KEYCAP_BORDER_COLOR),
        border_radius: BorderRadius::all(Val::Px(KEYCAP_BORDER_RADIUS)),
    }
}

fn legend_text_bundle(content: &str, font: Handle<Font>) -> LegendTextBundle {
    LegendTextBundle {
        text: Text::new(content),
        text_font: TextFont {
            font,
            font_size: FONT_SIZE,
            ..default()
        },
        text_color: TextColor::WHITE,
    }
}

fn legend_root_node() -> Node {
    Node {
        position_type: PositionType::Absolute,

        top: Val::Px(LEGEND_TOP_POSITION),
        right: Val::Px(LEGEND_RIGHT_POSITION),

        display: Display::Flex,
        flex_direction: FlexDirection::Column,

        row_gap: Val::Px(10.0),
        padding: UiRect::all(Val::Px(8.0)),

        ..default()
    }
}

fn legend_row_node() -> Node {
    Node {
        width: Val::Auto,
        height: Val::Auto, // Val::Px(30.0),
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(8.0),
        ..default()
    }
}

fn legend_row(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    key: &str,
    label: &str,
    font: Handle<Font>,
) {
    parent
        .spawn(legend_row_node())
        .with_children(|row| {
            row.spawn(keycap())
                .with_children(|kc| {
                    kc.spawn(legend_text_bundle(key, font.clone()));
                });

            row.spawn(legend_text_bundle(label, font.clone()));
        });
}

// This function marks the text that needs to be updated so that we can later find it and edit it
fn legend_row_mode(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    key: &str,
    label: &str,
    font: Handle<Font>,
) {
    parent
        .spawn(legend_row_node())
        .with_children(|row| {

            row.spawn(keycap())
                .with_children(|kc| {
                    kc.spawn((
                        legend_text_bundle(key, font.clone()),
                        ModeLegendKey,
                    ));
                });

            row.spawn((
                legend_text_bundle(label, font.clone()),
                ModeLegendLabel,
            ));
        });
}

fn legend_row_planet_range(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: Handle<Font>,
) {
    parent
        .spawn(legend_row_node())
        .with_children(|row| {
            row.spawn(keycap())
                .with_children(|kc| {
                    kc.spawn(legend_text_bundle("1", font.clone()));
                });

            row.spawn(legend_text_bundle("-", font.clone()));

            row.spawn(keycap())
                .with_children(|kc| {
                    kc.spawn(legend_text_bundle("7", font.clone()));
                });

            row.spawn(legend_text_bundle("Planet View", font.clone()));
        });
}

// Spawn/Despawn functions
pub fn spawn_legend(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_mode: Res<CurrentOrchestratorMode>,
) {
    let font = asset_server.load(FONT_PATH);

    let (key, label) = correct_mode_key_name(&current_mode);

    commands
        .spawn((
            LegendUI,
            legend_root_node(),
        ))
        .with_children(|root| {

            legend_row_planet_range(root, font.clone());

            legend_row(
                root,
                "G",
                "Galaxy View",
                font.clone(),
            );

            legend_row(
                root,
                "E",
                "Visit Explorer",
                font.clone(),
            );
            // Special function because the text here needs to be updated during execution
            legend_row_mode(
                root,
                key,
                label,
                font.clone(),
            );

            legend_row(
                root,
                "Esc",
                "Pause",
                font.clone(),
            );
        });
}

// helper function to show the correct input key
fn correct_mode_key_name(
    current_mode: &Res<CurrentOrchestratorMode>
) -> (&'static str, &'static str) {
    match current_mode.mode {
        OrchestratorMode::AutomaticMode => ("M", "Manual Mode"),
        OrchestratorMode::ManualMode => ("A", "Automatic Mode"),
    }
}

pub fn cleanup_legend(
    mut commands: Commands,
    mut query: Query<Entity, With<LegendUI>>,
) {
    for element in query.iter_mut() {
        commands.entity(element).despawn();
    }
}

// Update systems
/// We define a separated system in order to avoid race conditions on the CurrentOrchestratorMode resource
pub fn update_legend_mode(
    current_mode: Res<CurrentOrchestratorMode>,
    // Different queries in order to find the correct text to edit
    mut key_query: Query<&mut Text, (With<ModeLegendKey>, Without<ModeLegendLabel>)>,
    mut text_query: Query<&mut Text, (With<ModeLegendLabel>, Without<ModeLegendKey>)>,
) {
    if !current_mode.is_changed() {
        return;
    }

    let (key, label) = correct_mode_key_name(&current_mode);

    let mut key_entity = key_query.iter_mut();
    let mut label_entity = text_query.iter_mut();

    if let Some(mut key_text) = key_entity.next() {
        *key_text = Text::new(key);
    }

    if let Some(mut label_text) = label_entity.next() {
        *label_text = Text::new(label);
    }
}