use bevy::prelude::*;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,spawn_scene)
        .run();
}

#[derive(Component)]
struct Cube;

fn spawn_scene(mut commands: Commands,mut meshes: ResMut<Assets<Mesh>>,mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
        material: materials.add(bevy::prelude::Color::BEIGE.into()),
        transform: Transform::from_xyz(0.0,0.5,0.0),
        ..default()
    })
        .insert(Cube);

    let player_camera_y_offest: f32 = 20.0;
    let player_camera_z_offest: f32 = 10.0;

    commands.spawn(Camera3dBundle{
        transform: Transform::from_xyz(0.0,player_camera_y_offest,player_camera_z_offest).looking_at(Vec3::ZERO,Vec3::Y),
        ..default()
    });

    commands.spawn(PbrBundle{
       mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(bevy::prelude::Color::SEA_GREEN.into()),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Default::default(),
        brightness: 1.0
    });
}