use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use crate::legend::utils::*;
use crate::legend::bundles::*;

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
        background_color: BackgroundColor { 0: KEYCAP_BACKGROUND_COLOR },
        border_color: BorderColor::all(KEYCAP_BORDER_COLOR),
        border_radius: BorderRadius::all(Val::Px(KEYCAP_BORDER_RADIUS)),
    }
}

fn legend_text_bundle(content: &str, font: Handle<Font>) -> LegendTextBundle {
    LegendTextBundle {
        text: Text::new(content),
        text_font: TextFont {
            font,
            font_size: 18.0,
            ..default()
        },
        text_color: TextColor::WHITE,
    }
}

fn legend_row(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    key: &str,
    label: &str,
    font: Handle<Font>,
) {
    parent
        .spawn(Node {
            width: Val::Auto,
            height: Val::Px(30.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|row| {
            // KEYCAP
            row.spawn(keycap()).with_children(|kc| {
                kc.spawn(legend_text_bundle(key, font.clone()));
            });

            // LABEL TEXT
            row.spawn(legend_text_bundle(label, font.clone()));
        });
}

fn legend_row_planet_range(parent: &mut RelatedSpawnerCommands<ChildOf>, font: Handle<Font>) {
    parent
        .spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|row| {
            // [1]
            row.spawn(keycap()).with_children(|kc| {
                kc.spawn(legend_text_bundle("1", font.clone()));
            });

            // -
            row.spawn(legend_text_bundle("-", font.clone()));

            // [7]
            row.spawn(keycap()).with_children(|kc| {
                kc.spawn(legend_text_bundle("7", font.clone()));
            });

            // Planet View
            row.spawn(legend_text_bundle("Planet View", font.clone()));
        });
}

pub fn spawn_legend(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    asset_server: &Res<AssetServer>,
) {
    let font = asset_server.load(FONT_PATH);

    parent
        .spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        })
        .with_children(|root| {
            legend_row_planet_range(root, font.clone());
            legend_row(root, "G", "Galaxy View", font.clone());
            legend_row(root, "E", "Cycle Explorer", font.clone());
            legend_row(root, "M", "Manual Mode", font.clone());
            legend_row(root, "Esc", "Pause", font.clone());
        });
}