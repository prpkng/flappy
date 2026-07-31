use bevy::{
    camera::{ScalingMode, Viewport},
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};
use bevy_simple_screen_boxing::{CameraBox, CameraBox::ResolutionIntegerScale, CameraBoxingPlugin};

pub const GAME_WIDTH: f32 = 144.0;
pub const GAME_HEIGHT: f32 = 256.0;

pub struct GameplayPlugin;

#[derive(Component)]
pub struct AABB {
    pub rect: Rect,
}
#[derive(Component)]
struct Parallax {
    speed: f32,
}

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, parallax_system);
        app.add_plugins(CameraBoxingPlugin);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut projection = OrthographicProjection::default_2d();
    projection.scaling_mode = ScalingMode::Fixed {
        width: GAME_WIDTH,
        height: GAME_HEIGHT,
    };

    // Spawn letter-boxing camera
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        CameraBox::ResolutionIntegerScale {
            resolution: Vec2::new(GAME_WIDTH, GAME_HEIGHT),
            allow_imperfect_downscaled_boxing: false,
        },
        Projection::Orthographic(projection),
    ));

    // Spawn background

    commands.spawn((
        Parallax { speed: 20. },
        Sprite::from_image(
            asset_server
                .load_builder()
                .with_settings(|s: &mut _| {
                    *s = ImageLoaderSettings {
                        sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                            // rewriting mode to repeat image,
                            address_mode_u: ImageAddressMode::Repeat,
                            address_mode_v: ImageAddressMode::Repeat,
                            ..default()
                        }),
                        ..default()
                    }
                })
                .load("background.png"),
        ),
    ));
}

fn parallax_system(
    mut query: Query<(&mut Sprite, &Parallax)>,
    time: Res<Time>,
    images: Res<Assets<Image>>,
) {

    for (mut spr, parallax) in query {
        if let Some(rect) = spr.rect.as_mut() {
            rect.min.x += parallax.speed * time.delta_secs();
            rect.max.x += parallax.speed * time.delta_secs();
        } else {
            let img = images.get(&spr.image).unwrap();

            spr.rect = Some(Rect::new(0., 0., img.width() as f32, img.height() as f32));
            // let img = asset_server.get(spr.image)
        }
    }
}
