// sprite_renderer.rs

use bevy::prelude::*;

pub fn draw_sprite(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
    name: &str,
) {
    let sprite_handle = asset_server.load(format!("{}.png", name).as_str());

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(sprite_handle.into()),
        transform: Transform::from_translation(position),
        ..Default::default()
    });
}
