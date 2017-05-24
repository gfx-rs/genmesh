//   Copyright Dzmitry Malyshau 2017
//   
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//   
//       http://www.apache.org/licenses/LICENSE-2.0
//   
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

extern crate cgmath;
extern crate genmesh;

use std::fmt::Debug;
use genmesh::{
    generators,
    Triangle,
    EmitTriangles,
    Triangulate,
};

fn test<F, P, G>(generator: G) where
    F: EmitTriangles,
    F::Vertex: Clone + Copy + Debug + PartialEq,
    P: EmitTriangles<Vertex = usize>,
    G: generators::SharedVertex<F::Vertex> +
       generators::IndexedPolygon<P> +
       Iterator<Item = F>,
{
    let vertices: Vec<_> = generator.shared_vertex_iter()
                                    .collect();
    let fun = |f: Triangle<usize>| {
        Triangle::new(vertices[f.x],
                      vertices[f.y],
                      vertices[f.z])
    };
    let f1: Vec<_> = generator.indexed_polygon_iter()
                              .triangulate()
                              .map(fun)
                              .collect();
    let f0: Vec<_> = generator.triangulate()
                              .collect();

    assert_eq!(f0.len(), f1.len());
    for (i, (p0, p1)) in f0.iter().zip(f1.iter()).enumerate() {
        assert_eq!(p0, p1, "Mismatched polygon[{}]", i);
    }
}

#[test]
fn plane() {
    test(generators::Plane::new());
    test(generators::Plane::subdivide(3, 4));
}

#[test]
fn cube() {
    test(generators::Cube::new());
}

#[test]
fn cylinder() {
    test(generators::Cylinder::new(5));
}

#[test]
fn sphere_uv() {
    test(generators::SphereUV::new(4, 3));
}
