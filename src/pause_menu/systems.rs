use bevy::prelude::*;
use crate::app_state_manager::messages::PlayPressed;

pub fn send_play_pressed(
    mut reader: MessageReader<PlayPressed>,
    mut writer: MessageWriter<PlayPressed>)
{
    // if !reader.is_empty() {
    //     reader.clear();
    //     writer.write(PlayPressed);
    // }
    writer.write(PlayPressed);
}