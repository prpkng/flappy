use bevy::{prelude::*, text::FontSourceTemplate::Math};
use rand::{RngExt, SeedableRng};

use crate::{
    gameplay::AABB,
    gameplay::{GAME_HEIGHT, GAME_WIDTH},
};

const OPENING_SIZE: f32 = 70.;

const SCROLLING_SPEED: f32 = 30.0;
const SPAWN_FREQUENCY: f32 = 4.;
const SPAWN_MIN_Y: f32 = -GAME_HEIGHT / 2. + OPENING_SIZE + 16.;
const SPAWN_MAX_Y: f32 = GAME_HEIGHT / 2. - OPENING_SIZE + 16.;

#[derive(Component)]
pub struct Pipe;

#[derive(Resource)]
struct PipeSpawner {
    counter: f32,
    spawned_pipes: i32,
    rng: rand::rngs::SmallRng,
}

#[derive(Event)]
struct SpawnPipeEvent {
    pipe_y: f32,
    index: i32,
}

pub struct PipePlugin;

#[derive(Bundle)]
struct PipeBundle {
    pipe: Pipe,
    transform: Transform,
    sprite: Sprite,
}

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_pipes, free_pipes, pipe_scrolling));
        app.insert_resource(PipeSpawner {
            counter: f32::INFINITY,
            spawned_pipes: 0,
            rng: rand::make_rng(),
        });
        app.add_observer(on_pipe_spawned);
    }
}

fn on_pipe_spawned(
    event: On<SpawnPipeEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let entity = commands
        .spawn((
            Name::new(format!("Pipes {}", event.index)), 
            Transform::from_translation(Vec3::new(GAME_WIDTH / 2.0 + 16.0, 0.0, 0.0)),
            Pipe{},
        ))
        .id();
    // commands.spawn((
    //     Transform::from_translation(Vec3::new(GAME_WIDTH / 2.0 + 16.0, event.pipe_y, 10.0)),
    //     Sprite::from_image(asset_server.load("bird1.png")),
    //     Pipe{},
    // ));

    // Spawn top pipe
    let size = Vec2::new(
        32.,
        (GAME_HEIGHT / 2.0 - event.pipe_y).abs() - OPENING_SIZE / 2.0,
    );
    commands
        .spawn(PipeBundle {
            pipe: Pipe {},
            transform: Transform::from_translation(Vec3::new(
                0.0,
                GAME_HEIGHT / 2. - size.y / 2.0,
                8.0,
            )),
            sprite: Sprite {
                image: asset_server.load("pipe.png"),
                rect: Some(Rect::new(0., 16., 32., 16.)),
                custom_size: Some(size),
                ..default()
            },
        })
        .insert(AABB {
            rect: Rect::from_center_size(Vec2::ZERO, Vec2::new(size.x - 8.0, size.y - 4.0)),
        })
        .insert(ChildOf(entity));

    // Spawn bottom pipe
    let size = Vec2::new(
        32.,
        (-GAME_HEIGHT / 2.0 - event.pipe_y).abs() - OPENING_SIZE / 2.0,
    );
    commands
        .spawn(PipeBundle {
            pipe: Pipe {},
            transform: Transform::from_translation(Vec3::new(
                0.0,
                -GAME_HEIGHT / 2. + size.y / 2.0,
                8.0,
            )),
            sprite: Sprite {
                image: asset_server.load("pipe.png"),
                rect: Some(Rect::new(0., 16., 32., 16.)),
                custom_size: Some(size),
                ..default()
            },
        })
        .insert(AABB {
            rect: Rect::from_center_size(Vec2::ZERO, Vec2::new(size.x - 8.0, size.y - 4.0)),
        })
        .insert(ChildOf(entity));

    // Spawn top pipe cap
    commands
        .spawn(PipeBundle {
            pipe: Pipe {},
            transform: Transform::from_translation(Vec3::new(
                0.0,
                event.pipe_y + OPENING_SIZE / 2.0 + 8.0,
                10.0,
            )),
            sprite: Sprite {
                image: asset_server.load("pipe.png"),
                rect: Some(Rect::new(0., 0., 32., 16.)),
                flip_y: true,
                ..default()
            },
        })
        .insert(ChildOf(entity));

    // Spawn top pipe cap
    commands
        .spawn(PipeBundle {
            pipe: Pipe {},
            transform: Transform::from_translation(Vec3::new(
                0.0,
                event.pipe_y - OPENING_SIZE / 2.0 - 8.0,
                10.0,
            )),
            sprite: Sprite {
                image: asset_server.load("pipe.png"),
                rect: Some(Rect::new(0., 0., 32., 16.)),
                flip_y: false,
                ..default()
            },
        })
        .insert(ChildOf(entity));

    println!("Spawning pipe at {}", event.pipe_y);

    // // Spawn top pipe
    // let size = Vec2::new(32., (GAME_HEIGHT/2.0 - event.pipe_y).abs() - OPENING_SIZE/2.0);
    // commands.spawn(PipeBundle {
    //     pipe: Pipe {},
    //     transform: Transform::from_translation(Vec3::new(
    //         GAME_WIDTH / 2. + size.x/2.0,
    //         -GAME_HEIGHT / 2. + size.y/2.0,
    //         10.0,
    //     )),
    //     sprite: Sprite {
    //         image: asset_server.load("pipe.png"),
    //         rect: Some(Rect::new(0., 16., 32., 16.)),
    //         custom_size: Some(size),
    //         ..default()
    //     },
    // });
}

fn spawn_pipes(mut commands: Commands, time: Res<Time>, mut spawner: ResMut<PipeSpawner>) {
    spawner.counter += time.delta_secs();

    if spawner.counter < SPAWN_FREQUENCY {
        return;
    }
    spawner.counter = 0.;

    let y = spawner.rng.random_range(SPAWN_MIN_Y..SPAWN_MAX_Y);
    commands.trigger(SpawnPipeEvent {
        pipe_y: y,
        index: spawner.spawned_pipes,
    });
    spawner.spawned_pipes += 1;
}

fn free_pipes(mut commands: Commands, query: Query<(Entity, &Name, &Transform), (With<Pipe>, With<Children>)>) {
    for (entity, name, trans) in query.iter() {
        if trans.translation.x > -GAME_WIDTH / 2.0 - 32.0 {
            continue;
        }

        commands.entity(entity).despawn();

        info!("Freed {}", name);
    }
}

fn pipe_scrolling(mut query: Query<(&mut Transform), (With<Pipe>, With<Children>)>, time: Res<Time>) {
    for (mut trans) in query.iter_mut() {
        trans.translation.x -= time.delta_secs() * SCROLLING_SPEED;
    }
}
