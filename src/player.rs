use bevy::prelude::*;

use crate::{
    gameplay::{AABB, GAME_HEIGHT, GameState},
    pipes::Pipe,
    utils::InterpExt,
};

const GRAVITY: f32 = 320.0;
const MAX_FALL_SPEED: f32 = 180.0;
const JUMP_FORCE: f32 = 135.0;

#[derive(Event)]
pub struct PlayerDeath {}

pub struct PlayerPlugin {}

#[derive(Component, Default)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct AnimationIndices {
    frames: Vec<usize>,
    current_index: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);

        app.add_systems(
            Update,
            (
                animate_sprites,
                (menu_sine_wave)
                    .run_if(not(in_state(GameState::InGame))),
                (
                    player_gravity,
                    player_input,
                    player_rotation,
                    player_collisions,
                )
                    .run_if(in_state(GameState::InGame)),
            ),
        );

        app.add_observer(on_player_death);
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("bird.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None);
    let atlas_layout = atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { frames: vec![0, 0, 1, 2, 1], current_index: 0 };

    commands.spawn((
        Player {},
        Velocity::default(),
        // Transform::from_translation(Vec3::new(-16., 0., 5.)),
        Transform::from_translation(Vec3::new(0., 0., 5.)),
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: atlas_layout,
                index: animation_indices.frames[0],
            },
        ),
        animation_indices,
        AnimationTimer(Timer::from_seconds(1.0/13.0, TimerMode::Repeating)),
        AABB {
            rect: Rect::from_center_size(Vec2::ZERO, Vec2::ONE * 16.0),
        },
    ));
}

fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (mut indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            indices.current_index = if indices.current_index >= indices.frames.len()-1 {
                0
            } else {
                indices.current_index + 1
            };
            atlas.index = indices.frames[indices.current_index];
        }
    }
}


// ==== IN MENU =====

const MENU_SINE_FREQ: f32 = 7.5;
const MENU_SINE_AMP: f32 = 4.;

fn menu_sine_wave(mut query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
    let Ok(mut transform) = query.single_mut() else { return; };

    transform.translation.y = 16. + f32::sin(time.elapsed_secs() * MENU_SINE_FREQ) * MENU_SINE_AMP;
}

// ==== IN GAME =====

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
    mut q_player: Query<(&mut Transform, &AABB), With<Player>>,
    q_pipes: Query<(&AABB, &GlobalTransform), With<Pipe>>,
    mut commands: Commands,
) {
    let Ok((mut trans, player_aabb)) = q_player.single_mut() else {
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
        commands.trigger(PlayerDeath {});
    }
}

fn on_player_death(
    event: On<PlayerDeath>,
    mut q_player: Query<(&mut Transform, &AABB, &mut Velocity), With<Player>>,
    q_pipe_entities: Query<Entity, (With<Pipe>, With<Children>)>,
    mut commands: Commands,
) {
    let Ok((mut trans, player_aabb, mut vel)) = q_player.single_mut() else {
        return;
    };
    for entity in q_pipe_entities.iter() {
        commands.entity(entity).despawn();
    }

    trans.translation.y = 0.0;
    vel.dy = 0.0;
}
