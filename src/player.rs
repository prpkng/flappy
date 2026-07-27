use bevy::{math::VectorSpace, prelude::*};

use crate::gameplay::GAME_HEIGHT;

const GRAVITY: f32 = 140.;

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

        app.add_systems(Update, player_gravity);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player {},
        Velocity::default(),
        Transform::from_translation(Vec3::new(0., 0., 10.)),
        Sprite::from_image(asset_server.load("bird1.png")),
    ));
}

fn player_gravity(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation.x += velocity.dx * time.delta_secs();
        transform.translation.y += velocity.dy * time.delta_secs();

        velocity.dy -= GRAVITY * time.delta_secs();

        transform.translation.y = transform
            .translation
            .y
            .clamp(-GAME_HEIGHT / 2.0, GAME_HEIGHT / 2.0);
    }
}
