mod debug;
mod gameplay;
mod pipes;
mod utilities;
mod player;
mod ui;
mod screens;

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
                        resolution: WindowResolution::new(GAME_WIDTH as u32 * 4, GAME_HEIGHT as u32 * 4),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        present_mode: bevy::window::PresentMode::Immediate,
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
