extern crate native;
extern crate kiss3d;
extern crate nalgebra;
extern crate genmesh;

use std::rc::Rc;
use std::cell::RefCell;
use nalgebra::na::Vec3;
use nalgebra::na;
use kiss3d::window::Window;
use kiss3d::resource::Mesh;
use kiss3d::light;

use genmesh::{MapToVertices, Triangulate};
use genmesh::{LruIndexer, Indexer};
use genmesh::generators::{Plane, Cube, SphereUV};

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
            .vertex(|(a, b, c)| {
                Vec3::new(a, b, c)
            })
            .vertex(|v| indexer.index(v) as u32)
            .triangulate()
            .map(|p| Vec3::new(p.x, p.y, p.z) )
            .collect()
    };
    let cube_mesh = Rc::new(RefCell::new(Mesh::new(cube_v, cube_i, None, None, false)));
    let mut cube = window.add_mesh(cube_mesh, na::one());

    cube.set_color(1.0, 0.0, 0.0);
    cube.set_local_scale(0.1, 0.1, 0.1);
    cube.prepend_to_local_rotation(&Vec3::new(-0.5f32, 0., 0.));

    let mut plane_v = Vec::new();
    let plane_i: Vec<Vec3<u32>> = {
        let mut indexer = LruIndexer::new(8, |_, v| plane_v.push(v));
        Plane::subdivide(8, 8)
            .triangulate()
            .vertex(|(a, b)| {
                Vec3::new(a, b, 0.)
            })
            .vertex(|v| indexer.index(v) as u32)
            .map(|p| Vec3::new(p.x, p.y, p.z) )
            .collect()
    };
    let plane_mesh = Rc::new(RefCell::new(Mesh::new(plane_v, plane_i, None, None, false)));
    let mut plane = window.add_mesh(plane_mesh, na::one());

    plane.set_color(0.0, 1.0, 0.0);
    plane.set_local_scale(0.1, 0.1, 0.1);
    plane.set_local_translation(Vec3::new(0.25f32, 0.0, 0.0));
    plane.prepend_to_local_rotation(&Vec3::new(-0.5f32, 0., 0.));

    let mut sphere_v = Vec::new();
    let sphere_i: Vec<Vec3<u32>> = {
        let mut indexer = LruIndexer::new(8, |_, v| sphere_v.push(v));
        SphereUV::new(8, 8)
            .triangulate()
            .vertex(|(a, b, c)| {
                Vec3::new(a, b, c)
            })
            .vertex(|v| indexer.index(v) as u32)
            .map(|p| Vec3::new(p.x, p.y, p.z) )
            .collect()
    };
    let sphere_mesh = Rc::new(RefCell::new(Mesh::new(sphere_v, sphere_i, None, None, false)));
    let mut sphere = window.add_mesh(sphere_mesh, na::one());

    sphere.set_color(0.0, 0.0, 1.0);
    sphere.set_local_scale(0.1, 0.1, 0.1);
    sphere.set_local_translation(Vec3::new(-0.25f32, 0.0, 0.0));
    sphere.prepend_to_local_rotation(&Vec3::new(-0.5f32, 0., 0.));

    window.set_light(light::StickToCamera);

    while window.render() {
        cube.prepend_to_local_rotation(&Vec3::new(0.0f32, 0.014, 0.0));
        plane.prepend_to_local_rotation(&Vec3::new(0.0f32, 0.014, 0.0));
        sphere.prepend_to_local_rotation(&Vec3::new(0.0f32, 0.014, 0.0))
    }
}
