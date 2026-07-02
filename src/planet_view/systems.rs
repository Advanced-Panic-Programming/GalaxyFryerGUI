use bevy::prelude::*;
use bevy::window::WindowResized;
use common_game::utils::ID;
use galaxy_fryer::app::gui_protocol::GUIToOrchestrator::AskPlanetState;
use crate::galaxy_view::components::AnimationConfig;
use crate::planet_view::builders::*;
use crate::planet_view::components::*;
use crate::planet_view::utils::*;
use crate::setup_orchestrator::resources::ToOrchestrator;
use crate::setup_simulation::resources::{EnergyCellsSpritesData, ExplorerSpriteData, ExplorersData, PlanetTerrainSpriteData, PlanetsData, PlanetsSpritesData, RocketSpritesData, SelectedPlanet};
use crate::setup_simulation::utils::{visible_world_size, DESIGN_HEIGHT, DESIGN_WIDTH};

// ── Helpers ───────────────────────────────────────────────────────────────────
/// The terrain size to use when a real window isn't available.
fn fallback_terrain_size() -> Vec2 {
    Vec2::new(DESIGN_WIDTH, DESIGN_HEIGHT)
}

/// Despawns every entity tagged `SpawnedByPlanetView`, including their children.
fn despawn_view(commands: &mut Commands, query: &Query<Entity, (With<SpawnedByPlanetView>, Without<ChildOf>)>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Returns the planet index currently stored in `SelectedPlanet`, logging a
/// warning and returning `None` if no planet is selected.
fn require_selected(selected: &SelectedPlanet, caller: &str) -> Option<usize> {
    let index = selected.get();
    if index.is_none() {
        warn!("{caller}: SelectedPlanet has no value — skipping");
    }
    index
}

// ── OnEnter ───────────────────────────────────────────────────────────────────

/// Spawns the animated corner-planet sprite.
/// Must run after `cleanup` so no stale entity lingers from a previous visit.
pub fn spawn_corner_planet_system(
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    planets_sprites_data: Res<PlanetsSpritesData>,
    selected_planet: Res<SelectedPlanet>,
) {
    let Some(index) = require_selected(&selected_planet, "spawn_corner_planet") else {
        return;
    };

    spawn_corner_planet(
        &mut commands,
        &mut layouts,
        &planets_sprites_data.planets[index],
    );
}

/// Spawns the full planet view: terrain, rocket, cells, explorers.
/// Must run after `cleanup`.
pub fn spawn_planet_view_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selected: Res<SelectedPlanet>,
    planets_data: Res<PlanetsData>,
    planet_terrain_sprite_data: Res<PlanetTerrainSpriteData>,
    explorers_data: Res<ExplorersData>,
    explorer_sprite_data: Res<ExplorerSpriteData>,
    rocket_sprites_data: Res<RocketSpritesData>,
    energy_cells_sprites_data: Res<EnergyCellsSpritesData>,
    windows: Query<&Window>,
) {
    let Some(planet_index) = require_selected(&selected, "spawn_planet_view") else {
        return;
    };

    let terrain_size = windows.single()
        .map(|w| visible_world_size(w.width(), w.height()))
        .unwrap_or_else(|_| fallback_terrain_size());

    spawn_planet_view(
        &mut commands,
        &planets_data.planets[planet_index],
        &planet_terrain_sprite_data.terrain,
        &rocket_sprites_data,
        &energy_cells_sprites_data,
        &explorers_data,
        &explorer_sprite_data,
        asset_server.load(EXPLORERS_BAG_FONT_PATH),
        planet_index,
        terrain_size,
    );
}

/// When the user selects a different planet, tears down the whole view and
/// rebuilds it from scratch. This is the only case that warrants a full respawn:
/// every element on screen belongs to a different planet.
pub fn on_planet_changed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    selected: Res<SelectedPlanet>,
    planets_data: Res<PlanetsData>,
    planets_sprites_data: Res<PlanetsSpritesData>,
    planet_terrain_sprite_data: Res<PlanetTerrainSpriteData>,
    explorers_data: Res<ExplorersData>,
    explorer_sprite_data: Res<ExplorerSpriteData>,
    rocket_sprites_data: Res<RocketSpritesData>,
    energy_cells_sprites_data: Res<EnergyCellsSpritesData>,
    existing: Query<Entity, (With<SpawnedByPlanetView>, Without<ChildOf>)>,
    windows: Query<&Window>,
) {
    if !selected.is_changed() {
        return;
    }

    let Some(planet_index) = require_selected(&selected, "on_planet_changed") else {
        return;
    };

    despawn_view(&mut commands, &existing);

    spawn_corner_planet(
        &mut commands,
        &mut layouts,
        &planets_sprites_data.planets[planet_index],
    );

    let terrain_size = windows.single()
        .map(|w| visible_world_size(w.width(), w.height()))
        .unwrap_or_else(|_| fallback_terrain_size());

    spawn_planet_view(
        &mut commands,
        &planets_data.planets[planet_index],
        &planet_terrain_sprite_data.terrain,
        &rocket_sprites_data,
        &energy_cells_sprites_data,
        &explorers_data,
        &explorer_sprite_data,
        asset_server.load(EXPLORERS_BAG_FONT_PATH),
        planet_index,
        terrain_size,
    );
}

/// Swaps the terrain background and rocket images when `PlanetsData` changes
/// (planet destroyed, rocket built/used).
/// Touches only the two entities that may need a new image — no respawn.
pub fn update_terrain_and_rocket(
    selected: Res<SelectedPlanet>,
    planets_data: Res<PlanetsData>,
    planet_terrain_sprite_data: Res<PlanetTerrainSpriteData>,
    rocket_sprites_data: Res<RocketSpritesData>,
    mut terrain_query: Query<&mut Sprite, With<TerrainBackground>>,
    mut rocket_query: Query<&mut Sprite, (With<Rocket>, Without<TerrainBackground>)>,
) {
    if !planets_data.is_changed() {
        return;
    }

    let Some(planet_index) = require_selected(&selected, "update_terrain_and_rocket") else {
        return;
    };

    let planet_info = &planets_data.planets[planet_index];

    // Terrain background
    for mut sprite in terrain_query.iter_mut() {
        sprite.image = terrain_image(planet_info, &planet_terrain_sprite_data.terrain);
    }

    // Rocket (may not exist on all planets)
    if planet_info.can_have_rocket() {
        for mut sprite in rocket_query.iter_mut() {
            sprite.image = rocket_image(planet_info, &rocket_sprites_data);
        }
    }
}

/// Updates individual energy-cell sprites when `PlanetsData` changes.
/// Each cell is patched independently via its `cell_index` component — no respawn.
pub fn update_energy_cells(
    selected: Res<SelectedPlanet>,
    planets_data: Res<PlanetsData>,
    cell_sprites: Res<EnergyCellsSpritesData>,
    mut cell_query: Query<(&EnergyCell, &mut Sprite)>,
) {
    if !planets_data.is_changed() {
        return;
    }

    let Some(planet_index) = require_selected(&selected, "update_energy_cells") else {
        return;
    };

    let energy_cells = planets_data.planets[planet_index].get_energy_cells();

    for (cell, mut sprite) in cell_query.iter_mut() {
        if let Some(&charged) = energy_cells.get(cell.cell_index) {
            sprite.image = cell_image(charged, &cell_sprites);
        }
    }
}

// ── Update: explorer data changed ─────────────────────────────────────────────

/// Reacts to changes in `ExplorersData`:
/// - Explorer moved away from this planet → despawn its sprite + bag label.
/// - Explorer moved to this planet → spawn its sprite + bag label.
/// - Explorer died (same planet) → swap the sprite image.
/// - Bag contents changed (same planet) → update the label text.
pub fn update_explorers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selected: Res<SelectedPlanet>,
    explorers_data: Res<ExplorersData>,
    explorer_sprite_data: Res<ExplorerSpriteData>,
    // Separate queries to avoid mutable aliasing
    mut sprite_query: Query<(Entity, &ExplorerSprite, &mut Sprite)>,
    mut label_query: Query<(Entity, &ExplorerBagLabel, &mut Text2d)>,
    root_query: Query<Entity, (With<SpawnedByPlanetView>, Without<CornerPlanet>)>,
) {
    if !explorers_data.is_changed() {
        return;
    }

    let Some(planet_index) = require_selected(&selected, "update_explorers") else {
        return;
    };

    let explorer1_pos = Vec3::new(EXPLORER1_X, EXPLORER1_Y, EXPLORER_Z);
    let explorer2_pos = Vec3::new(EXPLORER2_X, EXPLORER2_Y, EXPLORER_Z);

    let explorer_configs = [
        (0 as ID, &explorers_data.explorer1, &explorer_sprite_data.explorer1, explorer1_pos),
        (1 as ID, &explorers_data.explorer2, &explorer_sprite_data.explorer2, explorer2_pos),
    ];

    for (explorer_id, explorer, sprite_info, position) in explorer_configs {
        let on_planet = explorer.get_current_planet_index() == planet_index;

        // Find existing sprite and label entities for this explorer (if any).
        let existing_sprite = sprite_query
            .iter()
            .find(|(_, s, _)| s.explorer_id == explorer_id)
            .map(|(e, _, _)| e);

        let existing_label = label_query
            .iter()
            .find(|(_, l, _)| l.explorer_id == explorer_id)
            .map(|(e, _, _)| e);

        match (on_planet, existing_sprite, existing_label) {
            // Explorer is here and was already here → patch in-place
            (true, Some(sprite_entity), Some(label_entity)) => {
                // Update sprite image in case the explorer just died
                if let Ok((_, _, mut sprite)) = sprite_query.get_mut(sprite_entity) {
                    sprite.image = explorer_image(explorer.is_alive(), sprite_info);
                }
                // Update bag label text
                if let Ok((_, _, mut text)) = label_query.get_mut(label_entity) {
                    text.0 = format!("{}", explorer.get_bag());
                }
            }

            // Explorer arrived on this planet → spawn
            (true, None, _) => {
                // Attach to the view root so the entity is despawned with the view
                if let Ok(root) = root_query.single() {
                    commands.entity(root).with_children(|parent| {
                        spawn_single_explorer_pub(
                            parent,
                            explorer_id,
                            explorer.is_alive(),
                            explorer.get_bag(),
                            sprite_info,
                            position,
                            asset_server.load(EXPLORERS_BAG_FONT_PATH),
                        );
                    });
                }
            }

            // Explorer left this planet → despawn
            (false, Some(sprite_entity), Some(label_entity)) => {
                commands.entity(sprite_entity).despawn();
                commands.entity(label_entity).despawn();
            }

            // Remaining combinations (e.g. label exists but sprite doesn't) are
            // inconsistent state — ignore and let the next full respawn fix them.
            _ => {}
        }
    }
}

/// Keeps the terrain background sized to fully cover the camera's visible area during PlanetView
pub fn update_terrain_size_on_resize(
    mut resize_events: MessageReader<WindowResized>,
    mut terrain_query: Query<&mut Sprite, With<TerrainBackground>>,
) {
    let Some(event) = resize_events.read().last() else {
        return;
    };
    let size = visible_world_size(event.width, event.height);
    for mut sprite in terrain_query.iter_mut() {
        sprite.custom_size = Some(size);
    }
}

/// Advances the corner-planet sprite-sheet animation by one frame when the per-frame timer fires.
pub fn animate_corner_planet(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<CornerPlanet>>,
) {
    let Ok((mut config, mut sprite)) = query.single_mut() else {
        return;
    };

    config.frame_timer.tick(time.delta());

    if config.frame_timer.just_finished()
        && let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = if atlas.index >= config.last_sprite_index {
                config.first_sprite_index
            } else {
                atlas.index + 1
            };
    }
}

// Update Planet State
pub fn immediately_ask_planet_state(
    sender: Res<ToOrchestrator>,
    selected_planet: Res<SelectedPlanet>,
) {
    if let Some(planet_id ) = selected_planet.get() {
        let _ = sender.0.send(AskPlanetState { planet_id: planet_id as ID });
    }
}

pub fn periodically_ask_planet_state(
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
    sender: Res<ToOrchestrator>,
    selected_planet: Res<SelectedPlanet>,
) {
    let timer = timer.get_or_insert_with(|| {
        Timer::from_seconds(0.5, TimerMode::Repeating)
    });

    if timer.tick(time.delta()).just_finished()
        && let Some(planet_id ) = selected_planet.get() {
            let _ = sender.0.send(AskPlanetState { planet_id: planet_id as ID });
    }
}

// ── OnExit ────────────────────────────────────────────────────────────────────

/// Removes every entity owned by the planet view, including their full child
/// hierarchy. Runs on state exit and before every full respawn.
pub fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<SpawnedByPlanetView>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}