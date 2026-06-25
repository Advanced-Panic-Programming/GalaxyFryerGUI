// use bevy::prelude::*;
// use crate::manual_mode_panel::resources::Tab;
// // ─────────────────────────────────────────────
// //  Root marker
// // ─────────────────────────────────────────────
// 
// /// Marker for the top-level panel entity.
// /// Despawning this entity (recursive) tears down the whole UI subtree.
// #[derive(Component)]
// pub struct AdminPanel;
// 
// // ─────────────────────────────────────────────
// //  Tab system
// // ─────────────────────────────────────────────
// 
// /// Marker attached to the clickable tab header buttons.
// #[derive(Component)]
// pub struct TabButton(pub Tab);
// 
// /// Marker attached to the content container that belongs to a given tab.
// /// Only the container whose `Tab` matches `AdminPanelState::active_tab` is visible.
// #[derive(Component)]
// pub struct TabContent(pub Tab);
// 
// // ─────────────────────────────────────────────
// //  Shared spinner (arrow buttons + display label)
// // ─────────────────────────────────────────────
// 
// /// Identifies the "decrement" arrow of a spinner.
// #[derive(Component)]
// pub struct SpinnerDecrement(pub SpinnerKind);
// 
// /// Identifies the "increment" arrow of a spinner.
// #[derive(Component)]
// pub struct SpinnerIncrement(pub SpinnerKind);
// 
// /// Label that displays the current human-readable value of a spinner (1-based).
// #[derive(Component)]
// pub struct SpinnerLabel(pub SpinnerKind);
// 
// /// Which logical counter this spinner widget controls.
// #[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
// pub enum SpinnerKind {
//     /// Planet selector used in the Galaxy tab and in the Explorer move sub-panel.
//     GalaxyPlanet,
//     /// Planet target selector inside the Explorer tab.
//     ExplorerTargetPlanet,
//     /// Which explorer is currently selected (index 0-1 → shown as 1-2).
//     ExplorerSelector,
//     /// Index into the first resource list shown in the Explorer tab.
//     ExplorerResource1,
//     /// Index into the second resource list shown in the Explorer tab.
//     ExplorerResource2,
// }
// 
// // ─────────────────────────────────────────────
// //  Galaxy-tab action buttons
// // ─────────────────────────────────────────────
// 
// /// Button: send a sun-ray event to the selected planet.
// #[derive(Component)]
// pub struct SendSunrayButton;
// 
// /// Button: send an asteroid event to the selected planet.
// #[derive(Component)]
// pub struct SendAsteroidButton;
// 
// /// Button: push a state-update request for the selected planet.
// #[derive(Component)]
// pub struct UpdateStateButton;
// 
// // ─────────────────────────────────────────────
// //  Explorer-tab action buttons
// // ─────────────────────────────────────────────
// 
// /// Button: order the selected explorer to move to the target planet.
// #[derive(Component)]
// pub struct MoveExplorerButton;
// 
// /// Button: start the AI controller for the selected explorer.
// #[derive(Component)]
// pub struct StartAiButton;
// 
// /// Button: stop the AI controller for the selected explorer.
// #[derive(Component)]
// pub struct StopAiButton;
// 
// // ─────────────────────────────────────────────
// //  Explorer info labels
// // ─────────────────────────────────────────────
// 
// /// Label that shows "Planet: X" for the currently selected explorer.
// #[derive(Component)]
// pub struct ExplorerPlanetLabel;
// 
// /// Label that shows the bag contents for the currently selected explorer.
// #[derive(Component)]
// pub struct ExplorerBagLabel;