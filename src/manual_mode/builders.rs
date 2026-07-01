use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use crate::manual_mode::utils::*;

// This module contains all the builder functions for the ui in order not to cram up the system.rs file
pub(super) fn small_arrow_btn(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: Handle<Font>,
    label: &str,
    marker: impl Bundle,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(24.0),
                height: Val::Px(24.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(BTN_BG),
            BorderColor::all(BTN_BORDER),
            BorderRadius::all(Val::Px(3.0)),
            marker,
        ))
        .with_children(|b| {
            b.spawn((
                Text::new(label),
                TextFont { font, font_size: FS_NM, ..default() },
                TextColor(TEXT_WHITE),
            ));
        });
}

pub(super) fn spinner_value_slot(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: Handle<Font>,
    label: &str,
    marker: impl Bundle,
) {
    parent
        .spawn(Node {
            width: Val::Px(SPINNER_VALUE_WIDTH),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|b| {
            b.spawn((
                Text::new(label),
                TextFont { font, font_size: FS_NM, ..default() },
                TextColor(TEXT_VALUE),
                TextLayout {
                    linebreak: LineBreak::NoWrap,
                    ..default()
                },
                marker,
            ));
        });
}

pub(super) fn action_btn(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: Handle<Font>,
    label: &str,
    bg: Color,
    border: Color,
    marker: impl Bundle,
) {
    parent
        .spawn((
            Button,
            Node {
                padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
                border: UiRect::all(Val::Px(1.5)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg),
            BorderColor::all(border),
            BorderRadius::all(Val::Px(5.0)),
            marker,
        ))
        .with_children(|b| {
            b.spawn((
                Text::new(label),
                TextFont { font, font_size: FS_NM, ..default() },
                TextColor(Color::WHITE),
                TextLayout {
                    linebreak: LineBreak::NoWrap,
                    ..default()
                }
            ));
        });
}