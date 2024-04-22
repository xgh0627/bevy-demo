use bevy::prelude::*;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,spawn_scene)
        // .add_systems(Update,move_cube)
        .run();
}

#[derive(Component)]
struct Cube;

fn spawn_scene(mut commands: Commands,mut meshes: ResMut<Assets<Mesh>>,mut materials: ResMut<Assets<StandardMaterial>>) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(6.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    }).insert(Cube);
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// fn move_cube(mut query: Query<&mut Transform,With<Cube>>) {
//     let mut transform = query.single_mut();
//     transform.translation.x += 0.05;
// }