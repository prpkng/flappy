

use bevy::{
    math::FloatExt, utils::default,
};
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};

pub trait InterpExt {
    fn pow_interp(&self, to: Self, smoothing: f32, delta: f32) -> Self;
    fn exp_interp(&self, to: Self, lambda: f32, delta: f32) -> Self;
}

impl InterpExt for f32 {
    fn pow_interp(&self, to: Self, smoothing: f32, delta: f32) -> Self {
        self.lerp(to, 1.0 - smoothing.powf(delta))
    }

    fn exp_interp(&self, to: Self, lambda: f32, delta: f32) -> Self {
        self.lerp(to, 1.0 - f32::exp(-lambda * delta))
    }
}

// Spawn background
pub fn repeat_texture_settings() -> ImageLoaderSettings {
    ImageLoaderSettings {
        sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
            // rewriting mode to repeat image,
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            ..default()
        }),
        ..default()
    }
}
