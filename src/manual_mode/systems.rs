use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use common_game::components::resource::{BasicResourceType, ComplexResourceType};
use common_game::utils::ID;
use galaxy_fryer::explorer::bag::BagView;
use crate::manual_mode::resources::*;
use crate::manual_mode::components::*;
use crate::manual_mode::builders::*;
use crate::manual_mode::utils::*;
use crate::setup_simulation::resources::ExplorersData;
// ---------------------------------------------------------------
//      Spawn / Despawn
// ---------------------------------------------------------------

pub fn spawn_manual_mode_panel(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    manual_mode_panel: Res<ManualModePanel>,
    planet_spinner: Res<PlanetSpinner>,
    explorers_data: Res<ExplorersData>,
    generatable_resources_on_planet: Res<GeneratableResourcesOnPlanet>,
    combinable_resources_on_planet: Res<CombinableResourcesOnPlanet>,
) {
    let font = asset_server.load(FONT);
    let font_bold = asset_server.load(FONT_BOLD);

    let explorer1_alive_sprite: Handle<Image> = asset_server.load(EXPLORER1_ALIVE_PATH);
    let explorer1_dead_sprite: Handle<Image> = asset_server.load(EXPLORER1_DEAD_PATH);

    let explorer2_alive_sprite: Handle<Image> = asset_server.load(EXPLORER2_ALIVE_PATH);
    let explorer2_dead_sprite: Handle<Image> = asset_server.load(EXPLORER2_DEAD_PATH);

    let explorer1_state = explorers_data.explorer1.is_alive();
    let explorer2_state = explorers_data.explorer2.is_alive();


    let explorer1_bag = explorers_data.explorer1.get_bag();
    let explorer2_bag = explorers_data.explorer2.get_bag();

    let explorer1_current_planet_id = ID::from(explorers_data.explorer1.get_current_planet_index() as u32);
    let explorer2_current_planet_id = ID::from(explorers_data.explorer2.get_current_planet_index() as u32);

    let generate_resource_spinner1 = &generatable_resources_on_planet.get_generate(explorer1_current_planet_id).get_current_value().clone();
    let generate_resource_spinner2 = &generatable_resources_on_planet.get_generate(explorer2_current_planet_id).get_current_value().clone();

    let combine_resource_spinner1 = &combinable_resources_on_planet.get_combine(explorer1_current_planet_id).get_current_value().clone();
    let combine_resource_spinner2 = &combinable_resources_on_planet.get_combine(explorer2_current_planet_id).get_current_value().clone();

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(PANEL_WIDTH),
                height: Val::Px(PANEL_HEIGHT),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(PANEL_BG),
            Visibility::Hidden,
            ManualModePanelRoot,
        ))
        .with_children(|panel| {
            spawn_tab_bar(panel, font_bold.clone());

            // Content area: all tabs sit here; only the active one is visible
            panel
                .spawn(Node {
                    width: Val::Percent(PANEL_WIDTH),
                    flex_grow: 1.0,
                    ..default()
                })
                .with_children(|area| {
                    spawn_galaxy_tab(area, font.clone(), font_bold.clone(), planet_spinner.get_current_value() + 1);
                    spawn_explorer_tab(area, 1, font.clone(), font_bold.clone(),
                                       explorer1_state,
                                       explorer1_alive_sprite,
                                       explorer1_dead_sprite,
                                       explorers_data.explorer1.get_current_planet_index(),
                                       explorer1_bag,
                                       planet_spinner.get_current_value() + 1,
                                       generate_resource_spinner1,
                                       combine_resource_spinner1,
                    );
                    spawn_explorer_tab(area, 2, font.clone(), font_bold.clone(),
                                       explorer2_state,
                                       explorer2_alive_sprite,
                                       explorer2_dead_sprite,
                                       explorers_data.explorer2.get_current_planet_index(),
                                       explorer2_bag,
                                       planet_spinner.get_current_value() + 1,
                                       generate_resource_spinner2,
                                       combine_resource_spinner2,
                    );
                });
        });
}

pub fn despawn_manual_mode_panel(
    mut commands: Commands,
    query: Query<Entity, With<ManualModePanelRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// ---------------------------------------------------------------
//      Tab bar
// ---------------------------------------------------------------
fn spawn_tab_bar(parent: &mut RelatedSpawnerCommands<ChildOf>, font: Handle<Font>) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Px(TAB_HEIGHT),
            flex_direction: FlexDirection::Row,
            border: UiRect::bottom(Val::Px(1.5)),
            ..default()
        })
        .with_children(|bar| {
            for (label, tab) in [
                ("Galaxy", Tab::Galaxy),
                ("Explorer 1", Tab::Explorer1),
                ("Explorer 2", Tab::Explorer2),
            ] {
                bar.spawn((
                    Button,
                    Node {
                        width: Val::Px(130.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::right(Val::Px(1.5)),
                        ..default()
                    },
                    BackgroundColor(TAB_INACTIVE_BG),
                    BorderColor::all(TAB_BORDER),
                    TabButton{ tab },
                ))
                    .with_children(|b| {
                        b.spawn((
                            Text::new(label),
                            TextFont { font: font.clone(), font_size: FS_LG, ..default() },
                            TextColor(Color::WHITE),
                        ));
                    });
            }
        });
}

// ---------------------------------------------------------------
//      Galaxy Tab
// ---------------------------------------------------------------
fn spawn_galaxy_tab(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: Handle<Font>,
    _fb: Handle<Font>,
    planet_spinner_value: usize,
) {
    parent
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(18.0)),
                row_gap: Val::Px(14.0),
                ..default()
            },
            TabContent{ tab: Tab::Galaxy },
        ))
        .with_children(|tab| {

            // Row: target planet selector
            tab.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.0),
                ..default()
            })
                .with_children(|row| {
                    row.spawn((
                        Text::new("Planet"),
                        TextFont { font: font.clone(), font_size: FS_NM, ..default() },
                        TextColor(TEXT_LABEL),
                    ));
                    small_arrow_btn(row, font.clone(), "<", PlanetSpinnerDecrementButton);
                    row.spawn((
                        Text::new(format!("{}", planet_spinner_value)),
                        TextFont { font: font.clone(), font_size: FS_NM, ..default() },
                        TextColor(TEXT_VALUE),
                        PlanetSpinnerValue,
                    ));
                    small_arrow_btn(row, font.clone(), ">", PlanetSpinnerIncrementButton);
                });

            // Row: action buttons
            tab.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(12.0),
                ..default()
            })
                .with_children(|row| {
                    action_btn(row, font.clone(), "Send Sunray", BTN_BG, BTN_BORDER, SendSunrayButton);
                    action_btn(row, font.clone(), "Send Asteroid", BTN_BG, BTN_BORDER, SendAsteroidButton);
                });
        });
}

// ---------------------------------------------------------------
//      Explorers Tab
// ---------------------------------------------------------------

// fn spawn_explorer_tab(
//     parent: &mut RelatedSpawnerCommands<ChildOf>,
//     explorer_id: u32,
//     font: Handle<Font>,
//     fb: Handle<Font>,
//     explorer_current_planet: usize,
//     explorer_bag: &BagView,
//     planet_spinner_value: usize,
//     current_generate: BasicResourceType,
//     current_combine: ComplexResourceType,
// ) {
//     let tab_kind = if explorer_id == 1 { Tab::Explorer1 } else { Tab::Explorer2 };
//
//     parent
//         .spawn((
//             Node {
//                 position_type: PositionType::Absolute,
//                 width: Val::Percent(100.0),
//                 height: Val::Percent(100.0),
//                 flex_direction: FlexDirection::Column,
//                 padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
//                 row_gap: Val::Px(6.0),
//                 ..default()
//             },
//             Visibility::Hidden,
//             TabContent { tab: tab_kind },
//         ))
//         .with_children(|tab| {
//             // Row 1: Current planet
//             tab.spawn(Node {
//                 flex_direction: FlexDirection::Row,
//                 align_items: AlignItems::Center,
//                 column_gap: Val::Px(22.0),
//                 ..default()
//             })
//                 .with_children(|row| {
//                     // Pianeta corrente
//                     row.spawn((
//                         Text::new("Planet "),
//                         TextFont { font: font.clone(), font_size: FS_NM, ..default() },
//                         TextColor(TEXT_LABEL),
//                     ));
//                     row.spawn((
//                         Text::new(explorer_current_planet.to_string()),
//                         TextFont { font: fb.clone(), font_size: FS_NM, ..default() },
//                         TextColor(TEXT_VALUE),
//                         ExplorerCurrentPlanetMarker{ explorer_id },
//                     ));
//                 });
//
//             // Row 2: bag
//             tab.spawn(Node {
//                 flex_direction: FlexDirection::Row,
//                 align_items: AlignItems::Center,
//                 column_gap: Val::Px(10.0),
//                 ..default()
//             })
//                 .with_children(|row| {
//                     row.spawn((
//                         Text::new("Bag "),
//                         TextFont { font: font.clone(), font_size: FS_NM, ..default() },
//                         TextColor(TEXT_LABEL),
//                     ));
//                     row.spawn((
//                         Text::new(explorer_bag.to_string()),
//                         TextFont { font: font.clone(), font_size: FS_NM, ..default() },
//                         TextColor(TEXT_VALUE),
//                         BagViewMarker{ explorer_id },
//                     ));
//                 });
//
//             // Move
//             tab.spawn(Node {
//                 flex_direction: FlexDirection::Row,
//                 align_items: AlignItems::Center,
//                 column_gap: Val::Px(10.0),
//                 ..default()
//             })
//                 .with_children(|row| {
//                     action_btn(row, font.clone(), "Move To Planet", BTN_BG, BTN_BORDER, MoveButton{ explorer_id });
//                     small_arrow_btn(row, font.clone(), "<", PlanetSpinnerDecrementButton);
//                     row.spawn((
//                         Text::new(planet_spinner_value.to_string()),
//                         TextFont { font: fb.clone(), font_size: FS_NM, ..default() },
//                         TextColor(TEXT_VALUE),
//                         PlanetSpinnerValue,
//                     ));
//                     small_arrow_btn(row, font.clone(), ">", PlanetSpinnerIncrementButton);
//                 });
//
//             // Resource Generation
//             tab.spawn(Node {
//                 flex_direction: FlexDirection::Row,
//                 align_items: AlignItems::Center,
//                 column_gap: Val::Px(10.0),
//                 ..default()
//             })
//                 .with_children(|row| {
//                     action_btn(row, font.clone(), "Ask Generatable", BTN_BG, BTN_BORDER, AskGenerateButton { explorer_id });
//
//                     small_arrow_btn(row, font.clone(), "<", GenerateResourceSpinnerDecrementButton);
//                     row.spawn((
//                         Text::new(basic_resource_type_to_string(current_generate)),
//                         TextFont { font: font.clone(), font_size: FS_NM, ..default() },
//                         TextColor(TEXT_VALUE),
//                         GenerateResourceSpinnerValue,
//                     ));
//                     small_arrow_btn(row, font.clone(), ">", GenerateResourceSpinnerIncrementButton);
//
//                     action_btn(row, font.clone(), "Generate", BTN_BG, BTN_BORDER, GenerateButton { explorer_id });
//                 });
//
//             // Resource Combine
//             tab.spawn(Node {
//                 flex_direction: FlexDirection::Row,
//                 align_items: AlignItems::Center,
//                 column_gap: Val::Px(10.0),
//                 ..default()
//             })
//                 .with_children(|row| {
//                     action_btn(row, font.clone(), "Ask Combinable", BTN_BG, BTN_BORDER, AskCombineButton{ explorer_id });
//
//                     small_arrow_btn(row, font.clone(), "<", CombinableResourceSpinnerDecrementButton);
//                     row.spawn((
//                         Text::new(complex_resource_type_to_string(current_combine)),
//                         TextFont { font: font.clone(), font_size: FS_NM, ..default() },
//                         TextColor(TEXT_VALUE),
//                         CombineResourceSpinnerValue,
//                     ));
//                     small_arrow_btn(row, font.clone(), ">", CombinableResourceSpinnerIncrementButton);
//
//                     action_btn(row, font.clone(), "Combine", BTN_BG, BTN_BORDER, CombineButton{ explorer_id });
//                 });
//
//             // Start/Stop AI
//             tab.spawn(Node {
//                 flex_direction: FlexDirection::Row,
//                 align_items: AlignItems::Center,
//                 column_gap: Val::Px(10.0),
//                 ..default()
//             })
//                 .with_children(|row| {
//                     action_btn(row, font.clone(), "Start AI", BTN_BG, BTN_BORDER, StartAIButton{ explorer_id });
//                     action_btn(row, font.clone(), "Stop AI", BTN_BG, BTN_BORDER, StopAIButton{ explorer_id });
//                 });
//
//         });
// }

fn spawn_explorer_tab(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    explorer_id: u32,
    font: Handle<Font>,
    fb: Handle<Font>,
    explorer_state: bool,
    explorer_alive_sprite: Handle<Image>,
    explorer_dead_sprite: Handle<Image>,
    explorer_current_planet: usize,
    explorer_bag: &BagView,
    planet_spinner_value: usize,
    current_generate: &BasicResourceType,
    current_combine: &ComplexResourceType,
) {
    let tab_kind = if explorer_id == 1 {
        Tab::Explorer1
    } else {
        Tab::Explorer2
    };

    parent
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            Visibility::Hidden,
            TabContent { tab: tab_kind },
        ))
        .with_children(|tab| {

            // =========================================================
            // MAIN ROW CONTAINER
            // =========================================================

            tab.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                ..default()
            })
                .with_children(|main_row| {

                    // =====================================================
                    // LEFT COLUMN
                    // =====================================================

                    main_row.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(6.0),
                        flex_grow: 1.0,
                        ..default()
                    })
                        .with_children(|left_column| {

                            // Row 1: Current planet
                            left_column.spawn(Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(22.0),
                                ..default()
                            })
                                .with_children(|row| {
                                    row.spawn((
                                        Text::new("Planet "),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: FS_NM,
                                            ..default()
                                        },
                                        TextColor(TEXT_LABEL),
                                    ));

                                    row.spawn((
                                        Text::new(explorer_current_planet.to_string()),
                                        TextFont {
                                            font: fb.clone(),
                                            font_size: FS_NM,
                                            ..default()
                                        },
                                        TextColor(TEXT_VALUE),
                                        ExplorerCurrentPlanetMarker { explorer_id },
                                    ));
                                });

                            // Row 2: Bag
                            left_column.spawn(Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(10.0),
                                ..default()
                            })
                                .with_children(|row| {
                                    row.spawn((
                                        Text::new("Bag "),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: FS_NM,
                                            ..default()
                                        },
                                        TextColor(TEXT_LABEL),
                                    ));

                                    row.spawn((
                                        Text::new(explorer_bag.to_string()),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: FS_NM,
                                            ..default()
                                        },
                                        TextColor(TEXT_VALUE),
                                        BagViewMarker { explorer_id },
                                    ));
                                });

                            // Move
                            left_column.spawn(Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(10.0),
                                ..default()
                            })
                                .with_children(|row| {
                                    action_btn(
                                        row,
                                        font.clone(),
                                        "Move To Planet",
                                        BTN_BG,
                                        BTN_BORDER,
                                        MoveButton { explorer_id },
                                    );

                                    small_arrow_btn(
                                        row,
                                        font.clone(),
                                        "<",
                                        PlanetSpinnerDecrementButton,
                                    );

                                    row.spawn((
                                        Text::new(planet_spinner_value.to_string()),
                                        TextFont {
                                            font: fb.clone(),
                                            font_size: FS_NM,
                                            ..default()
                                        },
                                        TextColor(TEXT_VALUE),
                                        PlanetSpinnerValue,
                                    ));

                                    small_arrow_btn(
                                        row,
                                        font.clone(),
                                        ">",
                                        PlanetSpinnerIncrementButton,
                                    );
                                });

                            // Resource Generation
                            left_column.spawn(Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(10.0),
                                ..default()
                            })
                                .with_children(|row| {
                                    action_btn(
                                        row,
                                        font.clone(),
                                        "Ask Generatable",
                                        BTN_BG,
                                        BTN_BORDER,
                                        AskGenerateButton { explorer_id },
                                    );

                                    small_arrow_btn(
                                        row,
                                        font.clone(),
                                        "<",
                                        GenerateResourceSpinnerDecrementButton { explorer_id },
                                    );

                                    row.spawn((
                                        Text::new(
                                            basic_resource_type_to_string(current_generate)
                                        ),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: FS_NM,
                                            ..default()
                                        },
                                        TextColor(TEXT_VALUE),
                                        GenerateResourceSpinnerValue {explorer_id},
                                    ));

                                    small_arrow_btn(
                                        row,
                                        font.clone(),
                                        ">",
                                        GenerateResourceSpinnerIncrementButton{explorer_id},
                                    );

                                    action_btn(
                                        row,
                                        font.clone(),
                                        "Generate",
                                        BTN_BG,
                                        BTN_BORDER,
                                        GenerateButton { explorer_id },
                                    );
                                });

                            // Resource Combine
                            left_column.spawn(Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(10.0),
                                ..default()
                            })
                                .with_children(|row| {
                                    action_btn(
                                        row,
                                        font.clone(),
                                        "Ask Combinable",
                                        BTN_BG,
                                        BTN_BORDER,
                                        AskCombineButton { explorer_id },
                                    );

                                    small_arrow_btn(
                                        row,
                                        font.clone(),
                                        "<",
                                        CombinableResourceSpinnerDecrementButton{explorer_id},
                                    );

                                    row.spawn((
                                        Text::new(
                                            complex_resource_type_to_string(current_combine)
                                        ),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: FS_NM,
                                            ..default()
                                        },
                                        TextColor(TEXT_VALUE),
                                        CombineResourceSpinnerValue{explorer_id},
                                    ));

                                    small_arrow_btn(
                                        row,
                                        font.clone(),
                                        ">",
                                        CombinableResourceSpinnerIncrementButton{explorer_id},
                                    );

                                    action_btn(
                                        row,
                                        font.clone(),
                                        "Combine",
                                        BTN_BG,
                                        BTN_BORDER,
                                        CombineButton { explorer_id },
                                    );
                                });

                            // Start/Stop AI
                            left_column.spawn(Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(10.0),
                                ..default()
                            })
                                .with_children(|row| {
                                    action_btn(
                                        row,
                                        font.clone(),
                                        "Start AI",
                                        BTN_BG,
                                        BTN_BORDER,
                                        StartAIButton { explorer_id },
                                    );

                                    action_btn(
                                        row,
                                        font.clone(),
                                        "Stop AI",
                                        BTN_BG,
                                        BTN_BORDER,
                                        StopAIButton { explorer_id },
                                    );
                                });
                        });

                    // =====================================================
                    // RIGHT COLUMN
                    // =====================================================

                    main_row.spawn(Node {
                        width: Val::Px(250.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    })
                        .with_children(|right_column| {

                            let texture = if explorer_state {
                                explorer_alive_sprite.clone()
                            } else {
                                explorer_dead_sprite.clone()
                            };

                            right_column.spawn((
                                ImageNode::new(texture),
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    flex_grow: 1.0,
                                    aspect_ratio: None,
                                    ..default()
                                },
                                ExplorerSpriteMarker { explorer_id },
                            ));

                        });
                });
        });
}