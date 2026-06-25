// use bevy::math::ops::exp;
// use bevy::prelude::*;
// use galaxy_fryer::explorer::bag::BagView;
// use crate::galaxy_view::messages::ReceivedPlanetGenerate;
// use crate::manual_mode_panel::components::*;
// use crate::manual_mode_panel::components::SpinnerKind::ExplorerResource1;
// use crate::manual_mode_panel::resources::*;
// use crate::manual_mode_panel::utils::*;
// use crate::setup_simulation::resources::{ExplorersData, PlanetsData};
// // ═══════════════════════════════════════════════════════════════════════════
// //  SPAWN
// // ═══════════════════════════════════════════════════════════════════════════
//
// /// One-shot system: builds the entire admin panel UI tree.
// pub fn spawn_manual_mode_panel(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     state: Res<ManualModeState>,
//     planets_data: Res<PlanetsData>,
// ) {
//     let font: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
//
//     // ── Root panel ──────────────────────────────────────────────────────
//     commands
//         .spawn((
//             AdminPanel,
//             Node {
//                 position_type: PositionType::Absolute,
//                 bottom: Val::Px(16.0),
//                 left: Val::Px(16.0),
//                 right: Val::Px(16.0),
//                 height: Val::Px(220.0),
//                 flex_direction: FlexDirection::Column,
//                 border: UiRect::all(Val::Px(2.0)),
//                 overflow: Overflow::clip(),
//                 ..default()
//             },
//             BorderColor::all(COLOR_BORDER),
//             BorderRadius::all(Val::Px(12.0)),
//             BackgroundColor(COLOR_PANEL_BG),
//             Visibility::Visible,
//         ))
//         .with_children(|panel| {
//             spawn_tab_bar(panel, font.clone(), &state);
//             spawn_galaxy_tab(panel, font.clone(), &state);
//             spawn_explorer_tab(panel, font.clone(), &state, planets_data);
//         });
// }
//
// // ─────────────────────────────────────────────
// //  Tab bar
// // ─────────────────────────────────────────────
//
// fn spawn_tab_bar(parent: &mut ChildSpawnerCommands, font: Handle<Font>, state: &ManualModeState) {
//     parent
//         .spawn((
//             Node {
//                 flex_direction: FlexDirection::Row,
//                 width: Val::Percent(100.0),
//                 border: UiRect::bottom(Val::Px(1.0)),
//                 ..default()
//             },
//             BorderColor::all(COLOR_SEPARATOR),
//         ))
//         .with_children(|bar| {
//             spawn_tab_button(bar, "Galaxy",   Tab::Galaxy,   font.clone(), state);
//             spawn_tab_button(bar, "Explorer", Tab::Explorer, font.clone(), state);
//         });
// }
//
// fn spawn_tab_button(
//     parent: &mut ChildSpawnerCommands,
//     label: &str,
//     tab: Tab,
//     font: Handle<Font>,
//     state: &ManualModeState,
// ) {
//     let is_active = state.active_tab == tab;
//
//     parent
//         .spawn((
//             Button,
//             TabButton(tab),
//             Node {
//                 padding: UiRect::axes(Val::Px(16.0), Val::Px(6.0)),
//                 ..default()
//             },
//             BackgroundColor(if is_active { COLOR_TAB_ACTIVE } else { COLOR_TAB_INACTIVE }),
//         ))
//         .with_children(|btn| {
//             btn.spawn((
//                 Text::new(label.to_owned()),
//                 TextFont { font, font_size: 13.0, ..default() },
//                 TextColor(COLOR_TEXT),
//             ));
//         });
// }
//
// // ═══════════════════════════════════════════════════════════════════════════
// //  GALAXY TAB
// // ═══════════════════════════════════════════════════════════════════════════
//
// fn spawn_galaxy_tab(
//     parent: &mut ChildSpawnerCommands,
//     font: Handle<Font>,
//     state: &ManualModeState,
// ) {
//     let initial_vis = tab_visibility(Tab::Galaxy, state);
//
//     parent
//         .spawn((
//             TabContent(Tab::Galaxy),
//             Node {
//                 flex_direction: FlexDirection::Row,
//                 flex_grow: 1.0,
//                 align_items: AlignItems::Center,
//                 padding: UiRect::all(Val::Px(8.0)),
//                 width: Val::Percent(100.0),
//                 ..default()
//             },
//             initial_vis,
//         ))
//         .with_children(|tab| {
//             // Column 1 – Planet selector
//             spawn_column(tab, Some("Planet"), font.clone(), |col| {
//                 spawn_spinner(
//                     col,
//                     SpinnerKind::GalaxyPlanet,
//                     &(state.selected_planet + 1).to_string(),
//                     font.clone(),
//                 );
//             });
//
//             // Column 2 – Planet actions
//             spawn_column(tab, Some("Actions"), font.clone(), |col| {
//                 let sunray = spawn_button(col, "Send Sunray", font.clone());
//                 col.commands().entity(sunray).insert(SendSunrayButton);
//
//                 let asteroid = spawn_button(col, "Send Asteroid", font.clone());
//                 col.commands().entity(asteroid).insert(SendAsteroidButton);
//
//                 let update = spawn_button(col, "Update State", font.clone());
//                 col.commands().entity(update).insert(UpdateStateButton);
//             });
//         });
// }
//
// // ═══════════════════════════════════════════════════════════════════════════
// //  EXPLORER TAB
// // ═══════════════════════════════════════════════════════════════════════════
//
// fn spawn_explorer_tab(
//     parent: &mut ChildSpawnerCommands,
//     font: Handle<Font>,
//     state: &ManualModeState,
//     planets_data: Res<PlanetsData>,
// ) {
//     let initial_vis = tab_visibility(Tab::Explorer, state);
//
//     parent
//         .spawn((
//             TabContent(Tab::Explorer),
//             Node {
//                 flex_direction: FlexDirection::Row,
//                 flex_grow: 1.0,
//                 align_items: AlignItems::Center,
//                 padding: UiRect::all(Val::Px(8.0)),
//                 width: Val::Percent(100.0),
//                 ..default()
//             },
//             initial_vis,
//         ))
//         .with_children(|tab| {
//             // Column 1 – Explorer selector + info labels
//             spawn_column(tab, Some("Explorer"), font.clone(), |col| {
//                 spawn_spinner(
//                     col,
//                     SpinnerKind::ExplorerSelector,
//                     &(state.selected_explorer + 1).to_string(),
//                     font.clone(),
//                 );
//
//                 // "Planet: X"
//                 let planet_e = spawn_info_label(col, "Planet: ", "–", font.clone());
//                 col.commands().entity(planet_e).insert(ExplorerPlanetLabel);
//
//                 // "Bag: …"
//                 let bag_e = spawn_info_label(col, "Bag: ", "empty", font.clone());
//                 col.commands().entity(bag_e).insert(ExplorerBagLabel);
//             });
//
//             // Column 2 – Move to planet
//             spawn_column(tab, Some("Move to Planet"), font.clone(), |col| {
//                 // Reuses the same planet-spinner logic (range 1-7) as the Galaxy tab.
//                 spawn_spinner(
//                     col,
//                     SpinnerKind::ExplorerTargetPlanet,
//                     &(state.explorer_target_planet + 1).to_string(),
//                     font.clone(),
//                 );
//                 let mv = spawn_button(col, "Move", font.clone());
//                 col.commands().entity(mv).insert(MoveExplorerButton);
//             });
//
//             // Column 3 – Resource list A
//             spawn_resource_column(
//                 tab,
//                 "Resource A",
//                 SpinnerKind::ExplorerResource1,
//                 // planets_data.planets[manual_mode_state.selected_explorer].get_combine(),
//                 &["TODO"],
//                 state.resource1_index,
//                 font.clone(),
//             );
//
//             // Column 4 – Resource list B
//             spawn_resource_column(
//                 tab,
//                 "Resource B",
//                 SpinnerKind::ExplorerResource2,
//                 &["TODO"],
//                 state.resource2_index,
//                 font.clone(),
//             );
//
//             // Column 5 – AI control
//             spawn_column(tab, Some("AI"), font.clone(), |col| {
//                 let start = spawn_button(col, "Start AI", font.clone());
//                 col.commands().entity(start).insert(StartAiButton);
//
//                 let stop = spawn_button(col, "Stop AI", font.clone());
//                 col.commands().entity(stop).insert(StopAiButton);
//             });
//         });
// }
//
// /// Generic helper: a column containing a spinner over any `&[&str]` resource list.
// fn spawn_resource_column(
//     parent: &mut ChildSpawnerCommands,
//     header: &str,
//     kind: SpinnerKind,
//     list: &[&str],
//     current_index: usize,
//     font: Handle<Font>,
// ) {
//     let display = list.get(current_index).copied().unwrap_or("–").to_owned();
//     spawn_column(parent, Some(header), font.clone(), |col| {
//         spawn_spinner(col, kind, &display, font.clone());
//     });
// }
//
// // ═══════════════════════════════════════════════════════════════════════════
// //  VISIBILITY
// // ═══════════════════════════════════════════════════════════════════════════
//
// /// Toggles the root panel `Visibility` whenever `AdminPanelState::visible` changes.
// pub fn update_panel_visibility(
//     state: Res<ManualModeState>,
//     mut panel_q: Query<&mut Visibility, With<AdminPanel>>,
// ) {
//     if !state.is_changed() {
//         return;
//     }
//
//     for mut vis in &mut panel_q {
//         *vis = if state.visible {
//             Visibility::Visible
//         } else {
//             Visibility::Hidden
//         };
//     }
// }
//
// /// Shows the tab content matching `active_tab`; hides the rest.
// ///
// /// Uses `Visibility::Inherited` (not `Visible`) on the active tab so that the
// /// parent panel's `Hidden` still propagates correctly when the panel is closed.
// pub fn update_tab_visibility(
//     state: Res<ManualModeState>,
//     mut tab_q: Query<(&TabContent, &mut Visibility)>,
// ) {
//     if !state.is_changed() {
//         return;
//     }
//
//     for (tab_content, mut vis) in &mut tab_q {
//         *vis = if tab_content.0 == state.active_tab {
//             Visibility::Inherited
//         } else {
//             Visibility::Hidden
//         };
//     }
// }
//
// /// Highlights the active tab button; dims the inactive one.
// pub fn update_tab_button_style(
//     state: Res<ManualModeState>,
//     mut btn_q: Query<(&TabButton, &mut BackgroundColor)>,
// ) {
//     if !state.is_changed() {
//         return;
//     }
//
//     for (tab_btn, mut bg) in &mut btn_q {
//         *bg = if tab_btn.0 == state.active_tab {
//             COLOR_TAB_ACTIVE.into()
//         } else {
//             COLOR_TAB_INACTIVE.into()
//         };
//     }
// }
//
// // ═══════════════════════════════════════════════════════════════════════════
// //  TAB SWITCHING
// // ═══════════════════════════════════════════════════════════════════════════
//
// /// Listens for clicks on `TabButton` entities and updates `active_tab`.
// pub fn handle_tab_clicks(
//     interaction_q: Query<(&Interaction, &TabButton), Changed<Interaction>>,
//     mut state: ResMut<ManualModeState>,
// ) {
//     for (interaction, tab_btn) in &interaction_q {
//         if *interaction == Interaction::Pressed {
//             state.active_tab = tab_btn.0;
//         }
//     }
// }
//
// // ═══════════════════════════════════════════════════════════════════════════
// //  SPINNER LOGIC  (shared between tabs)
// // ═══════════════════════════════════════════════════════════════════════════
//
// /// Handles all `SpinnerDecrement` / `SpinnerIncrement` button presses.
// pub fn handle_spinner_clicks(
//     decrement_q: Query<(&Interaction, &SpinnerDecrement), Changed<Interaction>>,
//     increment_q: Query<(&Interaction, &SpinnerIncrement), Changed<Interaction>>,
//     mut state: ResMut<ManualModeState>,
// ) {
//     for (interaction, SpinnerDecrement(kind)) in &decrement_q {
//         if *interaction == Interaction::Pressed {
//             decrement_spinner(kind, &mut state);
//         }
//     }
//     for (interaction, SpinnerIncrement(kind)) in &increment_q {
//         if *interaction == Interaction::Pressed {
//             increment_spinner(kind, &mut state);
//         }
//     }
// }
//
// fn decrement_spinner(kind: &SpinnerKind, state: &mut ManualModeState) {
//     let (value, min) = spinner_value_and_bounds_mut(kind, state);
//     if *value > min {
//         *value -= 1;
//     }
// }
//
// fn increment_spinner(kind: &SpinnerKind, state: &mut ManualModeState) {
//     let (value, _min) = spinner_value_and_bounds_mut(kind, state);
//     let max = spinner_max(kind);
//     if *value < max {
//         *value += 1;
//     }
// }
//
// /// Returns a mutable reference to the counter and its minimum bound.
// fn spinner_value_and_bounds_mut<'s>(
//     kind: &SpinnerKind,
//     state: &'s mut ManualModeState,
// ) -> (&'s mut usize, usize) {
//     match kind {
//         SpinnerKind::GalaxyPlanet         => (&mut state.selected_planet,        0),
//         SpinnerKind::ExplorerTargetPlanet => (&mut state.explorer_target_planet, 0),
//         SpinnerKind::ExplorerSelector     => (&mut state.selected_explorer,      0),
//         SpinnerKind::ExplorerResource1    => (&mut state.resource1_index,        0),
//         SpinnerKind::ExplorerResource2    => (&mut state.resource2_index,        0),
//     }
// }
//
// /// Returns the maximum inclusive index for each spinner kind.
// // fn spinner_max(kind: &SpinnerKind) -> usize {
// //     match kind {
// //         SpinnerKind::GalaxyPlanet         => 6,
// //         SpinnerKind::ExplorerTargetPlanet => 6,
// //         SpinnerKind::ExplorerSelector     => 1,
// //         SpinnerKind::ExplorerResource1    => RESOURCE_LIST_1.len().saturating_sub(1),
// //         SpinnerKind::ExplorerResource2    => RESOURCE_LIST_2.len().saturating_sub(1),
// //     }
// // }
//
// // ═══════════════════════════════════════════════════════════════════════════
// //  SPINNER LABEL SYNC
// // ═══════════════════════════════════════════════════════════════════════════
//
// /// Re-renders all spinner value labels whenever `AdminPanelState` changes.
// pub fn sync_spinner_labels(
//     state: Res<ManualModeState>,
//     mut label_q: Query<(&SpinnerLabel, &mut Text)>,
// ) {
//     if !state.is_changed() {
//         return;
//     }
//
//     for (SpinnerLabel(kind), mut text) in &mut label_q {
//         text.0 = spinner_display_value(kind, &state);
//     }
// }
//
// // fn spinner_display_value(kind: &SpinnerKind, state: &ManualModeState) -> String {
// //     match kind {
// //         SpinnerKind::GalaxyPlanet         => (state.selected_planet        + 1).to_string(),
// //         SpinnerKind::ExplorerTargetPlanet => (state.explorer_target_planet + 1).to_string(),
// //         SpinnerKind::ExplorerSelector     => (state.selected_explorer      + 1).to_string(),
// //         SpinnerKind::ExplorerResource1    => {
// //             RESOURCE_LIST_1.get(state.resource1_index).copied().unwrap_or("–").to_owned()
// //         }
// //         SpinnerKind::ExplorerResource2    => {
// //             RESOURCE_LIST_2.get(state.resource2_index).copied().unwrap_or("–").to_owned()
// //         }
// //     }
// // }
//
// // ═══════════════════════════════════════════════════════════════════════════
// //  EXPLORER INFO LABEL SYNC
// // ═══════════════════════════════════════════════════════════════════════════
//
// /// Updates "Planet: X" and "Bag: …" whenever explorer selection or data changes.
// pub fn sync_explorer_labels(
//     state: Res<ManualModeState>,
//     explorers: Res<ExplorersData>,
//     mut planet_q: Query<&mut Text, (With<ExplorerPlanetLabel>, Without<ExplorerBagLabel>)>,
//     mut bag_q:    Query<&mut Text, (With<ExplorerBagLabel>,    Without<ExplorerPlanetLabel>)>,
// ) {
//     if !state.is_changed() && !explorers.is_changed() {
//         return;
//     }
//
//     // In case of wrong index, returns explorer 1
//     let (current_planet, bag_content): (usize, &BagView) = match state.selected_explorer {
//         1 => {
//             (explorers.explorer2.get_current_planet_index(), explorers.explorer2.get_bag())
//         }
//         _ => {
//             (explorers.explorer1.get_current_planet_index(), explorers.explorer1.get_bag())
//         }
//     };
//
//     for mut text in &mut planet_q {
//         text.0 = (current_planet + 1).to_string()
//     }
//
//     for mut text in &mut bag_q {
//         // text.0 = bag_content.resources.to_string(); //TODO Wait for implementation
//     }
// }
//
// // ═══════════════════════════════════════════════════════════════════════════
// //  TEARDOWN
// // ═══════════════════════════════════════════════════════════════════════════
//
// /// Despawns the whole panel tree in one call.
// /// Schedule in `OnExit(AppState::InGame)` or wherever cleanup happens.
// pub fn despawn_admin_panel(
//     mut commands: Commands,
//     panel_q: Query<Entity, With<AdminPanel>>,
// ) {
//     for entity in &panel_q {
//         commands.entity(entity).despawn();
//     }
// }
//
// // ═══════════════════════════════════════════════════════════════════════════
// //  PRIVATE HELPERS
// // ═══════════════════════════════════════════════════════════════════════════
//
// /// Returns the correct initial `Visibility` for a tab content node.
// /// Active tab → `Inherited`; inactive → `Hidden`.
// fn tab_visibility(tab: Tab, state: &ManualModeState) -> Visibility {
//     if state.active_tab == tab {
//         Visibility::Inherited
//     } else {
//         Visibility::Hidden
//     }
// }