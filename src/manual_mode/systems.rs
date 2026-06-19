use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

use crate::app_state_manager::messages::{ActiveManualMode, ActiveAutomaticMode};
use crate::manual_mode::components::*;
use crate::manual_mode::resources::*;
use crate::manual_mode::utils::*;
use crate::setup_orchestrator::resources::ToOrchestrator;
use crate::setup_simulation::resources::GalaxyOrbit;
use crate::log::resources::{LogMessage, LogLevel::*};
use galaxy_fryer::app::gui_protocol::GUIToOrchestrator;
use crate::app_state_manager::resources::{CurrentMode, Mode};
// ─────────────────────────────────────────────────────────────────────────────
// Spawn / Despawn
// ─────────────────────────────────────────────────────────────────────────────

pub fn spawn_manual_mode_panel(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(FONT_PATH);
    let fb = asset_server.load(FONT_BOLD_PATH);

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(PANEL_HEIGHT),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(PANEL_BG),
            Visibility::Hidden,
            ManualModePanel,
        ))
        .with_children(|panel| {
            spawn_tab_bar(panel, fb.clone());

            // Content area: all tabs sit here; only the active one is visible
            panel
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    ..default()
                })
                .with_children(|area| {
                    spawn_galaxy_tab(area, font.clone(), fb.clone());
                    spawn_explorer_tab(area, 1, font.clone(), fb.clone());
                    spawn_explorer_tab(area, 2, font.clone(), fb.clone());
                    spawn_other_tab(area, font.clone(), fb.clone());
                });
        });
}

pub fn despawn_manual_mode_panel(
    mut commands: Commands,
    query: Query<Entity, With<ManualModePanel>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tab bar
// ─────────────────────────────────────────────────────────────────────────────

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
                ("Galassia", ManualModeTab::Galaxy),
                ("Explorer 1", ManualModeTab::Explorer1),
                ("Explorer 2", ManualModeTab::Explorer2),
                ("Other", ManualModeTab::Other),
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
                    TabSelectorBtn(tab),
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

// ─────────────────────────────────────────────────────────────────────────────
// Galaxy tab
// ─────────────────────────────────────────────────────────────────────────────

fn spawn_galaxy_tab(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: Handle<Font>,
    _fb: Handle<Font>,
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
            TabContent(ManualModeTab::Galaxy),
        ))
        .with_children(|tab| {
            // Section title
            tab.spawn((
                Text::new("── Galassia ─────────────────────────────────"),
                TextFont { font: font.clone(), font_size: FS_SM, ..default() },
                TextColor(TEXT_SECTION),
            ));

            // Row: target planet selector
            tab.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.0),
                ..default()
            })
                .with_children(|row| {
                    row.spawn((
                        Text::new("Pianeta:"),
                        TextFont { font: font.clone(), font_size: FS_NM, ..default() },
                        TextColor(TEXT_LABEL),
                    ));
                    small_arrow_btn(row, font.clone(), "<", GalaxyTargetDec);
                    row.spawn((
                        Text::new("1"),
                        TextFont { font: font.clone(), font_size: FS_NM, ..default() },
                        TextColor(TEXT_VALUE),
                        GalaxyTargetLabel,
                    ));
                    small_arrow_btn(row, font.clone(), ">", GalaxyTargetInc);
                });

            // Row: action buttons
            tab.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(12.0),
                ..default()
            })
                .with_children(|row| {
                    action_btn(row, font.clone(), "Invia Sunray", BTN_BG, BTN_BORDER, SendSunrayBtn);
                    action_btn(row, font.clone(), "Invia Asteroide", BTN_BG, BTN_BORDER, SendAsteroidBtn);
                });
        });
}

// ─────────────────────────────────────────────────────────────────────────────
// Explorer tab
// ─────────────────────────────────────────────────────────────────────────────

fn spawn_explorer_tab(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    id: u32,
    font: Handle<Font>,
    fb: Handle<Font>,
) {
    let tab_kind = if id == 1 { ManualModeTab::Explorer1 } else { ManualModeTab::Explorer2 };

    parent
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                row_gap: Val::Px(6.0),
                ..default()
            },
            Visibility::Hidden,
            TabContent(tab_kind),
        ))
        .with_children(|tab| {
            // ── Row 1: current planet + energy cells ──────────────────────────
            tab.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(22.0),
                ..default()
            })
                .with_children(|row| {
                    // Pianeta corrente
                    row.spawn((
                        Text::new("Pianeta:"),
                        TextFont { font: font.clone(), font_size: FS_NM, ..default() },
                        TextColor(TEXT_LABEL),
                    ));
                    row.spawn((
                        Text::new("?"),
                        TextFont { font: fb.clone(), font_size: FS_NM, ..default() },
                        TextColor(TEXT_VALUE),
                        CurrentPlanetLabel(id),
                    ));
                    // TODO: requires GUIToOrchestrator::AskCurrentPlanet(e_id)
                    stub_btn(row, font.clone(), "Ask Planet [TODO]", AskCurrentPlanetBtn(id));

                    // Spacer
                    row.spawn(Node { flex_grow: 1.0, ..default() });

                    // Celle energia
                    row.spawn((
                        Text::new("Celle:"),
                        TextFont { font: font.clone(), font_size: FS_NM, ..default() },
                        TextColor(TEXT_LABEL),
                    ));
                    row.spawn((
                        Text::new("?"),
                        TextFont { font: fb.clone(), font_size: FS_NM, ..default() },
                        TextColor(TEXT_VALUE),
                        EnergyCellsLabel(id),
                    ));
                    // TODO: requires GUIToOrchestrator::AskEnergyCells(e_id)
                    stub_btn(row, font.clone(), "Ask Celle [TODO]", AskEnergyCellsBtn(id));
                });

            // ── Row 2: bag ────────────────────────────────────────────────────
            tab.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.0),
                ..default()
            })
                .with_children(|row| {
                    row.spawn((
                        Text::new("Bag:"),
                        TextFont { font: font.clone(), font_size: FS_NM, ..default() },
                        TextColor(TEXT_LABEL),
                    ));
                    row.spawn((
                        Text::new("(vuota)"),
                        TextFont { font: font.clone(), font_size: FS_NM, ..default() },
                        TextColor(TEXT_VALUE),
                        BagLabel(id),
                    ));
                    row.spawn(Node { flex_grow: 1.0, ..default() });
                    // TODO: requires GUIToOrchestrator::BagContentRequest(e_id)
                    stub_btn(row, font.clone(), "Refresh Bag [TODO]", RefreshBagBtn(id));
                });

            // ── Section: Move ─────────────────────────────────────────────────
            section_title(tab, font.clone(), "── Muovi Explorer");

            tab.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.0),
                ..default()
            })
                .with_children(|row| {
                    row.spawn((
                        Text::new("Pianeta:"),
                        TextFont { font: font.clone(), font_size: FS_NM, ..default() },
                        TextColor(TEXT_LABEL),
                    ));
                    small_arrow_btn(row, font.clone(), "<", MoveTargetDec(id));
                    row.spawn((
                        Text::new("1"),
                        TextFont { font: fb.clone(), font_size: FS_NM, ..default() },
                        TextColor(TEXT_VALUE),
                        MoveTargetLabel(id),
                    ));
                    small_arrow_btn(row, font.clone(), ">", MoveTargetInc(id));
                    action_btn(row, font.clone(), "Move", BTN_BG, BTN_BORDER, MoveBtn(id));
                });

            // ── Section: Genera risorsa base ──────────────────────────────────
            section_title(tab, font.clone(), "── Genera Risorsa");

            tab.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.0),
                ..default()
            })
                .with_children(|row| {
                    // TODO: requires GUIToOrchestrator::SupportedResourceRequest(e_id)
                    stub_btn(row, font.clone(), "Ask Risorse [TODO]", AskSupportedResourcesBtn(id));

                    small_arrow_btn(row, font.clone(), "<", PrevResourceBtn(id));
                    row.spawn((
                        Text::new("— (chiedi prima)"),
                        TextFont { font: font.clone(), font_size: FS_NM, ..default() },
                        TextColor(TEXT_VALUE),
                        SelectedResourceLabel(id),
                    ));
                    small_arrow_btn(row, font.clone(), ">", NextResourceBtn(id));

                    // TODO: requires GUIToOrchestrator::GenerateResourceRequest { e_id, resource }
                    stub_btn(row, font.clone(), "Generate [TODO]", GenerateResourceBtn(id));
                });

            // ── Section: Combina risorsa complessa ────────────────────────────
            section_title(tab, font.clone(), "── Combina Risorsa");

            tab.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.0),
                ..default()
            })
                .with_children(|row| {
                    // TODO: requires GUIToOrchestrator::SupportedCombinationRequest(e_id)
                    stub_btn(row, font.clone(), "Ask Combo [TODO]", AskSupportedCombinationsBtn(id));

                    small_arrow_btn(row, font.clone(), "<", PrevCombinationBtn(id));
                    row.spawn((
                        Text::new("— (chiedi prima)"),
                        TextFont { font: font.clone(), font_size: FS_NM, ..default() },
                        TextColor(TEXT_VALUE),
                        SelectedCombinationLabel(id),
                    ));
                    small_arrow_btn(row, font.clone(), ">", NextCombinationBtn(id));

                    // TODO: requires GUIToOrchestrator::CombineResourceRequest { e_id, combination }
                    stub_btn(row, font.clone(), "Combine [TODO]", CombineResourceBtn(id));
                });
        });
}

// ─────────────────────────────────────────────────────────────────────────────
// Other tab
// ─────────────────────────────────────────────────────────────────────────────

fn spawn_other_tab(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: Handle<Font>,
    _fb: Handle<Font>,
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
            Visibility::Hidden,
            TabContent(ManualModeTab::Other),
        ))
        .with_children(|tab| {
            section_title(tab, font.clone(), "── AI Control");

            for explorer_id in [1u32, 2] {
                tab.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(12.0),
                    ..default()
                })
                    .with_children(|row| {
                        row.spawn((
                            Text::new(format!("Explorer {explorer_id}:")),
                            TextFont { font: font.clone(), font_size: FS_NM, ..default() },
                            TextColor(TEXT_LABEL),
                        ));
                        // TODO: requires GUIToOrchestrator::StopExplorerAI(e_id)
                        stub_btn(row, font.clone(), "Stop AI [TODO]", StopExplorerAIBtn(explorer_id));
                        // TODO: requires GUIToOrchestrator::StartExplorerAI(e_id)
                        stub_btn(row, font.clone(), "Start AI [TODO]", StartExplorerAIBtn(explorer_id));
                    });
            }

            tab.spawn((
                Text::new("Premi M per uscire dalla modalità manuale"),
                TextFont { font: font.clone(), font_size: FS_SM, ..default() },
                TextColor(TEXT_SECTION),
            ));
        });
}

// ─────────────────────────────────────────────────────────────────────────────
// UI helpers
// ─────────────────────────────────────────────────────────────────────────────

fn action_btn(
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
            ));
        });
}

/// Greyed-out button that signals the feature is not yet wired to the API.
fn stub_btn(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    font: Handle<Font>,
    label: &str,
    marker: impl Bundle,
) {
    action_btn(parent, font, label, STUB_BTN_BG, STUB_BTN_BORDER, marker);
}

fn small_arrow_btn(
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

fn section_title(parent: &mut RelatedSpawnerCommands<ChildOf>, font: Handle<Font>, title: &str) {
    parent.spawn((
        Text::new(title),
        TextFont { font, font_size: FS_SM, ..default() },
        TextColor(TEXT_SECTION),
    ));
}

// ─────────────────────────────────────────────────────────────────────────────
// Visibility updates
// ─────────────────────────────────────────────────────────────────────────────

/// Shows or hides the whole panel based on `ManualModeState.active`.
pub fn update_panel_visibility(
    current_mode: Res<CurrentMode>,
    state: Res<ManualModeState>,
    mut panel_q: Query<&mut Visibility, With<ManualModePanel>>,
) {
    if !current_mode.is_changed() {
        return;
    }

    for mut vis in &mut panel_q {
        *vis = if current_mode.current == Mode::Manual { Visibility::Visible } else { Visibility::Hidden };

        // for mut vis in &mut panel_q {
        //     *vis = if state.active { Visibility::Visible } else { Visibility::Hidden };
        // }
    }
}

    /// Shows the content node that matches the active tab; hides the rest.
    ///
    /// Uses `Visibility::Inherited` (not `Visible`) for the active tab so that it
    /// still inherits `Hidden` from the parent panel when manual mode is off.
    pub fn update_tab_visibility(
        state: Res<ManualModeState>,
        mut tab_q: Query<(&TabContent, &mut Visibility)>,
    ) {
        if !state.is_changed() {
            return;
        }

        for (tab, mut vis) in &mut tab_q {
            *vis = if tab.0 == state.active_tab {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Tab selector button interactions
    // ─────────────────────────────────────────────────────────────────────────────

    pub fn handle_tab_selector_buttons(
        mut state: ResMut<ManualModeState>,
        query: Query<(&Interaction, &TabSelectorBtn), Changed<Interaction>>,
    ) {
        for (interaction, btn) in &query {
            if *interaction == Interaction::Pressed {
                state.active_tab = btn.0;
            }
        }
    }

    pub fn update_tab_selector_colors(
        state: Res<ManualModeState>,
        mut query: Query<(&Interaction, &mut BackgroundColor, &TabSelectorBtn)>,
    ) {
        for (interaction, mut bg, btn) in &mut query {
            *bg = BackgroundColor(match interaction {
                Interaction::Hovered | Interaction::Pressed => BTN_HOVER_BG,
                Interaction::None => {
                    if btn.0 == state.active_tab { TAB_ACTIVE_BG } else { TAB_INACTIVE_BG }
                }
            });
        }
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Galaxy tab button handlers
    // ─────────────────────────────────────────────────────────────────────────────

    pub fn handle_galaxy_target_dec(
        mut state: ResMut<ManualModeState>,
        query: Query<&Interaction, (Changed<Interaction>, With<GalaxyTargetDec>)>,
    ) {
        for interaction in &query {
            if *interaction == Interaction::Pressed {
                state.galaxy_target = if state.galaxy_target <= 1 { 7 } else { state.galaxy_target - 1 };
            }
        }
    }

    pub fn handle_galaxy_target_inc(
        mut state: ResMut<ManualModeState>,
        query: Query<&Interaction, (Changed<Interaction>, With<GalaxyTargetInc>)>,
    ) {
        for interaction in &query {
            if *interaction == Interaction::Pressed {
                state.galaxy_target = if state.galaxy_target >= 7 { 1 } else { state.galaxy_target + 1 };
            }
        }
    }

    pub fn handle_send_sunray(
        state: Res<ManualModeState>,
        query: Query<&Interaction, (Changed<Interaction>, With<SendSunrayBtn>)>,
        sender: Option<Res<ToOrchestrator>>,
    ) {
        for interaction in &query {
            if *interaction == Interaction::Pressed {
                if let Some(s) = &sender {
                    let id = state.galaxy_target;
                    let _ = s.0.send(GUIToOrchestrator::SendSunray { planet_id: id - 1 });
                }
            }
        }
    }

    pub fn handle_send_asteroid(
        state: Res<ManualModeState>,
        query: Query<&Interaction, (Changed<Interaction>, With<SendAsteroidBtn>)>,
        sender: Option<Res<ToOrchestrator>>,
    ) {
        for interaction in &query {
            if *interaction == Interaction::Pressed {
                if let Some(s) = &sender {
                    let id = state.galaxy_target;
                    let _ = s.0.send(GUIToOrchestrator::SendAsteroid { planet_id: id - 1 });
                }
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Explorer tab – Move section
    // ─────────────────────────────────────────────────────────────────────────────

    pub fn handle_move_target_dec(
        mut state: ResMut<ManualModeState>,
        query: Query<(&Interaction, &MoveTargetDec), Changed<Interaction>>,
    ) {
        for (interaction, btn) in &query {
            if *interaction == Interaction::Pressed {
                let e = state.explorer_mut(btn.0);
                e.move_target = if e.move_target <= 1 { 7 } else { e.move_target - 1 };
            }
        }
    }

    pub fn handle_move_target_inc(
        mut state: ResMut<ManualModeState>,
        query: Query<(&Interaction, &MoveTargetInc), Changed<Interaction>>,
    ) {
        for (interaction, btn) in &query {
            if *interaction == Interaction::Pressed {
                let e = state.explorer_mut(btn.0);
                e.move_target = if e.move_target >= 7 { 1 } else { e.move_target + 1 };
            }
        }
    }

    pub fn handle_move_btn(
        state: Res<ManualModeState>,
        query: Query<(&Interaction, &MoveBtn), Changed<Interaction>>,
        sender: Option<Res<ToOrchestrator>>,
    ) {
        for (interaction, btn) in &query {
            if *interaction == Interaction::Pressed {
                if let Some(s) = &sender {
                    let e_id = btn.0;
                    let p_id = state.explorer(e_id).move_target;
                    let _ = s.0.send(GUIToOrchestrator::MoveExplorer { explorer_id: e_id - 1, planet_id: p_id - 1 });
                }
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Explorer tab – Info stubs (Ask Planet, Ask Celle, Refresh Bag)
    // ─────────────────────────────────────────────────────────────────────────────

    pub fn handle_ask_current_planet(
        query: Query<(&Interaction, &AskCurrentPlanetBtn), Changed<Interaction>>,
        sender: Option<Res<ToOrchestrator>>,
    ) {
        for (interaction, btn) in &query {
            if *interaction == Interaction::Pressed {
                // TODO: send GUIToOrchestrator::AskCurrentPlanet(btn.0) once the API is extended
                warn!(
                    "[TODO] AskCurrentPlanet per Explorer {} — estendi GUIToOrchestrator",
                    btn.0
                );
            }
        }
    }

    pub fn handle_ask_energy_cells(
        query: Query<(&Interaction, &AskEnergyCellsBtn), Changed<Interaction>>,
    ) {
        for (interaction, btn) in &query {
            if *interaction == Interaction::Pressed {
                // TODO: send GUIToOrchestrator::AskEnergyCells(btn.0)
                warn!(
                    "[TODO] AskEnergyCells per Explorer {} — estendi GUIToOrchestrator",
                    btn.0
                );
            }
        }
    }

    pub fn handle_refresh_bag(
        query: Query<(&Interaction, &RefreshBagBtn), Changed<Interaction>>,
    ) {
        for (interaction, btn) in &query {
            if *interaction == Interaction::Pressed {
                // TODO: send GUIToOrchestrator::BagContentRequest(btn.0)
                warn!(
                    "[TODO] BagContentRequest per Explorer {} — estendi GUIToOrchestrator",
                    btn.0
                );
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Explorer tab – Resource section stubs
    // ─────────────────────────────────────────────────────────────────────────────

    pub fn handle_ask_supported_resources(
        query: Query<(&Interaction, &AskSupportedResourcesBtn), Changed<Interaction>>,
        sender: Option<Res<ToOrchestrator>>,
    ) {
        for (interaction, btn) in &query {
            if *interaction == Interaction::Pressed {
                if let Some(ref s) = sender {
                    let _ = s.0.send(GUIToOrchestrator::AskAvailableGenerate { explorer_id: btn.0 });
                }
            }
        }
    }

    pub fn handle_prev_resource(
        mut state: ResMut<ManualModeState>,
        query: Query<(&Interaction, &PrevResourceBtn), Changed<Interaction>>,
    ) {
        for (interaction, btn) in &query {
            if *interaction == Interaction::Pressed {
                let e = state.explorer_mut(btn.0);
                if !e.supported_resources.is_empty() {
                    e.resource_cursor = if e.resource_cursor == 0 {
                        e.supported_resources.len() - 1
                    } else {
                        e.resource_cursor - 1
                    };
                }
            }
        }
    }

    pub fn handle_next_resource(
        mut state: ResMut<ManualModeState>,
        query: Query<(&Interaction, &NextResourceBtn), Changed<Interaction>>,
    ) {
        for (interaction, btn) in &query {
            if *interaction == Interaction::Pressed {
                let e = state.explorer_mut(btn.0);
                if !e.supported_resources.is_empty() {
                    e.resource_cursor = (e.resource_cursor + 1) % e.supported_resources.len();
                }
            }
        }
    }

    pub fn handle_generate_resource(
        state: Res<ManualModeState>,
        query: Query<(&Interaction, &GenerateResourceBtn), Changed<Interaction>>,
        sender: Option<Res<ToOrchestrator>>,
    ) {
        for (interaction, btn) in &query {
            if *interaction == Interaction::Pressed {
                let e = state.explorer(btn.0);
                if e.supported_resources.is_empty() {
                    warn!("Unknown — press 'Ask Resources'");
                } else {
                    let resource = e.supported_resources[e.resource_cursor];
                    if let Some(ref s) = sender {
                        let _ = s.0.send(GUIToOrchestrator::AskToGenerate { explorer_id: btn.0, resource });
                    }
                }
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Explorer tab – Combination section stubs
    // ─────────────────────────────────────────────────────────────────────────────

    pub fn handle_ask_supported_combinations(
        query: Query<(&Interaction, &AskSupportedCombinationsBtn), Changed<Interaction>>,
        sender: Option<Res<ToOrchestrator>>,
    ) {
        for (interaction, btn) in &query {
            if *interaction == Interaction::Pressed {
                if let Some(ref s) = sender {
                    let _ = s.0.send(GUIToOrchestrator::AskAvailableCombine { explorer_id: btn.0 });
                }
            }
        }
    }

    pub fn handle_prev_combination(
        mut state: ResMut<ManualModeState>,
        query: Query<(&Interaction, &PrevCombinationBtn), Changed<Interaction>>,
    ) {
        for (interaction, btn) in &query {
            if *interaction == Interaction::Pressed {
                let e = state.explorer_mut(btn.0);
                if !e.supported_combinations.is_empty() {
                    e.combination_cursor = if e.combination_cursor == 0 {
                        e.supported_combinations.len() - 1
                    } else {
                        e.combination_cursor - 1
                    };
                }
            }
        }
    }

    pub fn handle_next_combination(
        mut state: ResMut<ManualModeState>,
        query: Query<(&Interaction, &NextCombinationBtn), Changed<Interaction>>,
    ) {
        for (interaction, btn) in &query {
            if *interaction == Interaction::Pressed {
                let e = state.explorer_mut(btn.0);
                if !e.supported_combinations.is_empty() {
                    e.combination_cursor =
                        (e.combination_cursor + 1) % e.supported_combinations.len();
                }
            }
        }
    }

    pub fn handle_combine_resource(
        state: Res<ManualModeState>,
        query: Query<(&Interaction, &CombineResourceBtn), Changed<Interaction>>,
        sender: Option<Res<ToOrchestrator>>,
    ) {
        for (interaction, btn) in &query {
            if *interaction == Interaction::Pressed {
                let e = state.explorer(btn.0);
                if e.supported_combinations.is_empty() {
                    warn!("Nessuna combo — premi prima 'Ask Combo'");
                } else {
                    let combination = e.supported_combinations[e.combination_cursor];
                    if let Some(ref s) = sender {
                        let _ = s.0.send(GUIToOrchestrator::AskToCombine { explorer_id: btn.0, combine: combination });
                    }
                }
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Other tab stubs
    // ─────────────────────────────────────────────────────────────────────────────

    pub fn handle_stop_explorer_ai(
        query: Query<(&Interaction, &StopExplorerAIBtn), Changed<Interaction>>,
        sender: Option<Res<ToOrchestrator>>,
    ) {
        for (interaction, btn) in &query {
            if *interaction == Interaction::Pressed {
                if let Some(ref s) = sender {
                    let _ = s.0.send(GUIToOrchestrator::StopExplorerAI { explorer_id: btn.0 });
                }
            }
        }
    }

    pub fn handle_start_explorer_ai(
        query: Query<(&Interaction, &StartExplorerAIBtn), Changed<Interaction>>,
        sender: Option<Res<ToOrchestrator>>,
    ) {
        for (interaction, btn) in &query {
            if *interaction == Interaction::Pressed {
                if let Some(ref s) = sender {
                    let _ = s.0.send(GUIToOrchestrator::StartExplorerAI { explorer_id: btn.0 });
                }
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Label refresh
    // ─────────────────────────────────────────────────────────────────────────────

    pub fn update_labels(
        state: Res<ManualModeState>,
        mut galaxy_target: Query<&mut Text, With<GalaxyTargetLabel>>,
        mut curr_planet: Query<(&mut Text, &CurrentPlanetLabel), Without<GalaxyTargetLabel>>,
        mut energy_cells: Query<
            (&mut Text, &EnergyCellsLabel),
            (Without<GalaxyTargetLabel>, Without<CurrentPlanetLabel>),
        >,
        mut bag: Query<
            (&mut Text, &BagLabel),
            (
                Without<GalaxyTargetLabel>,
                Without<CurrentPlanetLabel>,
                Without<EnergyCellsLabel>,
            ),
        >,
        mut move_target: Query<
            (&mut Text, &MoveTargetLabel),
            (
                Without<GalaxyTargetLabel>,
                Without<CurrentPlanetLabel>,
                Without<EnergyCellsLabel>,
                Without<BagLabel>,
            ),
        >,
        mut sel_resource: Query<
            (&mut Text, &SelectedResourceLabel),
            (
                Without<GalaxyTargetLabel>,
                Without<CurrentPlanetLabel>,
                Without<EnergyCellsLabel>,
                Without<BagLabel>,
                Without<MoveTargetLabel>,
            ),
        >,
        mut sel_combo: Query<
            (&mut Text, &SelectedCombinationLabel),
            (
                Without<GalaxyTargetLabel>,
                Without<CurrentPlanetLabel>,
                Without<EnergyCellsLabel>,
                Without<BagLabel>,
                Without<MoveTargetLabel>,
                Without<SelectedResourceLabel>,
            ),
        >,
    ) {
        if !state.is_changed() {
            return;
        }

        for mut t in &mut galaxy_target {
            t.0 = state.galaxy_target.to_string();
        }
        for (mut t, label) in &mut curr_planet {
            t.0 = state.explorer(label.0).current_planet.clone();
        }
        for (mut t, label) in &mut energy_cells {
            t.0 = state.explorer(label.0).energy_cells.clone();
        }
        for (mut t, label) in &mut bag {
            t.0 = state.explorer(label.0).bag.clone();
        }
        for (mut t, label) in &mut move_target {
            t.0 = state.explorer(label.0).move_target.to_string();
        }
        for (mut t, label) in &mut sel_resource {
            t.0 = state.explorer(label.0).selected_resource_name();
        }
        for (mut t, label) in &mut sel_combo {
            t.0 = state.explorer(label.0).selected_combination_name();
        }
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Log panel
    // ─────────────────────────────────────────────────────────────────────────────

    /// Top offset below the legend — the legend sits at top:10, is ~220px tall.
    const LOG_PANEL_TOP: f32 = 240.0;
    const LOG_PANEL_RIGHT: f32 = 10.0;
    const LOG_PANEL_WIDTH: f32 = 260.0;
    /// Visible height of the scrollable log area (pixels per line × MAX_LINES / 2).
    const LOG_SCROLL_HEIGHT: f32 = 120.0;
    const LOG_SCROLL_SPEED: f32 = 12.0;

    // ─────────────────────────────────────────────────────────────────────────────
    // Orbit adjustment
    // ─────────────────────────────────────────────────────────────────────────────

    /// Shifts the galaxy orbit upward when the manual-mode panel is open so that
    /// the lower planets are not hidden behind the panel.
    pub fn adjust_orbit_for_manual_mode(
        state: Res<ManualModeState>,
        mut orbit: ResMut<GalaxyOrbit>,
    ) {
        if !state.is_changed() {
            return;
        }
        orbit.center.y = if state.active { ORBIT_Y_SHIFT_ACTIVE } else { 0.0 };
    }

    // ─────────────────────────────────────────────────────────────────────────────
    // Mode ACK handlers
    // ─────────────────────────────────────────────────────────────────────────────

    /// Reads `ActiveManualMode` (written by AppStateManager after orchestrator ACK)
    /// and opens the manual-mode panel.
    pub fn handle_active_manual_mode(
        mut messages: MessageReader<ActiveManualMode>,
        mut state: ResMut<ManualModeState>,
    ) {
        if !messages.is_empty() {
            messages.clear();
            state.active = true;
        }
    }

    /// Reads `ActiveAutomaticMode` (written by AppStateManager after orchestrator ACK)
    /// and closes the manual-mode panel.
    pub fn handle_active_automatic_mode(
        mut messages: MessageReader<ActiveAutomaticMode>,
        mut state: ResMut<ManualModeState>,
    ) {
        if !messages.is_empty() {
            messages.clear();
            state.active = false;
        }
    }