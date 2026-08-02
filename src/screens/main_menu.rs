use bevy::prelude::*;

use crate::{
    gameplay::GameState, ui::{ButtonReleased, Hoverable, SpriteButton, Title},
};

pub fn setup(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu_ui);
    app.add_systems(Update, check_start_keypress.run_if(in_state(GameState::MainMenu)));
    app.add_observer(
        |_: On<ButtonReleased>,
         _: Single<(), With<StartBtn>>,
         mut next: ResMut<NextState<GameState>>| {
            next.set(GameState::Preparing);
        },
    );
}

#[derive(Component)]
struct StartBtn;

fn check_start_keypress(mut next: ResMut<NextState<GameState>>, input: Res<ButtonInput<KeyCode>>) {
    if !input.just_released(KeyCode::Space) {return;}

    next.set(GameState::Preparing)
}

fn setup_main_menu_ui(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("Flappy Bird Title"),
        Transform::from_xyz(0., 64., 10.),
        Sprite {
            image: assets.load("titles.png"),
            rect: Some(Rect::new(0., 0., 96., 32.)),
            ..default()
        },
        Title {},
        DespawnOnExit(GameState::MainMenu),
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
        DespawnOnExit(GameState::MainMenu),
        StartBtn,
    ));

    info!("Setting up UI");
}
