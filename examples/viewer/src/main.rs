use bevy::prelude::*;
use bevy::render::{mesh::Indices, render_resource::PrimitiveTopology};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use genmesh::{
    generators::{
        self, Circle, Cone, Cube, Cylinder, IcoSphere, IndexedPolygon, Plane, SharedVertex,
        SphereUv, Torus,
    },
    Triangulate, Vertices,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_plugin(PanOrbitCameraPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {
    // Add a camera so we can see the generators.
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 3.0, 25.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(PanOrbitCamera {
            radius: 25.,
            ..default()
        });

    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_xyz(-3., 3., 10.),
    //     point_light: PointLight {
    //         intensity: 10000.,
    //         // shadows_enabled: true,
    //         ..default()
    //     },
    //     ..default()
    // });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(-10., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            illuminance: 10000.,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // Add a ground plane.
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        transform: Transform::from_xyz(0.0, -1.0, 0.0),
        ..default()
    });

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Cube { size: 1.0 }.into()),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0., 0., 0.),
    //     ..default()
    // });
        None
        None
        Some(shape::Cube::default().into()),
        // Some(shape::Box::default().into()),
        Some(shape::Capsule::default().into()),
        Some(shape::Cylinder::default().into()),
        Some(shape::Icosphere::default().try_into().unwrap()),
        None
        Some(shape::UVSphere::default().into()),
        Some(shape::Torus::default().into()),

    let generators: Vec<Mesh> = vec![
        Circle::new(10).into(),
        Cone::new(10).into(),
        Cube::new().into(),
        Cylinder::new(10).into(),
        IcoSphere::new().into(),
        Plane::new().into(),
        SphereUv::new(10, 10).into(),
        Torus::new(1., 1., 10, 10).into(),
    ];
    let count = generators.len();

    // Make a bevy mesh by hand.
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let generator = generators::Cube::new();
    let vertices: Vec<_> = generator
        .shared_vertex_iter()
        .map(|v| Vec3::new(v.pos.x / 2., v.pos.y / 2., v.pos.z / 2.))
        .collect();
    let normals: Vec<_> = generator
        .shared_vertex_iter()
        .map(|v| Vec3::new(v.normal.x, v.normal.y, v.normal.z))
        .collect();
    let indices: Vec<u32> = generator
        .indexed_polygon_iter()
        .triangulate()
        .vertices()
        .map(|u| u as u32)
        .collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(Indices::U32(indices)));

    for (i, mesh) in generators.into_iter().enumerate() {
        commands.spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(1., 1., 1.).into()),
            // transform: Transform::from_xyz(3. * (i - count / 2) as f32, 0., 0.),
            transform: Transform::from_xyz(3. * (i as f32 - count as f32 / 2.), 0., 0.),
            ..default()
        });
    }

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(mesh),
    //     material: materials.add(Color::rgb(0.8, 0.3, 0.3).into()),
    //     transform: Transform::from_xyz(1.5, 0., 0.),
    //     ..default()
    // });

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Cube::new().into()),
    //     material: materials.add(Color::rgb(0.3, 0.8, 0.3).into()),
    //     transform: Transform::from_xyz(3.5, 0., 0.),
    //     ..default()
    // });

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(generators::SphereUv::new(10, 10).into()),
    //     material: materials.add(Color::rgb(0.3, 0.3, 0.8).into()),
    //     transform: Transform::from_xyz(5.5, 0., 0.),
    //     ..default()
    // });
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vertices_as_indices() {
        let generator = generators::Cube::new();
        let first = generator
            .indexed_polygon_iter()
            .triangulate()
            .vertices()
            .map(|i| i as u32)
            .next();
        assert_eq!(Some(0u32), first);
    }

    #[test]
    fn vertices_as_vectors() {
        let generator = generators::Cube::new();
        let first = generator
            .triangulate()
            .vertices()
            .map(|v| v.pos.into())
            .next();
        assert_eq!(Some(Vec3::new(1., 1., -1.)), first);
    }
}
