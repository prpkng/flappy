use bevy::prelude::*;

use crate::{gameplay::GameState, ui::{Hoverable, MenuButtonAction, SpriteButton, Title}};

pub fn setup(app: &mut App) {
    app.add_systems(OnEnter(GameState::GameOver), setup_gameover_screen);
}

pub fn reset(mut cmd: Commands) {
    
}

fn setup_gameover_screen(
    mut commands: Commands,
    mut assets: ResMut<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("Game Over Title"),
        Transform::from_xyz(0., 64., 100.),
        Sprite {
            image: assets.load("titles.png"),
            rect: Some(Rect::new(0., 32., 96., 64.)),
            ..default()
        },
        Title {},
        DespawnOnExit(GameState::GameOver),
    ));

    let layout = TextureAtlasLayout::from_grid(UVec2::new(48, 16), 3, 6, None, None);
    let atlas_layout = atlases.add(layout);

    commands.spawn((
        Name::new("Buttons"),
        Transform::from_xyz(0., -32., 20.),
        Sprite {
            image: assets.load("buttons.png"),
            texture_atlas: Some(TextureAtlas {
                layout: atlas_layout,
                index: 15,
            }),
            ..default()
        },
        SpriteButton {
            normal_frame_idx: 15,
            hover_frame_idx: 16,
            pressed_frame_idx: 17,
            ..Default::default()
        },
        Hoverable::default(),
        DespawnOnExit(GameState::GameOver),
        MenuButtonAction::TryAgain,
    ));
}
