use bevy::prelude::*;
use crate::app_state_manager::messages::PlayPressed;

pub fn send_play_pressed(
    mut writer: MessageWriter<PlayPressed>)
{
    writer.write(PlayPressed);
}