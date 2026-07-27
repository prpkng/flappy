use bevy::{prelude::*, text::FontSourceTemplate::Math};
use rand::{RngExt, SeedableRng};

use crate::gameplay::{GAME_HEIGHT, GAME_WIDTH};

const OPENING_SIZE: f32 = 60.;

const SCROLLING_SPEED: f32 = 30.0;
const SPAWN_FREQUENCY: f32 = 4.;
const SPAWN_MIN_Y: f32 = -GAME_HEIGHT / 2. + 16.;
const SPAWN_MAX_Y: f32 = GAME_HEIGHT / 2. + 16.;

#[derive(Component)]
struct Pipe;

#[derive(Resource)]
struct PipeSpawner {
    counter: f32,
    rng: rand::rngs::SmallRng,
}

#[derive(Event)]
struct SpawnPipeEvent {
    pipe_y: f32,
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
        app.add_systems(Update, (spawn_pipes, pipe_scrolling));
        app.insert_resource(PipeSpawner {
            counter: f32::INFINITY,
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
    // Spawn top pipe
    commands.spawn(PipeBundle {
        pipe: Pipe {},
        transform: Transform::from_translation(Vec3::new(
            GAME_WIDTH / 2. + 16.,
            -GAME_HEIGHT / 2.,
            10.0,
        )),
        sprite: Sprite {
            image: asset_server.load("pipe.png"),
            rect: Some(Rect::new(0., 16., 32., 16.)),
            custom_size: Some(Vec2::new(32., (event.pipe_y - SPAWN_MIN_Y).abs())),
            ..default()
        },
    });
    // Spawn top pipe
    commands.spawn(PipeBundle {
        pipe: Pipe {},
        transform: Transform::from_translation(Vec3::new(
            GAME_WIDTH / 2. + 16.,
            -GAME_HEIGHT / 2.,
            10.0,
        )),
        sprite: Sprite {
            image: asset_server.load("pipe.png"),
            rect: Some(Rect::new(0., 16., 32., 16.)),
            custom_size: Some(Vec2::new(32., (event.pipe_y - SPAWN_MIN_Y).abs())),
            ..default()
        },
    });
}

fn spawn_pipes(mut commands: Commands, time: Res<Time>, mut spawner: ResMut<PipeSpawner>) {
    spawner.counter += time.delta_secs();

    if spawner.counter < SPAWN_FREQUENCY {
        return;
    }
    spawner.counter = 0.;

    let y = spawner.rng.random_range(SPAWN_MIN_Y..SPAWN_MAX_Y);
    commands.trigger(SpawnPipeEvent { pipe_y: y });
}

fn pipe_scrolling(mut query: Query<(&mut Transform), With<Pipe>>, time: Res<Time>) {
    for (mut trans) in query.iter_mut() {
        trans.translation.x -= time.delta_secs() * SCROLLING_SPEED;
    }
}
