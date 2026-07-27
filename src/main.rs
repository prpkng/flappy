mod gameplay;
mod pipes;
mod player;

use crate::{gameplay::GameplayPlugin, pipes::PipePlugin, player::PlayerPlugin};
use bevy::{prelude::*, window::WindowResolution};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "flappy".into(),
                        name: Some("flappy".into()),
                        resolution: WindowResolution::new(288 * 2, 1024),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((PlayerPlugin {}, GameplayPlugin {}, PipePlugin {}))
        .run();
}
