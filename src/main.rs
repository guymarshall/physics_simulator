mod planet;

use bevy::prelude::*;
use std::f32::consts::PI;

use crate::planet::Planet;

fn scale_radius(radius: f32) -> f32 {
    1.5 / 695_700.0 * radius
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, orbit_system)
        .run();
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
        Mesh3d(meshes.add(Sphere::new(scale_radius(695_700.0)))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.8, 0.2))),
        Transform::default(),
    ));

    let planet_data: [Planet; 8] = [
        Planet::new(3.0, 1.0, scale_radius(2_440.0), 0.0), // Mercury
        Planet::new(5.0, 0.7, scale_radius(6_052.0), 0.0), // Venus
        Planet::new(7.0, 0.5, scale_radius(6_371.0), 0.0), // Earth
        Planet::new(9.0, 0.3, scale_radius(3_390.0), 0.0), // Mars
        Planet::new(11.0, 0.2, scale_radius(69_911.0), 0.0), // Jupiter
        Planet::new(13.0, 0.18, scale_radius(58_232.0), 0.0), // Saturn
        Planet::new(15.0, 0.11, scale_radius(25_362.0), 0.0), // Uranus
        Planet::new(17.0, 0.09, scale_radius(24_622.0), 0.0), // Neptune
    ];

    for planet in planet_data {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(planet.size))),
            MeshMaterial3d(materials.add(Color::srgb(0.2 + planet.size, 0.4, 1.0 - planet.size))),
            Transform::from_xyz(planet.radius, 0.0, 0.0),
            planet,
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
