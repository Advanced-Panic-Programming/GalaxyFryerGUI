use std::ops::DerefMut;
use bevy::prelude::*;
use common_game::utils::ID;
use crate::setup_simulation::resources::*;
use crate::galaxy_view::components::*;
use crate::galaxy_view::messages::{ReceivedExplorerBag, ReceivedExplorerMove, ReceivedExplorerPosition, ReceivedKilledExplorer, ReceivedPlanetCombine, ReceivedPlanetDestroyed, ReceivedPlanetGenerate, ReceivedPlanetState};
use crate::galaxy_view::utils::*;

// =====================
// === Setup Systems ===
// =====================

pub fn spawn_planets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    planet_data: Res<PlanetsSpritesData>,
    mut galaxy: ResMut<Galaxy>,
) {
    // Every planet has 144 frames of 72x72 pixels (v: parameter)
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(72), 144, 1, None, None);
    let texture_atlas_layout = layouts.add(layout);

    let animation = AnimationConfig::new(0, 143, ANIMATION_FPS);

    for planet in planet_data.planets.iter() {

        let sprite_path = if planet.alive {
            &planet.sprite_path
        } else {
            &planet.destroyed_sprite_path
        };

        let texture: Handle<Image> = asset_server.load(sprite_path);

        let entity  = commands.spawn((
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: animation.first_sprite_index,
                }),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, -50.0)).with_scale(Vec3::splat(PLANET_INITIAL_SPLAT)),
            AnimationConfig::new(0, 143, ANIMATION_FPS),
            Planet{
                index : planet.index,
            },
            SpawnedByGalaxyView,
        )).id();
        galaxy.planets.push(entity);
    }
}

// ======================
// === Update Systems ===
// ======================

pub fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite), With<SpawnedByGalaxyView>>) {
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index >= config.last_sprite_index {
                    atlas.index = config.first_sprite_index;
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}

pub fn update_planets_sprites(
    time: Res<Time>,
    orbit: Res<GalaxyOrbit>,
    asset_server: Res<AssetServer>,
    mut planets: ResMut<PlanetsSpritesData>,
    mut query: Query<(&mut Transform, &mut Sprite, &mut Planet)>,
) {
    for (mut transform, mut sprite, planet) in query.iter_mut() {

        let curr_planet = &mut planets.planets[planet.index];

        // Update the timer
        curr_planet.timer.tick(time.delta());

        // Move clockwise: decrease the angle
        curr_planet.angle -= curr_planet.speed * time.delta_secs() * 0.5;

        // Calculate new position along the ellipse
        let x = orbit.a * curr_planet.angle.cos();
        let y = -orbit.b * curr_planet.angle.sin();

        // Apply new position
        transform.translation = Vec3::new(
            orbit.center.x + x,
            orbit.center.y + y,
            -50.0,
        );

        // Fake depth: smaller scale for higher Y (closer to top)
        let normalized_y = (y + orbit.b) / (2.0 * orbit.b);
        let scale_value = PLANET_INITIAL_SPLAT * (1.0 - normalized_y * 0.5); // tweak depth factor
        transform.scale = Vec3::splat(scale_value);

        // Check if the planet exploded
        let should_be_alive = curr_planet.alive;
        let current_handle = &sprite.image;

        let expected = if should_be_alive {
            &curr_planet.sprite_path
        } else {
            &curr_planet.destroyed_sprite_path
        };

        // if handle doesn't match expected image: update
        if asset_server
            .get_path(current_handle)
            .map_or(true, |p| p.path().to_str() != Some(expected.as_str()))
        {
            sprite.image = asset_server.load(expected);
        }

    }
}

pub fn bound_explorer_arrows(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    explorers: Res<ExplorersData>,
    planets: Query<(Entity, &Planet), Without<ExplorerArrow>>,
    arrows: Query<(Entity, &ExplorerArrow)>,
) {
    // Collect planet index for each explorer
    let e1_planet = explorers.explorer1.get_current_planet_index();
    let e2_planet = explorers.explorer2.get_current_planet_index();

    // Are they on the same planet?
    let same_planet = explorers.explorer1.is_alive()
        && explorers.explorer2.is_alive()
        && e1_planet == e2_planet;

    // Pre-computed offsets
    fn explorer_offset(explorer_id: usize, same_planet: bool) -> Vec3 {
        if same_planet {
            match explorer_id {
                1 => Vec3::new(-EXPLORER_ARROW_HORIZONTAL_OFFSET, EXPLORER_ARROW_VERTICAL_OFFSET, 0.1),
                2 => Vec3::new(EXPLORER_ARROW_HORIZONTAL_OFFSET, EXPLORER_ARROW_VERTICAL_OFFSET, 0.1),
                _ => Vec3::ZERO,
            }
        } else {
            // centered
            Vec3::new(0.0, EXPLORER_ARROW_VERTICAL_OFFSET, 0.1)
        }
    }

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(72), 14, 1, None, None);
    let texture_atlas_layout = layouts.add(layout);

    let animation = AnimationConfig::new(0, 13, ANIMATION_FPS);

    // helper table
    let explorer_states = [
        (1, &explorers.explorer1),
        (2, &explorers.explorer2),
    ];

    for (explorer_id, explorer) in explorer_states {
        let alive = explorer.is_alive();
        let planet_index = explorer.get_current_planet_index();

        // check existing arrow
        let existing_arrow = arrows
            .iter()
            .find(|(_, a)| a.explorer_id == explorer_id)
            .map(|(e, _)| e);

        match (alive, existing_arrow) {
            //--------------------------
            // SPAWN new arrow
            //--------------------------
            (true, None) => {
                if let Some((planet_entity, _)) =
                    planets.iter().find(|(_, p)| p.index == planet_index)
                {
                    let texture_path = match explorer_id {
                        1 => EXPLORER_1_SPRITE_PATH,
                        2 => EXPLORER_2_SPRITE_PATH,
                        _ => unreachable!(),
                    };

                    commands.entity(planet_entity).with_children(|parent| {
                        parent.spawn((
                            Sprite {
                                image: asset_server.load(texture_path),
                                custom_size: Some(Vec2::new(EXPLORER_ARROW_DIMENSION, EXPLORER_ARROW_DIMENSION)),
                                texture_atlas: Some(TextureAtlas {
                                    layout: texture_atlas_layout.clone(),
                                    index: animation.first_sprite_index,
                                }),
                                ..default()
                            },
                            Transform::from_translation(
                                explorer_offset(explorer_id, same_planet)
                            ).with_scale(Vec3::splat(EXPLORER_INITIAL_SPLAT)),
                            ExplorerArrow { planet_index, explorer_id },
                            AnimationConfig::new(0, 13, ANIMATION_FPS),
                            SpawnedByGalaxyView,
                        ));
                    });
                }
            }

            //--------------------------
            // UPDATE PARENT + OFFSET
            //--------------------------
            (true, Some(arrow_entity)) => {
                if let Some((planet_entity, _)) =
                    planets.iter().find(|(_, p)| p.index == planet_index)
                {
                    // ensure parent is correct
                    commands.entity(planet_entity).add_child(arrow_entity);

                    // also update its offset
                    commands.entity(arrow_entity).insert((
                        Transform::from_translation(
                            explorer_offset(explorer_id, same_planet)
                        ).with_scale(Vec3::splat(EXPLORER_INITIAL_SPLAT)),
                        SpawnedByGalaxyView,
                    ));
                }
            }

            //--------------------------
            // DESPAWN arrow
            //--------------------------
            (false, Some(arrow_entity)) => {
                commands.entity(arrow_entity).despawn();
            }

            // nothing
            (false, None) => {}
        }
    }
}

pub fn update_explorer_arrows_planet_binding(
    mut commands: Commands,
) {

}

pub fn update_planets_data(
    // Event readers
    mut planet_state_reader: MessageReader<ReceivedPlanetState>,
    mut planet_destroyed_reader: MessageReader<ReceivedPlanetDestroyed>,
    mut planet_generate_reader: MessageReader<ReceivedPlanetGenerate>,
    mut planet_combine_reader: MessageReader<ReceivedPlanetCombine>,
    // Resources
    mut planets_sprites: ResMut<PlanetsSpritesData>,
    mut planets_data: ResMut<PlanetsData>,
) {
    if !planet_state_reader.is_empty() {
        for msg in planet_state_reader.read() {
            if let Some(planet) = planets_data.planets.get_mut(msg.planet_id as usize - 1) {
                let charged_cells_count = msg.dummy_planet_state.charged_cells_count;
                planet.set_charged_cells_count(charged_cells_count);
                let has_rocket = msg.dummy_planet_state.has_rocket;
                planet.set_has_rocket(has_rocket);
                let energy_cells = msg.dummy_planet_state.energy_cells.clone();
                planet.set_energy_cells(energy_cells);
            }
        }
    }

    if !planet_destroyed_reader.is_empty() {
        for msg in planet_destroyed_reader.read() {
            if let Some(planet) = planets_data.planets.get_mut(msg.planet_id as usize - 1) {
                if planet.get_alive() {
                    planet.kill();
                }
            }
            planets_sprites.planets[msg.planet_id as usize - 1].alive = false; //TODO! Implement methods for the struct
        }
    }

    if !planet_generate_reader.is_empty() {
        for msg in planet_generate_reader.read() {
            if let Some(planet) = planets_data.planets.get_mut(msg.planet_id as usize - 1) {
                planet.set_generate(msg.generate.clone());
            }
        }
    }

    if !planet_combine_reader.is_empty() {
        for msg in planet_combine_reader.read() {
            if let Some(planet) = planets_data.planets.get_mut(msg.planet_id as usize - 1) {
                planet.set_combine(msg.combine.clone());
            }
        }
    }
}

// ====================
// ===== Explorer =====
// ====================

/*
pub fn bound_explorer_arrows(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    explorers: Res<ExplorersData>,
    planets: Query<(Entity, &Planet)>,
    arrows: Query<(Entity, &ExplorerArrow)>,
) {
    // Stati esploratori
    let explorer_states = [(1, &explorers.explorer1), (2, &explorers.explorer2)];

    // Conta esploratori per pianeta
    let mut explorers_on_planet = HashMap::<usize, usize>::new();
    for (_, e) in &explorer_states {
        if e.is_alive() {
            *explorers_on_planet.entry(e.get_current_planet_index()).or_insert(0) += 1;
        }
    }

    // Helper: calcola offset per evitare sovrapposizioni
    fn explorer_offset(explorer_id: usize, count: usize) -> Vec3 {
        match count {
            1 => Vec3::new(0.0, EXPLORER_ARROW_VERTICAL_OFFSET, 0.1),
            2 => {
                if explorer_id == 1 {
                    Vec3::new(-EXPLORER_ARROW_HORIZONTAL_OFFSET, EXPLORER_ARROW_VERTICAL_OFFSET, 0.1)
                } else {
                    Vec3::new(EXPLORER_ARROW_HORIZONTAL_OFFSET, EXPLORER_ARROW_VERTICAL_OFFSET, 0.1)
                }
            }
            _ => Vec3::new(0.0, EXPLORER_ARROW_VERTICAL_OFFSET, 0.1),
        }
    }

    for (explorer_id, explorer) in &explorer_states {
        let alive = explorer.is_alive();
        let planet_index = explorer.get_current_planet_index();

        let existing_arrow_entity = arrows
            .iter()
            .find(|(_, a)| a.explorer_id == *explorer_id)
            .map(|(e, _)| e);

        if !alive {
            if let Some(ent) = existing_arrow_entity {
                commands.entity(ent).despawn();
            }
            continue;
        }

        // Trova l’entity del pianeta
        let (planet_entity, _) = match planets.iter().find(|(_, p)| p.index == planet_index) {
            Some(p) => p,
            None => continue,
        };

        let same_planet_count = *explorers_on_planet.get(&planet_index).unwrap_or(&1);
        let offset = explorer_offset(*explorer_id, same_planet_count);

        let texture_path = match explorer_id {
            1 => "objects/explorer1_arrow.png",
            2 => "objects/explorer2_arrow.png",
            _ => unreachable!(),
        };
        let texture = asset_server.load(texture_path);

        match existing_arrow_entity {
            None => {
                // Spawn nuova arrow come child del pianeta
                commands.entity(planet_entity).with_children(|parent| {
                    parent.spawn((
                        Sprite {
                            image: texture,
                            custom_size: Some(Vec2::new(32.0, 32.0)),
                            ..default()
                        },
                        Transform::from_translation(offset),
                        ExplorerArrow { planet_index, explorer_id: *explorer_id },
                    ));
                });
            }
            Some(arrow_ent) => {
                // Aggiorna parent e offset se necessario
                commands.entity(arrow_ent)
                    .set_parent_in_place(planet_entity)
                    .insert(ExplorerArrow { planet_index, explorer_id: *explorer_id });
            }
        }
    }
}

pub fn update_explorer_arrows_planet_binding(
    mut commands: Commands,
    explorers: Res<ExplorersData>,
    arrows: Query<(Entity, &ExplorerArrow)>,
    planets: Query<(Entity, &Planet)>,
) {
    let find_planet_entity = |index: usize| -> Option<Entity> {
        for (entity, planet) in planets.iter() {
            if planet.index == index {
                return Some(entity);
            }
        }
        None
    };

    let explorer_states = [(1, &explorers.explorer1), (2, &explorers.explorer2)];

    for (explorer_id, explorer) in &explorer_states {
        if !explorer.is_alive() { continue; }
        let new_index = explorer.get_current_planet_index();

        for (arrow_entity, arrow) in arrows.iter() {
            if arrow.explorer_id == *explorer_id && arrow.planet_index != new_index {
                if let Some(new_planet_ent) = find_planet_entity(new_index) {
                    // Calcola offset
                    let offset = if *explorer_id == 1 {
                        Vec3::new(-EXPLORER_ARROW_HORIZONTAL_OFFSET, EXPLORER_ARROW_VERTICAL_OFFSET, 0.1)
                    } else {
                        Vec3::new(EXPLORER_ARROW_HORIZONTAL_OFFSET, EXPLORER_ARROW_VERTICAL_OFFSET, 0.1)
                    };

                    commands.entity(arrow_entity)
                        .set_parent_in_place(new_planet_ent)
                        .insert(ExplorerArrow { planet_index: new_index, explorer_id: *explorer_id });
                }
            }
        }
    }
}
*/

pub fn update_explorer_data (
    // Event readers
    mut explorer_position_reader: MessageReader<ReceivedExplorerPosition>,
    mut explorer_bag_reader: MessageReader<ReceivedExplorerBag>,
    mut explorer_move_reader: MessageReader<ReceivedExplorerMove>,
    mut killed_explorer_message: MessageReader<ReceivedKilledExplorer>,
    // Resource
    mut explorer_data: ResMut<ExplorersData>,
) {

    // Before updating I should check if the explorer is alive, BUT, if the explorer is dead, no updated will arrive!

    if !explorer_position_reader.is_empty() {
        for msg in explorer_position_reader.read() {
            match msg.explorer_id {
                1 => {explorer_data.explorer1.set_current_planet_index(msg.planet_id as usize - 1)}
                2 => {explorer_data.explorer2.set_current_planet_index(msg.planet_id as usize - 1)}
                _ => {}
            }
        }
        explorer_position_reader.clear(); //TODO! check when to put the clear()
    }

    if !explorer_bag_reader.is_empty() {
        for msg in explorer_bag_reader.read() {
            match msg.explorer_id {
                1 => {explorer_data.explorer1.set_bag(msg.explorer_bag.clone())}
                2 => {explorer_data.explorer2.set_bag(msg.explorer_bag.clone())}
                _ => {}
            }
        }
        explorer_bag_reader.clear();
    }

    if !explorer_move_reader.is_empty() {
        for msg in explorer_move_reader.read() {
            match msg.explorer_id {
                1 => {explorer_data.explorer1.set_current_planet_index(msg.planet_id as usize - 1)}
                2 => {explorer_data.explorer2.set_current_planet_index(msg.planet_id as usize - 1)}
                _ => {}
            }
        }
        explorer_move_reader.clear();
    }

    if !killed_explorer_message.is_empty() {
        for msg in killed_explorer_message.read() {
            match msg.explorer_id {
                1 => {explorer_data.explorer1.kill()}
                2 => {explorer_data.explorer2.kill()}
                _ => {}
            }
        }
        killed_explorer_message.clear();
    }
}


// ======================
// === OnExit Systems ===
// ======================

pub fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<SpawnedByGalaxyView>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}