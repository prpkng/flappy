use bevy::{
    color::palettes::{
        css::WHITE,
        tailwind::{NEUTRAL_900, RED_600},
    },
    input_focus::InputFocus,
    picking::hover::Hovered,
    prelude::*,
};

use crate::{gameplay::{GAME_HEIGHT, GameState, MainCamera, Parallax}, utilities::repeat_texture_settings};

#[derive(EntityEvent)]
pub struct HoverEnter(Entity);

#[derive(EntityEvent)]
pub struct HoverExit(Entity);

#[derive(EntityEvent)]
pub struct ButtonPressed(Entity);

#[derive(EntityEvent)]
pub struct ButtonReleased(Entity);

#[derive(Component, Default)]
pub struct Hoverable {
    pub hovered: bool,
}

#[derive(Component, Default)]
pub struct SpriteButton {
    pub normal_frame_idx: usize,
    pub hover_frame_idx: usize,
    pub pressed_frame_idx: usize,

    pub is_pressed: bool,
}

#[derive(Component)]
pub enum MenuButtonAction {
    StartGame,
    TryAgain
}

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
         action: Single<&MenuButtonAction>,
         mut next: ResMut<NextState<GameState>>| {
            match *action {
                MenuButtonAction::StartGame => next.set(GameState::Preparing),
                MenuButtonAction::TryAgain => next.set(GameState::Preparing),
            };
            
        },
    );
        
    }
}

#[derive(Component)]
pub struct Title;



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
