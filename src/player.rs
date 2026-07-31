use bevy::prelude::*;

use crate::{
    gameplay::{AABB, GAME_HEIGHT},
    pipes::Pipe,
    utils::InterpExt,
};

const GRAVITY: f32 = 320.0;
const MAX_FALL_SPEED: f32 = 180.0;
const JUMP_FORCE: f32 = 135.0;

pub struct PlayerPlugin {}

#[derive(Component, Default)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[derive(Component)]
struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);

        app.add_systems(
            Update,
            (
                player_gravity,
                player_input,
                player_rotation,
                player_collisions,
            ),
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player {},
        Velocity::default(),
        Transform::from_translation(Vec3::new(0., 0., 5.)),
        Sprite::from_image(asset_server.load("bird1.png")),
        AABB {
            rect: Rect::from_center_size(Vec2::ZERO, Vec2::ONE*16.0),
        },
    ));
}

fn player_input(mut query: Query<&mut Velocity, With<Player>>, input: Res<ButtonInput<KeyCode>>) {
    let Ok(mut velocity) = query.single_mut() else {
        return;
    };

    if input.just_pressed(KeyCode::Space) {
        velocity.dy = JUMP_FORCE;
    }
}

fn player_gravity(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut velocity)) = query.single_mut() else {
        return;
    };

    transform.translation.x += velocity.dx * time.delta_secs();
    transform.translation.y += velocity.dy * time.delta_secs();

    velocity.dy -= GRAVITY * time.delta_secs();
    velocity.dy = velocity.dy.max(-MAX_FALL_SPEED);

    transform.translation.y = transform
        .translation
        .y
        .clamp(-GAME_HEIGHT / 2.0, GAME_HEIGHT / 2.0);
}

fn player_rotation(mut query: Query<(&mut Transform, &Velocity), With<Player>>, time: Res<Time>) {
    let Ok((mut transform, velocity)) = query.single_mut() else {
        return;
    };

    let last_rot = transform.rotation.to_euler(EulerRot::XYZ).2;
    let target_rot = last_rot.exp_interp((velocity.dy / 4.0).to_radians(), 15.0, time.delta_secs());
    transform.rotation = Quat::from_rotation_z(target_rot);
}

fn player_collisions(
    mut q_player: Query<(&mut Transform, &AABB, &mut Velocity), With<Player>>,
    q_pipes: Query<(&AABB, &GlobalTransform), With<Pipe>>,
    q_pipe_entities: Query<Entity, (With<Pipe>, With<Children>)>,
    mut commands: Commands
) {
    let Ok((mut trans, player_aabb, mut vel)) = q_player.single_mut() else {
        return;
    };

    let mut should_reset = false;

    let player_rect = player_aabb.rect.translate(trans.translation.xy());
    for (aabb, pipe_trans) in q_pipes.iter() {
        let rect = aabb.rect.translate(pipe_trans.translation().xy());
        if rect.intersect(player_rect).is_empty() {
            continue;
        }

        info!("Player collided with pipe!");
        should_reset = true;
    }

    if !should_reset { return; }
    for entity in q_pipe_entities.iter() {
        commands.entity(entity).despawn();
    }

    trans.translation.y = 0.0;
    vel.dy = 0.0;
    
    
 }
