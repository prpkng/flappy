mod debug;
mod gameplay;
mod pipes;
mod utils;
mod player;
mod ui;

use crate::{
    debug::DebugPlugin, gameplay::{GAME_HEIGHT, GAME_WIDTH, GameplayPlugin}, pipes::PipePlugin, player::PlayerPlugin, ui::GameUIPlugin,
};
use bevy::{prelude::*, ui::UiPlugin, window::WindowResolution};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "flappy".into(),
                        name: Some("flappy".into()),
                        resolution: WindowResolution::new(GAME_WIDTH as u32 * 2, GAME_HEIGHT as u32 * 2),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((
            PlayerPlugin {},
            GameplayPlugin {},
            PipePlugin {},
            DebugPlugin {},
            GameUIPlugin {},
        ))
        .run();
}
