use bevy::{input_focus::InputFocus, picking::hover::Hovered, prelude::*};

use crate::gameplay::{GameState, MainCamera};

#[derive(Message)]
struct HoverMessage {
    entity: Entity,
    hovered: bool,
}

#[derive(Component, Default)]
pub struct Hoverable {
    pub hovered: bool,
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
        app.add_message::<HoverMessage>();
        app.add_systems(Startup, setup_main_menu_ui);
        app.add_systems(Update, (check_hovered_sprites, calculate_mouse_pos));
    }
}

fn setup_main_menu_ui(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        Name::new("Buttons"),
        Transform::from_xyz(0., -32., 20.),
        Sprite {
            image: assets.load("buttons.png"),
            rect: Some(Rect::new(0., 16. * 4., 48., 16. * 5.)),
            ..default()
        },
        Hoverable::default(),
        Test {},
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
    mut q_sprites: Query<(Entity, Option<&Name>, &Sprite, &GlobalTransform, &mut Hoverable)>,
    images: Res<Assets<Image>>,
    mouse_pos: Res<MousePos>,
    mut writer: MessageWriter<HoverMessage>,
) {
    for (id, name, sprite, transform, mut hoverable) in &mut q_sprites {
        // Use custom_size if set, otherwise fallback to the raw image asset size
        let size = sprite
            .custom_size
            .unwrap_or_else(|| images.get(&sprite.image).unwrap().size().as_vec2());

        // Apply global transform scale
        let scale = transform.scale().truncate();
        let scaled_size = size * scale;

        // Center position in world coordinates
        let translation = transform.translation().truncate();

        // Final bounding box rectangle
        let bounds = Rect::from_center_size(translation, scaled_size);
        // TODO fix bounds computation for hover
        if bounds.contains(mouse_pos.pos) && !hoverable.hovered {
            // Starting hover
            info!("hovered sprite {:?}", name.and_then(|name| Some(name.to_string())).unwrap_or(id.to_string()));
            writer.write(HoverMessage {
                entity: id,
                hovered: true,
            });
            hoverable.hovered = true;
        } else if !bounds.contains(mouse_pos.pos) && hoverable.hovered {
            // Finishing hover
            info!("unhovered sprite {:?}", name.and_then(|name| Some(name.to_string())).unwrap_or(id.to_string()));
            writer.write(HoverMessage {
                entity: id,
                hovered: false,
            });
            hoverable.hovered = false;
        }
    }
}

fn check_hover_system(query: Query<(Entity, &Hovered), Changed<Hovered>>) {
    for (entity, hovered) in &query {
        if hovered.get() {
            println!("Entity {:?} is now being hovered!", entity);
        } else {
            println!("Mouse left entity {:?}", entity);
        }
    }
}
