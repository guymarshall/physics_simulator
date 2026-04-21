use bevy::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, orbit_system)
        .run();
}

#[derive(Component)]
struct Planet {
    radius: f32,
    speed: f32,
    angle: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PointLight {
            intensity: 5_000_000.0,
            range: 100.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-10.0, 10.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.5))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.8, 0.2))),
        Transform::default(),
    ));

    // TODO: refactor into separate Planet struct with gravitational formulae
    let planet_data: [(f32, f32, f32); 4] = [
        (3.0, 1.0, 0.3),
        (5.0, 0.7, 0.5),
        (7.0, 0.5, 0.7),
        (9.0, 0.3, 0.9),
    ];

    for (radius, speed, size) in planet_data {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(size))),
            MeshMaterial3d(materials.add(Color::srgb(0.2 + size, 0.4, 1.0 - size))),
            Transform::from_xyz(radius, 0.0, 0.0),
            Planet {
                radius,
                speed,
                angle: 0.0,
            },
        ));
    }
}

fn orbit_system(time: Res<Time>, mut query: Query<(&mut Transform, &mut Planet)>) {
    for (mut transform, mut planet) in &mut query {
        planet.angle += time.delta_secs() * planet.speed;

        let x: f32 = planet.radius * planet.angle.cos();
        let z: f32 = planet.radius * planet.angle.sin();

        transform.translation = Vec3::new(x, 0.0, z);

        transform.rotate_y(PI * time.delta_secs());
    }
}
