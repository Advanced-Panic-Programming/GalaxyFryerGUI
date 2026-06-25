// use bevy::prelude::*;
// use crate::manual_mode_panel::components::{SpinnerDecrement, SpinnerIncrement, SpinnerKind, SpinnerLabel};
//
// // ─────────────────────────────────────────────
// //  Colour palette
// // ─────────────────────────────────────────────
//
// pub const COLOR_PANEL_BG:     Color = Color::srgba(0.08, 0.08, 0.12, 0.92);
// pub const COLOR_BORDER:       Color = Color::WHITE;
// pub const COLOR_TAB_ACTIVE:   Color = Color::srgba(0.25, 0.25, 0.40, 1.0);
// pub const COLOR_TAB_INACTIVE: Color = Color::srgba(0.12, 0.12, 0.18, 1.0);
// pub const COLOR_BUTTON:       Color = Color::srgba(0.20, 0.20, 0.32, 1.0);
// pub const COLOR_BUTTON_HOVER: Color = Color::srgba(0.30, 0.30, 0.48, 1.0);
// pub const COLOR_TEXT:         Color = Color::WHITE;
// pub const COLOR_LABEL:        Color = Color::srgba(0.75, 0.75, 0.90, 1.0);
// pub const COLOR_SEPARATOR:    Color = Color::srgba(0.30, 0.30, 0.45, 1.0);
//
// // ─────────────────────────────────────────────
// //  Generic button builder
// // ─────────────────────────────────────────────
//
// /// Spawns a styled text button as a child of `parent`.
// ///
// /// Returns the button's `Entity` so the caller can attach marker components via
// /// `commands.entity(id).insert(MyMarker)`.  The text label is spawned as a child
// /// automatically; no closure is needed for the common case.
// ///
// /// # Example
// /// ```rust
// /// let id = spawn_button(col, "Send Sunray", font.clone(), commands);
// /// commands.entity(id).insert(SendSunrayButton);
// /// ```
// pub fn spawn_button(
//     parent: &mut ChildSpawnerCommands,
//     label: &str,
//     font: Handle<Font>,
// ) -> Entity {
//     let label = label.to_owned();
//
//     // `spawn` on a ChildSpawnerCommands returns a ChildBuilder-scoped EntityCommands.
//     // We grab the Entity *before* calling with_children so we never hold a
//     // reference across the closure boundary.
//     let entity = parent
//         .spawn((
//             Button,
//             Node {
//                 padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
//                 margin: UiRect::all(Val::Px(4.0)),
//                 border: UiRect::all(Val::Px(1.0)),
//                 justify_content: JustifyContent::Center,
//                 align_items: AlignItems::Center,
//                 ..default()
//             },
//             BorderColor::all(COLOR_SEPARATOR),
//             BorderRadius::all(Val::Px(6.0)),
//             BackgroundColor(COLOR_BUTTON),
//         ))
//         .id(); // <-- capture Entity here, before any further borrows
//
//     // Re-borrow to add the text child.
//     parent.commands().entity(entity).with_children(|btn| {
//         btn.spawn((
//             Text::new(label),
//             TextFont { font, font_size: 13.0, ..default() },
//             TextColor(COLOR_TEXT),
//         ));
//     });
//
//     entity
// }
//
// // ─────────────────────────────────────────────
// //  Column builder
// // ─────────────────────────────────────────────
//
// /// Spawns a flex-column container as a child of `parent`, with an optional
// /// header label, and runs `children_fn` to populate the column body.
// ///
// /// Accepts a closure instead of returning `EntityCommands` to avoid the
// /// "returns value referencing temporary" lifetime issue.
// ///
// /// # Example
// /// ```rust
// /// spawn_column(tab, Some("Actions"), font.clone(), |col| {
// ///     let id = spawn_button(col, "Send Sunray", font.clone());
// ///     col.commands().entity(id).insert(SendSunrayButton);
// /// });
// /// ```
// pub fn spawn_column(
//     parent: &mut ChildSpawnerCommands,
//     header: Option<&str>,
//     font: Handle<Font>,
//     children_fn: impl FnOnce(&mut ChildSpawnerCommands),
// ) {
//     parent
//         .spawn((
//             Node {
//                 flex_direction: FlexDirection::Column,
//                 align_items: AlignItems::Center,
//                 padding: UiRect::all(Val::Px(6.0)),
//                 margin: UiRect::axes(Val::Px(4.0), Val::Px(0.0)),
//                 border: UiRect::right(Val::Px(1.0)),
//                 ..default()
//             },
//             BorderColor::all(COLOR_SEPARATOR),
//         ))
//         .with_children(|col| {
//             // Optional section header
//             if let Some(title) = header {
//                 col.spawn((
//                     Text::new(title.to_owned()),
//                     TextFont { font: font.clone(), font_size: 11.0, ..default() },
//                     TextColor(COLOR_LABEL),
//                     Node {
//                         margin: UiRect::bottom(Val::Px(4.0)),
//                         ..default()
//                     },
//                 ));
//             }
//
//             // Caller-supplied body
//             children_fn(col);
//         });
// }
//
// // ─────────────────────────────────────────────
// //  Spinner widget builder
// // ─────────────────────────────────────────────
//
// /// Spawns a `◀ [value] ▶` spinner row as children of `parent`.
// ///
// /// * `kind`    – which logical counter this widget controls.
// /// * `display` – the initial human-readable value string (e.g. `"1"`).
// /// * `font`    – font handle shared across the panel.
// pub fn spawn_spinner(
//     parent: &mut ChildSpawnerCommands,
//     kind: SpinnerKind,
//     display: &str,
//     font: Handle<Font>,
// ) {
//     parent
//         .spawn(Node {
//             flex_direction: FlexDirection::Row,
//             align_items: AlignItems::Center,
//             margin: UiRect::vertical(Val::Px(4.0)),
//             ..default()
//         })
//         .with_children(|row| {
//             // ◀ decrement button
//             let dec = spawn_button(row, "◀", font.clone());
//             row.commands().entity(dec).insert(SpinnerDecrement(kind));
//
//             // current-value label
//             row.spawn((
//                 Text::new(display.to_owned()),
//                 TextFont { font: font.clone(), font_size: 14.0, ..default() },
//                 TextColor(COLOR_TEXT),
//                 Node {
//                     min_width: Val::Px(28.0),
//                     justify_content: JustifyContent::Center,
//                     ..default()
//                 },
//                 SpinnerLabel(kind),
//             ));
//
//             // ▶ increment button
//             let inc = spawn_button(row, "▶", font.clone());
//             row.commands().entity(inc).insert(SpinnerIncrement(kind));
//         });
// }
//
// // ─────────────────────────────────────────────
// //  Info label builder
// // ─────────────────────────────────────────────
//
// /// Spawns a `"<prefix>  <value>"` row.
// ///
// /// Returns the `Entity` of the *value* text node so the caller can tag it with
// /// a marker component for later updates.
// ///
// /// # Example
// /// ```rust
// /// let e = spawn_info_label(col, "Planet: ", "–", font.clone());
// /// commands.entity(e).insert(ExplorerPlanetLabel);
// /// ```
// pub fn spawn_info_label(
//     parent: &mut ChildSpawnerCommands,
//     prefix: &str,
//     initial_value: &str,
//     font: Handle<Font>,
// ) -> Entity {
//     let mut value_entity = Entity::PLACEHOLDER;
//
//     parent
//         .spawn(Node {
//             flex_direction: FlexDirection::Row,
//             align_items: AlignItems::Center,
//             margin: UiRect::vertical(Val::Px(3.0)),
//             ..default()
//         })
//         .with_children(|row| {
//             // Static prefix
//             row.spawn((
//                 Text::new(prefix.to_owned()),
//                 TextFont { font: font.clone(), font_size: 13.0, ..default() },
//                 TextColor(COLOR_LABEL),
//             ));
//
//             // Dynamic value – capture Entity before any further borrow
//             value_entity = row
//                 .spawn((
//                     Text::new(initial_value.to_owned()),
//                     TextFont { font: font.clone(), font_size: 13.0, ..default() },
//                     TextColor(COLOR_TEXT),
//                 ))
//                 .id();
//         });
//
//     value_entity
// }