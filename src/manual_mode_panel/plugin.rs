// use bevy::prelude::*;
// use crate::manual_mode_panel::resources::*;
// use crate::manual_mode_panel::systems::*;
// 
// pub struct ManualModePanelPlugin;
// 
// impl Plugin for ManualModePanelPlugin {
//     fn build(&self, app: &mut App) {
//         app
//             .init_resource::<ManualModeState>()
// 
//             // ── Spawn / despawn ────────────────────────────────────────
//             // Swap `Startup` for `OnEnter(AppState::InGame)` if you use states.
//             // .add_systems(Startup, spawn_admin_panel)
//             // Swap `Last` for `OnExit(AppState::InGame)` if you use states.
//             // .add_systems(OnExit(AppState::InGame), despawn_admin_panel)
// 
//             // ── Per-frame systems ───────────────────────────────────────
//             .add_systems(
//                 Update,
//                 (
//                     // Input handling (must run before sync systems)
//                     handle_tab_clicks,
//                     handle_spinner_clicks,
// 
//                     // Reactive UI updates (run after input)
//                     update_panel_visibility,
//                     update_tab_visibility,
//                     update_tab_button_style,
//                     sync_spinner_labels,
//                     sync_explorer_labels,
//                 )
//                     .chain(), // keep ordering deterministic
//             )
//         ;
//     }
// }