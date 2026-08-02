use bevy::{
    color::palettes::{
        css::WHITE,
        tailwind::{NEUTRAL_900, RED_600},
    },
    input_focus::InputFocus,
    picking::hover::Hovered,
    prelude::*,
};

use crate::{gameplay::{GAME_HEIGHT, GameState, MainCamera, Parallax}, utils::repeat_texture_settings};

#[derive(EntityEvent)]
struct HoverEnter(Entity);

#[derive(EntityEvent)]
struct HoverExit(Entity);

#[derive(EntityEvent)]
struct ButtonPressed(Entity);

#[derive(EntityEvent)]
struct ButtonReleased(Entity);

#[derive(Component, Default)]
pub struct Hoverable {
    pub hovered: bool,
}

#[derive(Component, Default)]
pub struct SpriteButton {
    pub normal_frame_idx: usize,
    pub hover_frame_idx: usize,
    pub pressed_frame_idx: usize,

    is_pressed: bool,
}

#[derive(Component)]
struct StartBtn;

pub struct GameUIPlugin;

#[derive(Resource, Default)]
pub struct MousePos {
    pub pos: Vec2,
    pub last_pos: Vec2,
}

#[derive(Component)]
struct Test;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputFocus>();
        app.init_resource::<MousePos>();
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu_ui);
        app.add_systems(
            Update,
            (
                calculate_mouse_pos,
                check_hovered_sprites,
                update_sprite_buttons,
            )
                .chain(),
        );

        app.add_observer(
            |_: On<ButtonReleased>,
             _: Single<(), With<StartBtn>>,
             mut next: ResMut<NextState<GameState>>| {
                next.set(GameState::InGame);
            },
        );
    }
}

#[derive(Component)]
struct Title;

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
        Test {},
        DespawnOnExit(GameState::MainMenu),
        StartBtn,
    ));

    commands.spawn((
        Parallax { speed: 70. },
        Transform::from_translation(Vec3::new(0., -GAME_HEIGHT / 2.0 + 56.0 / 2.0, 0.)),
        Sprite::from_image(
            assets
                .load_builder()
                .with_settings(|s: &mut _| *s = repeat_texture_settings())
                .load("ground.png"),
        ),
        DespawnOnExit(GameState::MainMenu),
    ));
    info!("Setting up UI");
}

fn calculate_mouse_pos(
    mut mp: ResMut<MousePos>,
    window: Single<&Window>,
    q_camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Some(pos) = window.cursor_position() else {
        return;
    };
    let (camera, camera_transform) = *q_camera;

    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, pos) else {
        return;
    };

    mp.last_pos = mp.pos;
    mp.pos = world_pos;
}

fn check_hovered_sprites(
    mut commands: Commands,
    mut q_sprites: Query<(
        Entity,
        Option<&Name>,
        &Sprite,
        &GlobalTransform,
        &mut Hoverable,
    )>,
    images: Res<Assets<Image>>,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    mouse_pos: Res<MousePos>,
) {
    for (id, name, sprite, transform, mut hoverable) in &mut q_sprites {
        // Use custom_size if set, otherwise fallback to the custom rect size, and then image asset size
        let size = sprite
            .custom_size
            .or_else(|| {
                sprite.texture_atlas.as_ref().and_then(|atlas| {
                    atlas
                        .texture_rect(&texture_atlases)
                        .and_then(|r| Some(r.size().as_vec2()))
                })
            })
            .or_else(|| sprite.rect.and_then(|r| Some(r.size())))
            .unwrap_or_else(|| images.get(&sprite.image).unwrap().size().as_vec2());

        // Apply global transform scale
        let scale = transform.scale().truncate();
        let scaled_size = size * scale;

        // Center position in world coordinates
        let translation = transform.translation().truncate();

        // Final bounding box rectangle
        let bounds = Rect::from_center_size(translation, scaled_size);

        if bounds.contains(mouse_pos.pos) && !hoverable.hovered {
            commands.trigger(HoverEnter(id));
            hoverable.hovered = true;
        } else if !bounds.contains(mouse_pos.pos) && hoverable.hovered {
            commands.trigger(HoverExit(id));
            hoverable.hovered = false;
        }
    }
}

fn update_sprite_buttons(
    mut commands: Commands,
    mut q_buttons: Query<(Entity, &mut SpriteButton, &mut Sprite, &Hoverable)>,
    mouse_btns: Res<ButtonInput<MouseButton>>,
) {
    for (entity, mut btn, mut spr, hover) in &mut q_buttons {
        let Some(atlas) = spr.texture_atlas.as_mut() else {
            continue;
        };

        if mouse_btns.pressed(MouseButton::Left) && !btn.is_pressed {
            atlas.index = btn.pressed_frame_idx;
            btn.is_pressed = true;
            commands.trigger(ButtonPressed(entity));
        } else if !mouse_btns.pressed(MouseButton::Left) && btn.is_pressed {
            // We don't need to set atlas index here, as it'll already be set below
            btn.is_pressed = false;
            commands.trigger(ButtonReleased(entity));
        }

        if !btn.is_pressed {
            atlas.index = if hover.hovered {
                btn.hover_frame_idx
            } else {
                btn.normal_frame_idx
            };
        }
    }
}
