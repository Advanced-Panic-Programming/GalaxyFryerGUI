use bevy::prelude::*;

/// Marks every entity that belongs to the currently-playing cutscene.
/// in order to despawn everything when the timer expires
#[derive(Component)]
pub struct CutsceneEntity;

#[derive(Component)]
pub enum CutsceneAnim {
    /// Full-screen black overlay — fades in over [0, 0.5] to `peak_alpha`,
    /// then the global fade-out takes over at 2.7 s.
    Overlay,

    /// Intact planet sprite: scales in [0.3, 0.9], fades out [1.1, 1.5]
    /// as the destruction animation takes over.
    PlanetStill,

    /// Break-apart animation (destroy_planet{i}.png, NUM_DESTROY_FRAMES frames):
    /// starts at t=1.3, plays through to t=2.0, then holds last frame.
    PlanetDestruction { num_frames: usize },

    Explosion {
        num_frames: usize,
    },

    /// Intact asteroid sprite: scales in [0.3, 0.9], fades out [1.1, 1.5]
    /// as the destruction animation takes over.
    AsteroidStill,

    /// Break-apart animation of the asteroid
    /// starts at t=1.3, plays through to t=2.0, then holds last frame.
    AsteroidDestruction { num_frames: usize },

    /// Asteroid sprite: fades in [0.3, 0.6], moves right→left [0.3, 1.3],
    /// fades out [1.3, 1.6].
    IncomingAsteroidSprite {
        num_frames: usize,
    },

    /// Rocket sprite: fades in [0.3, 0.6], moves right→left [0.3, 1.3],
    /// fades out [1.3, 1.6].
    IncomingRocketSprite,

    /// Text caption.  Alpha is driven separately through `TextColor`.
    Caption,
}