// line_renderer.rs

use bevy::prelude::*;

pub fn draw_line(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    start: Vec3,
    end: Vec3,
    start_color: Color,
    end_color: Color,
) {
    let points = [start, end];
    let colors = [start_color, end_color];

    let mesh = Mesh::from(shape::Line::new(&points, &colors));

    commands.spawn_bundle(PbrBundle {
        mesh: mesh,
        material: materials.add(StandardMaterial::from(Color::WHITE)),
        ..Default::default()
    });
}
