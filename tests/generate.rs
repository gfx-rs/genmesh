extern crate cgmath;
extern crate genmesh;

use std::fmt::Debug;
use genmesh::{generators, EmitTriangles, MapVertex, Triangulate};

/// Test a generator by comparing two triangular meshes:
/// 1) by using the `Iterator` implementation of the given generator
/// 2) by producing shared vertices and sampling them with the
///    produced indexed polygons.
fn test<F, P, G>(generator: G)
where
    F: EmitTriangles,
    F::Vertex: Clone + Copy + Debug + PartialEq,
    P: EmitTriangles<Vertex = usize>,
    G: generators::SharedVertex<F::Vertex> + generators::IndexedPolygon<P> + Iterator<Item = F>,
{
    let vertices: Vec<_> = generator.shared_vertex_iter().collect();

    let f1: Vec<_> = generator
        .indexed_polygon_iter()
        .triangulate()
        .map(|f| f.map_vertex(|u| vertices[u]))
        .collect();
    let f0: Vec<_> = generator.triangulate().collect();

    assert_eq!(f0.len(), f1.len());
    for (i, (p0, p1)) in f0.iter().zip(f1.iter()).enumerate() {
        assert_eq!(p0, p1, "Mismatched polygon[{}]", i);
    }
}

#[test]
fn gen_plane() {
    test(generators::Plane::new());
    test(generators::Plane::subdivide(3, 4));
}

#[test]
fn gen_cube() {
    test(generators::Cube::new());
}

#[test]
fn gen_cylinder() {
    test(generators::Cylinder::new(5));
    test(generators::Cylinder::subdivide(3, 4));
}

#[test]
fn gen_sphere_uv() {
    test(generators::SphereUv::new(4, 3));
}

#[test]
fn gen_ico_sphere() {
    test(generators::IcoSphere::new());
    test(generators::IcoSphere::subdivide(3));
}

#[test]
fn gen_cone() {
    test(generators::Cone::new(8));
}

#[test]
fn gen_torus() {
    test(generators::Torus::new(1., 0.5, 8, 8));
}
