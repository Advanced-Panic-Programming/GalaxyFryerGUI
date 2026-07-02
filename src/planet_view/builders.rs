use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use common_game::utils::ID;
use galaxy_fryer::explorer::bag::BagView;
use crate::galaxy_view::components::AnimationConfig;
use crate::planet_view::components::*;
use crate::planet_view::utils::*;
use crate::setup_simulation::resources::{EnergyCellsSpritesData, ExplorerSpriteData, ExplorerSpriteInfo, ExplorersData, PlanetInfo, PlanetSpriteInfo, PlanetTerrainSpriteInfo, RocketSpritesData};

// =========================
//      Corner planet
// =========================

/// Spawns the animated corner-planet sprite sheet.
/// The atlas is a single row of 144 frames, each 72×72 px.
pub fn spawn_corner_planet(
    commands: &mut Commands,
    layouts: &mut Assets<TextureAtlasLayout>,
    planet_sprite: &PlanetSpriteInfo,
) {
    let planet = if planet_sprite.alive {
        &planet_sprite.alive_sprite
    } else {
        &planet_sprite.destroyed_sprite
    };

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(72), 144, 1, None, None);
    let atlas_handle = layouts.add(layout);
    
    commands.spawn((
        Sprite {
            image: planet.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: atlas_handle,
                index: 0,
            }),
            ..default()
        },
        Transform::from_translation(Vec3::new(CORNER_PLANET_X, CORNER_PLANET_Y, CORNER_PLANET_Z))
            .with_scale(Vec3::splat(CORNER_PLANET_SCALE)),
        AnimationConfig::new(0, 143, ANIMATION_FPS),
        SpawnedByPlanetView,
        CornerPlanet,
    ));
}

// =========================
//     Full planet view
// =========================
/// Spawns the complete planet view: terrain, rocket, energy cells and explorers.
/// All entities are children of an invisible root (Transform + Visibility) so they can be despawned via the root.
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
    terrain_size: Vec2,
) {

    if planet_info.get_alive() {
    commands
        .spawn((
            Transform::default(),
            Visibility::default(),
            SpawnedByPlanetView,
        ))
        .with_children(|parent| {
            spawn_terrain_background(parent, planet_info, terrain_sprites, terrain_size);
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
                spawn_terrain_background(parent, planet_info, terrain_sprites, terrain_size);
                spawn_explorers(parent, explorers_data, explorer_sprites, font, planet_index);
            });
    }

}

/// Spawns the terrain background.
pub fn spawn_terrain_background(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    planet_info: &PlanetInfo,
    terrain_sprites: &PlanetTerrainSpriteInfo,
    terrain_size: Vec2,
) {
    let image = terrain_image(planet_info, terrain_sprites);
    parent.spawn((
        Sprite {
            image,
            custom_size: Some(terrain_size),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, TERRAIN_Z)
            .with_scale(Vec3::splat(TERRAIN_SCALE)),
        TerrainBackground,
    ));
}

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
            custom_size: Some(ROCKET_SCALE),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            ROCKET_X,
            ROCKET_Y,
            ROCKET_Z,
        )),
        Rocket,
    ));
}

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
            Transform::from_xyz(cell_x(i), CELLS_Y, CELLS_Z),
            EnergyCell {
                cell_index: i,
            },
        ));
    }
}

/// Spawns whichever explorers are currently on `planet_index`.
pub fn spawn_explorers(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    explorers_data: &ExplorersData,
    explorer_sprites: &ExplorerSpriteData,
    font: Handle<Font>,
    planet_index: usize,
) {

    let explorer1_pos = Vec3::new(
        EXPLORER1_X,
        EXPLORER1_Y,
        EXPLORER_Z,
    );

    let explorer2_pos = Vec3::new(
        EXPLORER2_X,
        EXPLORER2_Y,
        EXPLORER_Z,
    );

    let explorer_configs = [
        (0 as ID, &explorers_data.explorer1, &explorer_sprites.explorer1, explorer1_pos),
        (1 as ID, &explorers_data.explorer2, &explorer_sprites.explorer2, explorer2_pos),
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
    
    parent
        .spawn((
            Text2d::new(format!("{bag}")),
            TextFont::from(font).with_font_size(BAG_FONT_SIZE),
            TextColor(Color::WHITE),
            ExplorerBagLabel { explorer_id },
        ))
        .insert(Transform::from_translation(bag_label_pos(position)));
}

// Image-selection helpers: return the correct sprite to spawn based on the current state
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

// `systems::update_explorers` can spawn a late-arriving explorer into an existing view root without duplicating the spawn logic.
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