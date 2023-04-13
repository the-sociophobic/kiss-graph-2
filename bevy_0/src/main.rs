// main.rs

use bevy::prelude::*;
use bevy_webgl2::WebGL2Plugin;

mod data;
mod line_renderer;
mod sprite_renderer;

use crate::data::{CustomPoint, data};
use crate::line_renderer::draw_line;
use crate::sprite_renderer::draw_sprite;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugins(MinimalPlugins)
        .add_plugin(WebGL2Plugin) // Use WebGL2Plugin for WebAssembly
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    let points = data();

    for i in 0..points.len() {
        let point = &points[i];

        draw_sprite(&mut commands, &asset_server, &mut materials, point.pos, &point.name);

        for j in i + 1..points.len() {
            let next_point = &points[j];
            draw_line(
                &mut commands,
                &mut standard_materials,
                point.pos,
                next_point.pos,
                point.color,
                next_point.color,
            );
        }
    }
}
