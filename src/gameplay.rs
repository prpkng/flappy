use bevy::{
    camera::{ScalingMode, Viewport},
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};
use bevy_simple_screen_boxing::{CameraBox, CameraBox::ResolutionIntegerScale, CameraBoxingPlugin};

use crate::utils::repeat_texture_settings;

pub const GAME_WIDTH: f32 = 144.0;
pub const GAME_HEIGHT: f32 = 256.0;


#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Preparing,
    InGame,
    // GameOver
}

#[derive(Resource, Default)]
pub struct GameInfo {
    // pub score: u32,
}

#[derive(Component)]
pub struct AABB {
    pub rect: Rect,
}

#[derive(Component)]
pub struct Parallax {
    pub speed: f32,
}

#[derive(Component)]
pub struct MainCamera;


pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup);
        app.add_plugins(CameraBoxingPlugin);
        
        app.init_state::<GameState>();
        app.insert_resource(GameInfo::default());
        app.add_systems(Update, parallax_system);
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
        MainCamera{},
    ));

    commands.spawn((
        Parallax { speed: 15. },
        Sprite::from_image(
            asset_server
                .load_builder()
                .with_settings(|s: &mut _| { *s = repeat_texture_settings()} )
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
