use bevy::prelude::*;
use crate::cutscene::components::*;
use crate::cutscene::CutSceneType;
use crate::cutscene::resources::*;
use crate::cutscene::utils::*;
use crate::galaxy_view::messages::*;
use crate::setup_simulation::resources::RocketSpritesData;

/// Receives `PlanetDestroyedCutscene` / `AsteroidDestroyedCutscene` messages and
/// stores the cutscene type in `ActiveCutscene`. Ignores new messages while one is playing.
pub fn handle_cutscene(
    mut planet_destroyed: MessageReader<PlanetDestroyedCutscene>,
    mut asteroid_destroyed: MessageReader<AsteroidDestroyedCutscene>,
    mut active_cutscene: ResMut<ActiveCutscene>,
) {
    for msg in planet_destroyed.read() {
        active_cutscene.pending.push_back(CutSceneType::PlanetDestroyed(msg.planet_id));
    }
    for _msg in asteroid_destroyed.read() {
        active_cutscene.pending.push_back(CutSceneType::AsteroidDestroyed);
    }
    // Start immediately if nothing is currently playing.
    if active_cutscene.current.is_none() {
        active_cutscene.current = active_cutscene.pending.pop_front();
    }
}

/// Spawns all visual entities for the active cutscene.
/// Runs every frame but the early-return guard ensures it only fires once.
pub fn spawn_cutscene_visuals(
    mut commands: Commands,
    mut active: ResMut<ActiveCutscene>,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    rocket_sprites_data: Res<RocketSpritesData>,
) {
    // Only run once per cutscene.
    if active.current.is_none() || active.spawned {
        return;
    }
    active.spawned = true;

    let cutscene_type = active.current.unwrap();

    // Dark overlay
    commands.spawn((
        CutsceneEntity,
        CutsceneAnim::Overlay,
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(1920.0, 1080.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 100.0), // (z = 100 so that it's on top of everything)
    ));

    match cutscene_type {
        CutSceneType::PlanetDestroyed(id) => {
            // Orchestrator IDs are 1-based (0-6); asset files are named planet1.png...planet7.png.
            let file_idx = (id + 1) as usize;

            // Atlas for the live planet sprite (144 frames, used only for texture).
            let planet_atlas = layouts.add(
                TextureAtlasLayout::from_grid(UVec2::splat(72), 144, 1, None, None),
            );

            // PlanetStill (z = 103): shows the intact planet, scales in then fades out at impact.
            commands.spawn((
                CutsceneEntity,
                CutsceneAnim::PlanetStill,
                Sprite {
                    image: asset_server.load(format!("planets/planet{file_idx}.png")),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                    texture_atlas: Some(TextureAtlas {
                        layout: planet_atlas,
                        index: 0,
                    }),
                    ..default()
                },
                Transform::from_xyz(0.0, 30.0, 103.0).with_scale(Vec3::splat(3.0)),
            ));

            // Atlas for the destruction animation
            let destroy_atlas = layouts.add(
                TextureAtlasLayout::from_grid(UVec2::splat(72), EXPLOSION_ANIMATION_FRAMES as u32, 1, None, None),
            );

            // PlanetDestruction (z = 103): break-apart sequence starting at impact.
            commands.spawn((
                CutsceneEntity,
                CutsceneAnim::PlanetDestruction { num_frames: EXPLOSION_ANIMATION_FRAMES },
                Sprite {
                    image: asset_server.load(format!("destroyedPlanets/destroyedPlanet{file_idx}.png")),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                    texture_atlas: Some(TextureAtlas {
                        layout: destroy_atlas,
                        index: 0,
                    }),
                    ..default()
                },
                Transform::from_xyz(0.0, 30.0, 103.0).with_scale(Vec3::splat(PLANET_SPRITE_SCALE))
            ));

            // Asteroid sprite (z = 104 → in front of asteroid).
            // asteroid.png is 72×72 px
            let asteroid_atlas = layouts.add(
                TextureAtlasLayout::from_grid(UVec2::splat(72), 144, 1, None, None),
            );

            commands.spawn((
                CutsceneEntity,
                CutsceneAnim::IncomingAsteroidSprite { num_frames: ASTEROID_ANIMATION_FRAMES },
                Sprite {
                    image: asset_server.load(ASTEROID_SPRITE_PATH),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                    texture_atlas: Some(TextureAtlas {
                        layout: asteroid_atlas,
                        index: 0,
                    }),
                    ..default()
                },
                Transform::from_xyz(700.0, 0.0, 104.0).with_scale(Vec3::splat(ASTEROID_SPRITE_SCALE)),
            ));

            let explosion_layout = layouts.add(
                TextureAtlasLayout::from_grid(
                    UVec2::new(96, 96),
                    12,
                    1,
                    None,
                    None,
                ),
            );

            commands.spawn((
                CutsceneEntity,
                CutsceneAnim::Explosion {
                    num_frames: 12,
                },
                Sprite {
                    image: asset_server.load(EXPLOSION_SPRITE_PATH),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                    texture_atlas: Some(TextureAtlas {
                        layout: explosion_layout,
                        index: 0,
                    }),
                    ..default()
                },
                Transform::from_xyz(0.0, 30.0, 104.5)
                    .with_scale(Vec3::splat(EXPLOSION_SPRITE_SCALE)),
            ));

            // Caption text (z = 105).
            commands.spawn((
                CutsceneEntity,
                CutsceneAnim::Caption,
                Text2d::new(format!("PLANET {} DESTROYED!", id+1)),
                TextFont {
                    font: asset_server.load(FONT_PATH),
                    font_size: 52.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 0.3, 0.1, 1.0)),
                Transform::from_xyz(0.0, CAPTION_POSITION_Y, 105.0),
            ));
        }

        // ══════════════════════════════════════════════════════════════════════
        CutSceneType::AsteroidDestroyed => {

            // Atlas for the asteroid sprite (144 frames, used only for texture).
            let asteroid_atlas = layouts.add(
                TextureAtlasLayout::from_grid(UVec2::splat(72), 144, 1, None, None),
            );

            // Asteroid sprite (z = 103): shows the intact asteroid, scales in then fades out at impact.
            // Centered at origin so the rocket nose aligns with it at impact.
            commands.spawn((
                CutsceneEntity,
                CutsceneAnim::AsteroidStill,
                Sprite {
                    image: asset_server.load(ASTEROID_SPRITE_PATH),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                    texture_atlas: Some(TextureAtlas {
                        layout: asteroid_atlas,
                        index: 0,
                    }),
                    ..default()
                },
                Transform::from_xyz(0.0, 30.0, 103.0).with_scale(Vec3::splat(ASTEROID_SPRITE_SCALE)),
            ));

            // Atlas for the destruction animation
            let destroy_atlas = layouts.add(
                TextureAtlasLayout::from_grid(UVec2::splat(72), ASTEROID_ANIMATION_FRAMES as u32, 1, None, None),
            );

            // AsteroidDestruction (z = 104): break-apart sequence starting at impact.
            commands.spawn((
                CutsceneEntity,
                CutsceneAnim::AsteroidDestruction {num_frames: ASTEROID_ANIMATION_FRAMES},
                Sprite {
                    image: asset_server.load(DESTROYED_ASTEROID_SPRITE_PATH),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                    texture_atlas: Some(TextureAtlas {
                        layout: destroy_atlas,
                        index: 0,
                    }),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 103.0).with_scale(Vec3::splat(ASTEROID_SPRITE_SCALE)),
            ));

            // Rocket sprite (z = 104 → in front of asteroid).
            // rocket_pixel.png is 92×43 px (cropped from clean_rocket.png, facing left).
            // At scale 2.5 → 230×107 px.  Half-width = 115.
            // Final x = 80 → nose at 80 − 115 = −35, well inside the asteroid.
            commands.spawn((
                CutsceneEntity,
                CutsceneAnim::IncomingRocketSprite,
                Sprite {
                    image: rocket_sprites_data.rocket.flying_rocket.clone(),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                    ..default()
                },
                Transform::from_xyz(700.0, 0.0, 103.5).with_scale(Vec3::splat(ROCKET_SPRITE_SCALE)),
            ));

            let explosion_layout = layouts.add(
                TextureAtlasLayout::from_grid(
                    UVec2::new(96, 96),
                    12,
                    1,
                    None,
                    None,
                ),
            );

            commands.spawn((
                CutsceneEntity,
                CutsceneAnim::Explosion {
                    num_frames: 12,
                },
                Sprite {
                    image: asset_server.load(EXPLOSION_SPRITE_PATH),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                    texture_atlas: Some(TextureAtlas {
                        layout: explosion_layout,
                        index: 0,
                    }),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 104.5)
                    .with_scale(Vec3::splat(EXPLOSION_SPRITE_SCALE)),
            ));

            // Caption text (z = 105).
            commands.spawn((
                CutsceneEntity,
                CutsceneAnim::Caption,
                Text2d::new("ASTEROID DESTROYED!"),
                TextFont {
                    font: asset_server.load(FONT_PATH),
                    font_size: 52.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 0.92, 0.3, 1.0)),
                Transform::from_xyz(0.0, CAPTION_POSITION_Y, 105.0),
            ));
        }
    }
}

pub fn animate_cutscene(
    timer: Res<CutsceneTimer>,
    active: Res<ActiveCutscene>,
    mut sprite_q: Query<(&CutsceneAnim, &mut Sprite, &mut Transform), Without<Text2d>>,
) {
    if active.current.is_none() {
        return;
    }
    let t = timer.0.elapsed_secs();
    let gf = global_fade(t);

    // ── Sprite entities ───────────────────────────────────────────────────
    for (anim, mut sprite, mut transform) in sprite_q.iter_mut() {
        match anim {
            CutsceneAnim::Overlay => {
                let alpha = tween(t, 0.0, 0.5, 0.0, 0.85) * gf;
                sprite.color = with_alpha(sprite.color, alpha);
            }

            CutsceneAnim::Explosion { num_frames } => {
                if t < 1.3 {
                    continue;
                }

                let progress = ((t - 1.3) / 0.7).clamp(0.0, 1.0);

                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = ((progress * *num_frames as f32) as usize)
                        .min(num_frames - 1);
                }

                let alpha = tween(t, 1.3, 1.45, 0.0, 1.0)
                    * tween(t, 1.8, 2.1, 1.0, 0.0)
                    * gf;

                sprite.color = with_alpha(sprite.color, alpha);
            }

            CutsceneAnim::PlanetStill => {
                // Scale in from 0 → 3 over [0.3, 0.9], then fade out before impact.
                let scale = tween(t, 0.3, 0.9, 0.0, 3.0);
                transform.scale = Vec3::splat(scale);
                let alpha = tween(t, 0.3, 0.9, 0.0, 1.0)  // fade in
                    * tween(t, 1.1, 1.5, 1.0, 0.0)   // fade out as destruction begins
                    * gf;
                sprite.color = with_alpha(sprite.color, alpha);
            }

            CutsceneAnim::PlanetDestruction { num_frames } => {
                if t < 1.3 {
                    continue;   // skip this entity, keep iterating others
                }
                // Advance atlas frame: full animation plays over 0.7 s from impact.
                let progress = ((t - 1.3) / 0.7).clamp(0.0, 1.0);
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = ((progress * *num_frames as f32) as usize)
                        .min(num_frames - 1);
                }
                let alpha = tween(t, 1.3, 1.5, 0.0, 1.0) * gf;
                sprite.color = with_alpha(sprite.color, alpha);
            }

            CutsceneAnim::AsteroidStill => {
                // Scale in from 0 → 3 over [0.3, 0.9], then fade out before impact.
                let scale = tween(t, 0.3, 0.9, 0.0, 3.0);
                transform.scale = Vec3::splat(scale);
                let alpha = tween(t, 0.3, 0.9, 0.0, 1.0)  // fade in
                    * tween(t, 1.1, 1.5, 1.0, 0.0)   // fade out as destruction begins
                    * gf;
                sprite.color = with_alpha(sprite.color, alpha);
            }

            CutsceneAnim::AsteroidDestruction { num_frames } => {
                if t < 1.3 {
                    continue;   // skip this entity, keep iterating others
                }
                if let Some(atlas) = &mut sprite.texture_atlas {
                    let frame = ((t - 0.3).max(0.0) * ASTEROID_ANIMATION_FPS) as usize;
                    atlas.index = frame % num_frames;
                }
                let alpha = tween(t, 1.3, 1.5, 0.0, 1.0) * gf;
                sprite.color = with_alpha(sprite.color, alpha);
            }

            CutsceneAnim::IncomingAsteroidSprite { num_frames } => {
                // Move from x = 700 (off-screen right) to x = 80 (nose inside asteroid at x=0).
                let x = tween(t, 0.3, 1.3, 700.0, 80.0);
                transform.translation.x = x;
                let alpha = if t < 1.3 {
                    tween(t, 0.3, 0.6, 0.0, 1.0)
                } else {
                    tween(t, 1.3, 1.6, 1.0, 0.0)
                } * gf;
                sprite.color = with_alpha(sprite.color, alpha);
                // Advance atlas frame: full animation plays over 0.7 s from impact.
                if let Some(atlas) = &mut sprite.texture_atlas {
                    let frame = ((t - 0.3).max(0.0) * ASTEROID_ANIMATION_FPS) as usize;
                    atlas.index = frame % num_frames;
                }
            }

            CutsceneAnim::IncomingRocketSprite => {
                // Move from x = 700 (off-screen right) to x = 80 (nose inside asteroid at x=0).
                let x = tween(t, 0.3, 1.3, 700.0, 80.0);
                transform.translation.x = x;
                let alpha = if t < 1.3 {
                    tween(t, 0.3, 0.6, 0.0, 1.0)
                } else {
                    tween(t, 1.3, 1.6, 1.0, 0.0)
                } * gf;
                sprite.color = with_alpha(sprite.color, alpha);
            }

            CutsceneAnim::Caption => {} // I like it to be still
        }
    }
}

pub fn update_cutscene(
    time: Res<Time>,
    mut timer: ResMut<CutsceneTimer>,
    mut active_cutscene: ResMut<ActiveCutscene>,
    mut commands: Commands,
    entities: Query<Entity, With<CutsceneEntity>>,
) {
    if active_cutscene.current.is_none() {
        return;
    }

    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        for entity in entities.iter() {
            commands.entity(entity).despawn();
        }
        // Start the next queued cutscene (or None if the queue is empty).
        active_cutscene.current = active_cutscene.pending.pop_front();
        active_cutscene.spawned = false;
        timer.0.reset();
    }
}

pub fn despawn_cutscene(mut commands: Commands, query: Query<Entity, With<CutsceneEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}