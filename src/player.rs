use std::{f32::consts::PI, time::Duration};

use bevy::{color::palettes::tailwind::RED_500, prelude::*};

use crate::{
    gameplay::{AABB, GAME_HEIGHT, GameOverEvent, GameState}, pipes::{Pipe, PipesSet}, ui::Hoverable, utilities::InterpExt,
};

const GRAVITY: f32 = 320.0;
const MAX_FALL_SPEED: f32 = 180.0;
const JUMP_FORCE: f32 = 120.0;

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
                set_player_anim_speed,
                set_player_x_pos,
                player_input,
                (menu_sine_wave)
                    .run_if(in_state(GameState::MainMenu).or_else(in_state(GameState::Preparing))),
                (player_gravity, player_rotation)
                    .run_if(in_state(GameState::InGame)),
                player_gravity.run_if(in_state(GameState::GameOver))
            ),
        );

        app.add_systems(
            Update, player_collisions.run_if(in_state(GameState::InGame)).before(PipesSet)
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
    let animation_indices = AnimationIndices {
        frames: vec![0, 0, 1, 2, 1],
        current_index: 0,
    };

    commands.spawn((
        Name::new("Player"),
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
        Hoverable::default(),
        AnimationTimer(Timer::from_seconds(1.0 / 13.0, TimerMode::Repeating)),
        AABB {
            rect: Rect::from_center_size(Vec2::ZERO, Vec2::ONE * 16.0),
        },
    ));
}

fn set_player_x_pos(
    mut trans: Single<&mut Transform, With<Player>>,
    state: Res<State<GameState>>,
    time: Res<Time>,
) {
    let target = match state.get() {
        GameState::MainMenu => 0.0,
        GameState::Preparing | GameState::InGame | GameState::GameOver => -16.,
    };
    trans.translation.x = trans
        .translation
        .x
        .exp_interp(target, 16., time.delta_secs())
}

fn set_player_anim_speed(
    mut timer: Single<&mut AnimationTimer, With<Player>>,
    mut trans_msg: MessageReader<StateTransitionEvent<GameState>>,
) {
    for msg in trans_msg.read() {
        if let Some(new) = msg.entered.as_ref() {
            if *new == GameState::GameOver {
                timer.pause();
                continue;
            }
            timer.set_duration(Duration::from_secs_f32(match new {
                GameState::MainMenu | GameState::Preparing => 1.0 / 13.0,
                _ => 1.0 / 20.0,
            }));
            if timer.is_paused() {
                timer.unpause();
            }
        }
    }
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
            indices.current_index = if indices.current_index >= indices.frames.len() - 1 {
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
    let Ok(mut transform) = query.single_mut() else {
        return;
    };

    transform.translation.y = 16. + f32::sin(time.elapsed_secs() * MENU_SINE_FREQ) * MENU_SINE_AMP;
}

// ==== IN GAME =====

fn player_input(
    mut query: Query<&mut Velocity, With<Player>>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
    kb: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let Ok(mut velocity) = query.single_mut() else {
        return;
    };

    match state.get() {
        GameState::InGame | GameState::Preparing => {
            if mouse.just_pressed(MouseButton::Left) || kb.just_pressed(KeyCode::Space) {
                velocity.dy = JUMP_FORCE;
                
            if *state.get() != GameState::InGame {
                next.set(GameState::InGame);
            }
            }
        }
        _ => {}
    };
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

    let vel = velocity.dy / 4.0;

    let t = f32::inverse_lerp(-25., -45., vel).clamp(0., 1.);

    let target_angle = f32::lerp(PI / 6.0, -PI / 2.0, t);

    let last_rot = transform.rotation.to_euler(EulerRot::XYZ).2;
    let interp_rot = last_rot.exp_interp(target_angle, 15.0, time.delta_secs());
    transform.rotation = Quat::from_rotation_z(interp_rot);
}

fn player_collisions(
    mut q_player: Query<(&mut Transform, &AABB), With<Player>>,
    q_pipes: Query<(&GlobalTransform, &AABB), (Without<Children>, With<Pipe>)>,
    mut commands: Commands,
    mut gizmos: Gizmos,
) {
    let Ok((mut trans, player_aabb)) = q_player.single_mut() else {
        return;
    };

    let mut should_reset = false;

    let player_rect = player_aabb.rect.translate(trans.translation.xy());
    for (pipe_trans, aabb) in q_pipes.iter() {
        let rect = aabb.rect.translate(pipe_trans.translation().xy());
        gizmos.rect_2d(Isometry2d::from_translation(rect.center()), rect.size(), RED_500);
        if rect.intersect(player_rect).is_empty() {
            continue;
        }

        commands.trigger(PlayerDeath {});
    }

    if trans.translation.y < -GAME_HEIGHT / 2.0 + 56. {
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
    // for entity in q_pipe_entities.iter() {
    //     commands.entity(entity).despawn();
    // }

    commands.trigger(GameOverEvent {});

    // trans.translation.y = 0.0;
    vel.dy = 0.0;
}
