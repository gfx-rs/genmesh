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
use genmesh::generators::Cube;

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


    let mesh  = Rc::new(RefCell::new(Mesh::new(cube_v, cube_i, None, None, false)));
    let mut c = window.add_mesh(mesh, na::one());

    c.set_color(1.0, 0.0, 0.0);
    c.enable_backface_culling(false);
    c.set_local_scale(0.1, 0.1, 0.1);
    c.set_local_translation(Vec3::new(0f32, 0., 0.));

    window.set_light(light::StickToCamera);

    while window.render() {
        c.prepend_to_local_rotation(&Vec3::new(0.0f32, 0.014, 0.01));
    }
}
