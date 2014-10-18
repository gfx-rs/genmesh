extern crate native;
extern crate kiss3d;
extern crate "nalgebra" as na;
extern crate genmesh;

use std::rc::Rc;
use std::cell::RefCell;
use na::{Pnt3, Vec3};
use kiss3d::window::Window;
use kiss3d::resource::Mesh;
use kiss3d::light;

use genmesh::{MapToVertices, Triangulate};
use genmesh::{LruIndexer, Indexer};
use genmesh::generators::{Plane, Cube, SphereUV};
use genmesh::generators::{SharedVertex, IndexedPolygon};

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    let mut window = Window::new("Kiss3d: custom_mesh");

    let mut cube_v = Vec::new();
    let cube_i: Vec<Vec3<u32>> = {
        let mut indexer = LruIndexer::new(8, |_, v| cube_v.push(v));
        Cube::new()
            .vertex(|(a, b, c)| Pnt3::new(a, b, c))
            .vertex(|v| indexer.index(v) as u32)
            .triangulate()
            .map(|p| Vec3::new(p.x, p.y, p.z) )
            .collect()
    };
    let cube_mesh = Rc::new(RefCell::new(Mesh::new(cube_v, cube_i, None, None, false)));
    let mut cube0 = window.add_mesh(cube_mesh, na::one());

    cube0.set_color(1.0, 0.0, 0.0);
    cube0.set_local_scale(0.1, 0.1, 0.1);
    cube0.prepend_to_local_rotation(&Vec3::new(-0.5f32, 0., 0.));

    let cube_v: Vec<Pnt3<f32>> = Cube::new()
        .shared_vertex_iter()
        .map(|(x, y, z)| Pnt3::new(x, y, z))
        .collect();

    let cube_i: Vec<Vec3<u32>> = Cube::new()
        .indexed_polygon_iter()
        .triangulate()
        .map(|v| Vec3::new(v.x as u32, v.y as u32, v.z as u32))
        .collect();
    let cube_mesh = Rc::new(RefCell::new(Mesh::new(cube_v, cube_i, None, None, false)));
    let mut cube1 = window.add_mesh(cube_mesh, na::one());

    cube1.set_color(1.0, 0.0, 0.0);
    cube1.set_local_scale(0.1, 0.1, 0.1);
    cube1.set_local_translation(Vec3::new(0.0f32, 0.25, 0.0));
    cube1.prepend_to_local_rotation(&Vec3::new(-0.5f32, 0., 0.));

    let mut plane_v = Vec::new();
    let plane_i: Vec<Vec3<u32>> = {
        let mut indexer = LruIndexer::new(8, |_, v| plane_v.push(v));
        Plane::subdivide(8, 8)
            .triangulate()
            .vertex(|(a, b)| Pnt3::new(a, b, 0.) )
            .vertex(|v| indexer.index(v) as u32)
            .map(|p| Vec3::new(p.x, p.y, p.z) )
            .collect()
    };
    let plane_mesh = Rc::new(RefCell::new(Mesh::new(plane_v, plane_i, None, None, false)));
    let mut plane0 = window.add_mesh(plane_mesh, na::one());

    plane0.set_color(0.0, 1.0, 0.0);
    plane0.set_local_scale(0.1, 0.1, 0.1);
    plane0.set_local_translation(Vec3::new(0.25f32, 0.0, 0.0));
    plane0.prepend_to_local_rotation(&Vec3::new(-0.5f32, 0., 0.));

    let plane_v: Vec<Pnt3<f32>> = Plane::subdivide(2, 2)
        .shared_vertex_iter()
        .map(|(x, y)| Pnt3::new(x, y, 0.))
        .collect();

    let plane_i: Vec<Vec3<u32>> = Plane::subdivide(2, 2)
        .indexed_polygon_iter()
        .triangulate()
        .map(|v| Vec3::new(v.x as u32, v.y as u32, v.z as u32))
        .collect();

    let plane_mesh = Rc::new(RefCell::new(Mesh::new(plane_v, plane_i, None, None, false)));
    let mut plane1 = window.add_mesh(plane_mesh, na::one());

    plane1.set_color(0.0, 1.0, 0.0);
    plane1.set_local_scale(0.1, 0.1, 0.1);
    plane1.set_local_translation(Vec3::new(0.25f32, 0.25, 0.0));
    plane1.prepend_to_local_rotation(&Vec3::new(-0.5f32, 0., 0.));

    let mut sphere_v = Vec::new();
    let sphere_i: Vec<Vec3<u32>> = {
        let mut indexer = LruIndexer::new(8, |_, v| sphere_v.push(v));
        SphereUV::new(32, 16)
            .triangulate()
            .vertex(|(a, b, c)| Pnt3::new(a, b, c) )
            .vertex(|v| indexer.index(v) as u32)
            .map(|p| Vec3::new(p.x, p.y, p.z) )
            .collect()
    };
    let sphere_mesh = Rc::new(RefCell::new(Mesh::new(sphere_v, sphere_i, None, None, false)));
    let mut sphere0 = window.add_mesh(sphere_mesh, na::one());

    sphere0.set_color(0.0, 0.0, 1.0);
    sphere0.set_local_scale(0.1, 0.1, 0.1);
    sphere0.set_local_translation(Vec3::new(-0.25f32, 0.0, 0.0));
    sphere0.prepend_to_local_rotation(&Vec3::new(-0.5f32, 0., 0.));

    let sphere_v: Vec<Pnt3<f32>> = SphereUV::new(32, 16)
        .shared_vertex_iter()
        .map(|(x, y, z)| Pnt3::new(x, y, z))
        .collect();

    let sphere_i: Vec<Vec3<u32>> = SphereUV::new(32, 16)
        .indexed_polygon_iter()
        .triangulate()
        .map(|v| Vec3::new(v.x as u32, v.y as u32, v.z as u32))
        .collect();
    let sphere_mesh = Rc::new(RefCell::new(Mesh::new(sphere_v, sphere_i, None, None, false)));
    let mut sphere1 = window.add_mesh(sphere_mesh, na::one());

    sphere1.set_color(0.0, 0.0, 1.0);
    sphere1.set_local_scale(0.1, 0.1, 0.1);
    sphere1.set_local_translation(Vec3::new(-0.25f32, 0.25, 0.0));
    sphere1.prepend_to_local_rotation(&Vec3::new(-0.5f32, 0., 0.));

    window.set_light(light::StickToCamera);

    while window.render() {
        cube0.prepend_to_local_rotation(&Vec3::new(0.0f32, 0.014, 0.0));
        cube1.prepend_to_local_rotation(&Vec3::new(0.0f32, 0.014, 0.0));
        plane0.prepend_to_local_rotation(&Vec3::new(0.0f32, 0.014, 0.0));
        plane1.prepend_to_local_rotation(&Vec3::new(0.0f32, 0.014, 0.0));
        sphere0.prepend_to_local_rotation(&Vec3::new(0.0f32, 0.014, 0.0));
        sphere1.prepend_to_local_rotation(&Vec3::new(0.0f32, 0.014, 0.0))
    }
}
