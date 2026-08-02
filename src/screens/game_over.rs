use bevy::prelude::*;

use crate::{gameplay::GameState, ui::Title};

pub fn setup(app: &mut App) {
    app.add_systems(OnEnter(GameState::GameOver), setup_gameover_screen);
}

fn setup_gameover_screen(mut commands: Commands, mut assets: ResMut<AssetServer>) {
    commands.spawn((
        Name::new("Game Over Title"),
        Transform::from_xyz(0., 64., 10.),
        Sprite {
            image: assets.load("titles.png"),
            rect: Some(Rect::new(0., 32., 96., 64.)),
            ..default()
        },
        Title {},
        DespawnOnExit(GameState::GameOver),
    ));
}
