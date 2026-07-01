use bevy::color::Color;

pub const FONT_PATH: &str = "fonts/BoldPixel.ttf";
pub const CAPTION_POSITION_Y: f32 = 300.0;
pub const ASTEROID_SPRITE_PATH: &str = "planets/asteroid.png";
pub const DESTROYED_ASTEROID_SPRITE_PATH: &str = "planets/destroyedAsteroid.png";
pub const ASTEROID_ANIMATION_FRAMES: usize = 144;
pub const ASTEROID_ANIMATION_FPS: f32 = 24.0;
pub const EXPLOSION_SPRITE_PATH: &str = "effects/explosion_96x96_12_frames.png";
pub const EXPLOSION_ANIMATION_FRAMES: usize = 12;
pub const EXPLOSION_SPRITE_SCALE: f32 = 4.0;
pub const PLANET_SPRITE_SCALE: f32 = 3.0;
pub const ASTEROID_SPRITE_SCALE: f32 = 3.5;
pub const ROCKET_SPRITE_SCALE: f32 = 0.2;


/// Linear interpolation between `v0` and `v1` over the interval [t0, t1].
pub fn tween(elapsed: f32, t0: f32, t1: f32, v0: f32, v1: f32) -> f32 {
    if elapsed <= t0 {
        return v0;
    }
    if elapsed >= t1 {
        return v1;
    }
    v0 + (v1 - v0) * (elapsed - t0) / (t1 - t0)
}

/// Multiplier [1 → 0] applied to every entity's alpha from 2.7 s onward,
/// making the whole scene fade to black gracefully before the timer ends.
pub fn global_fade(elapsed: f32) -> f32 {
    tween(elapsed, 2.7, 3.0, 1.0, 0.0)
}

/// Read the RGB channels of the current sprite color, replace only the alpha.
pub fn with_alpha(color: Color, alpha: f32) -> Color {
    let s = color.to_srgba();
    Color::srgba(s.red, s.green, s.blue, alpha.clamp(0.0, 1.0))
}