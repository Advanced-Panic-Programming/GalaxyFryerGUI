//! Pure spawn helpers.
//!
//! Every function in this module **only spawns entities** — no queries, no
//! resource mutations, no change detection. All update logic lives in `systems`.

use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use common_game::utils::ID;
use galaxy_fryer::explorer::bag::BagView;
use crate::galaxy_view::components::AnimationConfig;
use crate::planet_view::components::*;
use crate::planet_view::utils::*;
use crate::setup_simulation::resources::{
    EnergyCellsSpritesData, ExplorerSpriteData, ExplorerSpriteInfo, ExplorersData,
    PlanetInfo, PlanetSpriteInfo, PlanetTerrainSpriteInfo,
    RocketSpritesData,
};

// ── Corner planet ─────────────────────────────────────────────────────────────

/// Spawns the animated corner-planet sprite sheet.
/// The atlas is a single row of 144 frames, each 72×72 px.
pub fn spawn_corner_planet(
    commands: &mut Commands,
    layouts: &mut Assets<TextureAtlasLayout>,
    planet_sprite: &PlanetSpriteInfo,
    window: &Window,
) {
    let planet = if planet_sprite.alive {
        &planet_sprite.alive_sprite
    } else {
        &planet_sprite.destroyed_sprite
    };

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(72), 144, 1, None, None);
    let atlas_handle = layouts.add(layout);

    let width = window.width();
    let height = window.height();
    
    commands.spawn((
        Sprite {
            image: planet.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: atlas_handle,
                index: 0,
            }),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            adapt_to_width(width, CORNER_PLANET_X), 
            adapt_to_height(height, CORNER_PLANET_Y), 
            adapt_scale_to_current_screen(width*height, CORNER_PLANET_SCALE)))
            .with_scale(Vec3::splat(CORNER_PLANET_SCALE)),
        AnimationConfig::new(0, 143, ANIMATION_FPS),
        SpawnedByPlanetView,
        CornerPlanet,
    ));
}

// ── Full planet view ──────────────────────────────────────────────────────────

/// Spawns the complete planet view: terrain, rocket, energy cells and explorers.
///
/// All entities are children of an invisible root (Transform + Visibility) so
/// they can be bulk-despawned via `despawn_related::<ChildOf>()` on the root.
pub fn spawn_planet_view(
    commands: &mut Commands,
    planet_info: &PlanetInfo,
    terrain_sprites: &PlanetTerrainSpriteInfo,
    rocket_sprites: &RocketSpritesData,
    cell_sprites: &EnergyCellsSpritesData,
    explorers_data: &ExplorersData,
    explorer_sprites: &ExplorerSpriteData,
    font: Handle<Font>,
    planet_index: usize,
) {

    if planet_info.get_alive() {
    commands
        .spawn((
            Transform::default(),
            Visibility::default(),
            SpawnedByPlanetView,
        ))
        .with_children(|parent| {
            spawn_terrain_background(parent, planet_info, terrain_sprites);
            spawn_rocket(parent, planet_info, rocket_sprites);
            spawn_energy_cells(parent, planet_info, cell_sprites);
            spawn_explorers(parent, explorers_data, explorer_sprites, font, planet_index);
        });
    } else {
        commands
            .spawn((
                Transform::default(),
                Visibility::default(),
                SpawnedByPlanetView,
            ))
            .with_children(|parent| {
                spawn_terrain_background(parent, planet_info, terrain_sprites);
                spawn_explorers(parent, explorers_data, explorer_sprites, font, planet_index);
            });
    }

}

// ── Terrain ───────────────────────────────────────────────────────────────────

/// Spawns the 1920×1080 terrain background.
pub fn spawn_terrain_background(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    planet_info: &PlanetInfo,
    terrain_sprites: &PlanetTerrainSpriteInfo,
) {
    let image = terrain_image(planet_info, terrain_sprites);
    parent.spawn((
        Sprite {
            image,
            custom_size: Some(TERRAIN_SIZE),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, Z_TERRAIN),
        TerrainBackground,
    ));
}

// ── Rocket ────────────────────────────────────────────────────────────────────

/// Spawns the rocket sprite (only if the planet supports one).
pub fn spawn_rocket(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    planet_info: &PlanetInfo,
    rocket_sprites: &RocketSpritesData,
) {
    if !planet_info.can_have_rocket() {
        return;
    }

    let image = rocket_image(planet_info, rocket_sprites);
    parent.spawn((
        Sprite {
            image,
            custom_size: Some(ROCKET_SIZE),
            ..default()
        },
        Transform::from_translation(ROCKET_POS),
        Rocket,
    ));
}

// ── Energy cells ──────────────────────────────────────────────────────────────

/// Spawns the full row of energy-cell sprites.
pub fn spawn_energy_cells(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    planet_info: &PlanetInfo,
    cell_sprites: &EnergyCellsSpritesData,
) {
    for (i, &charged) in planet_info.get_energy_cells().iter().enumerate() {
        let image = cell_image(charged, cell_sprites);
        parent.spawn((
            Sprite {
                image,
                custom_size: Some(CELL_SIZE),
                ..default()
            },
            Transform::from_xyz(cell_x(i), CELLS_Y, Z_CELLS),
            EnergyCell {
                cell_index: i,
            },
        ));
    }
}

// ── Explorers ─────────────────────────────────────────────────────────────────

/// Spawns whichever explorers are currently on `planet_index`.
pub fn spawn_explorers(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    explorers_data: &ExplorersData,
    explorer_sprites: &ExplorerSpriteData,
    font: Handle<Font>,
    planet_index: usize,
) {
    let explorer_configs = [
        (0 as ID, &explorers_data.explorer1, &explorer_sprites.explorer1, EXPLORER1_POS),
        (1 as ID, &explorers_data.explorer2, &explorer_sprites.explorer2, EXPLORER2_POS),
    ];

    for (id, explorer, sprite_info, pos) in explorer_configs {
        if explorer.get_current_planet_index() == planet_index {
            spawn_single_explorer(
                parent,
                id,
                explorer.is_alive(),
                explorer.get_bag(),
                sprite_info,
                pos,
                font.clone(),
            );
        }
    }
}

/// Spawns sprite + bag label for one explorer.
fn spawn_single_explorer(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    explorer_id: ID,
    alive: bool,
    bag: &BagView,
    sprite_info: &ExplorerSpriteInfo,
    position: Vec3,
    font: Handle<Font>,
) {
    let image = explorer_image(alive, sprite_info);

    parent.spawn((
        Sprite {
            image,
            custom_size: Some(EXPLORER_SIZE),
            ..default()
        },
        Transform::from_translation(position),
        ExplorerSprite { explorer_id },
    ));

    // Text 2d auto-inserts Transform as a required component; we override it via
    // .insert() to avoid a duplicate-component conflict in the spawn tuple.
    parent
        .spawn((
            Text2d::new(format!("{bag}")),
            TextFont::from(font).with_font_size(BAG_FONT_SIZE),
            TextColor(Color::WHITE),
            ExplorerBagLabel { explorer_id },
        ))
        .insert(Transform::from_translation(bag_label_pos(position)));
}

// ── Image-selection helpers ───────────────────────────────────────────────────
// These small pure functions centralise every "pick the right handle" decision
// so callers (spawn + update) share a single source of truth.

pub fn terrain_image(
    planet_info: &PlanetInfo,
    terrain_sprites: &PlanetTerrainSpriteInfo,
) -> Handle<Image> {
    if planet_info.get_alive() {
        terrain_sprites.alive.clone()
    } else {
        terrain_sprites.destroyed.clone()
    }
}

pub fn rocket_image(
    planet_info: &PlanetInfo,
    rocket_sprites: &RocketSpritesData,
) -> Handle<Image> {
    if planet_info.get_rocket() {
        rocket_sprites.rocket.full_built_rocket.clone()
    } else {
        rocket_sprites.rocket.empty_rocket_base.clone()
    }
}

pub fn cell_image(charged: bool, cell_sprites: &EnergyCellsSpritesData) -> Handle<Image> {
    if charged {
        cell_sprites.energy_cell.charged.clone()
    } else {
        cell_sprites.energy_cell.empty.clone()
    }
}

pub fn explorer_image(alive: bool, sprite_info: &ExplorerSpriteInfo) -> Handle<Image> {
    if alive {
        sprite_info.alive_sprite.clone()
    } else {
        sprite_info.dead_sprite.clone()
    }
}

// ── Public re-export of single-explorer spawn ─────────────────────────────────
// Exposed so `systems::update_explorers` can spawn a late-arriving explorer
// into an existing view root without duplicating the spawn logic.

pub fn spawn_single_explorer_pub(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    explorer_id: ID,
    alive: bool,
    bag: &BagView,
    sprite_info: &ExplorerSpriteInfo,
    position: Vec3,
    font: Handle<Font>,
) {
    spawn_single_explorer(parent, explorer_id, alive, bag, sprite_info, position, font);
}