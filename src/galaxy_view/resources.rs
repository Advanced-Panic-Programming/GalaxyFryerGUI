use bevy::prelude::*;

#[derive(Resource)]
pub struct ExplorerArrowAtlas {
    pub layout: Handle<TextureAtlasLayout>,
}