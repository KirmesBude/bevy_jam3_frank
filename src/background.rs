use bevy::{
    prelude::*,
    render::{render_resource::SamplerDescriptor, texture::ImageSampler},
};
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BackgroundAssets>()
            .add_startup_systems((load_background_assets, spawn_background).chain())
            .add_system(repeat_background);
    }
}

#[derive(Debug, Default, Resource)]
pub struct BackgroundAssets {
    tile: Handle<Image>,
}

fn load_background_assets(
    asset_server: Res<AssetServer>,
    mut background_assets: ResMut<BackgroundAssets>,
) {
    background_assets.tile = asset_server.load("tile.png");
}

fn repeat_background(background_assets: Res<BackgroundAssets>, mut images: ResMut<Assets<Image>>) {
    if let Some(mut image) = images.get_mut(&background_assets.tile) {
        image.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
            address_mode_u: bevy::render::render_resource::AddressMode::Repeat,
            address_mode_v: bevy::render::render_resource::AddressMode::Repeat,
            address_mode_w: bevy::render::render_resource::AddressMode::Repeat,
            ..Default::default()
        });
    }
}

fn spawn_background(mut commands: Commands, background_assets: Res<BackgroundAssets>) {
    for x in -20..20 {
        for y in -20..20 {
            let transform =
                Transform::from_xyz(x as f32 * 16.0 * 10.0, y as f32 * 16.0 * 10.0, 0.0)
                    .with_scale(Vec3::splat(10.0));
            commands.spawn(SpriteBundle {
                transform,
                texture: background_assets.tile.clone_weak(),
                ..Default::default()
            });
        }
    }
}
