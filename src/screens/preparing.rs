use bevy::prelude::*;

use crate::{gameplay::GameState, ui::Title};

pub fn setup(app: &mut App) {
    app.add_systems(OnEnter(GameState::Preparing), setup_preparing_screen);
}

fn setup_preparing_screen(mut commands: Commands, mut assets: ResMut<AssetServer>) {
    commands.spawn((
        Name::new("Get Ready Title"),
        Transform::from_xyz(0., 64., 10.),
        Sprite {
            image: assets.load("titles.png"),
            rect: Some(Rect::new(0., 64., 96., 96.)),
            ..default()
        },
        Title {},
        DespawnOnExit(GameState::Preparing),
    ));
}
