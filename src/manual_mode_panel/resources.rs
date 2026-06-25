// use bevy::prelude::*;
// 
// // ─────────────────────────────────────────────
// //  Tab enum
// // ─────────────────────────────────────────────
// 
// /// The two tabs available
// #[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
// pub enum Tab {
//     #[default]
//     Galaxy,
//     Explorer,
// }
// 
// // ─────────────────────────────────────────────
// //  Panel state resource
// // ─────────────────────────────────────────────
// 
// /// Central state for the admin panel.
// /// Insert this as a `Resource` before the panel is spawned.
// #[derive(Resource)]
// pub struct ManualModeState {
//     /// Whether the panel is currently visible.
//     pub visible: bool,
//     /// Which tab is rendered on top.
//     pub active_tab: Tab,
// 
//     // ── Galaxy tab ──────────────────────────
//     /// Selected planet index (0-6). Display as `selected_planet + 1`.
//     pub selected_planet: usize,
// 
//     // ── Explorer tab ────────────────────────
//     /// Selected explorer index (0-1). Display as `selected_explorer + 1`.
//     pub selected_explorer: usize,
//     /// Target planet index for the move command (0-6).
//     pub explorer_target_planet: usize,
//     /// Index into `RESOURCE_LIST_1` currently highlighted.
//     pub resource1_index: usize,
//     /// Index into `RESOURCE_LIST_2` currently highlighted.
//     pub resource2_index: usize,
// }
// 
// impl Default for ManualModeState {
//     fn default() -> Self {
//         Self {
//             visible: true,
//             active_tab: Tab::Galaxy,
//             selected_planet: 0,
//             selected_explorer: 0,
//             explorer_target_planet: 0,
//             resource1_index: 0,
//             resource2_index: 0,
//         }
//     }
// }