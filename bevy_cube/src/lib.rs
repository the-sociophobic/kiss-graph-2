use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(bevy_webgl2::WebGL2Plugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(3.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: Mesh::from(shape::Cube { size: 1.0 }),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        ..Default::default()
    });

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
