use bevy::prelude::*;

/// Marks every entity that belongs to the currently-playing cutscene.
/// in order to despawn everything when the timer expires
#[derive(Component)]
pub struct CutsceneEntity;

#[derive(Component)]
pub enum CutsceneAnim {
    /// Full-screen black overlay
    Overlay,

    /// Intact planet sprite: scales in then fades out as the destruction animation takes over.
    PlanetStill,

    /// Break-apart animation (destroy_planet{i}.png, NUM_DESTROY_FRAMES frames):
    /// starts at t=1.3, plays through to t=2.0, then holds last frame.
    PlanetDestruction { num_frames: usize },

    /// Explosion animation played after the rocket/asteroid reaches the screen center
    Explosion {
        num_frames: usize,
    },

    /// Intact asteroid sprite: scales in then fades out as the destruction animation takes over.
    AsteroidStill,

    /// Break-apart animation of the asteroid
    /// starts at t=1.3, plays through to t=2.0, then holds last frame.
    AsteroidDestruction { num_frames: usize },

    /// Asteroid sprite: fades in then moves right→left and fades out after destruction.
    IncomingAsteroidSprite {
        num_frames: usize,
    },

    /// Rocket sprite: fades in, moves right→left and fades out.
    IncomingRocketSprite,

    /// Text caption.  Alpha is driven separately through `TextColor`.
    Caption,
}