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

use cgmath::{
    InnerSpace,
};
use genmesh::{
    generators,
    Line,
    EmitLines,
    MapToVertices,
    Vertex,
};


struct Edge {
    dir: cgmath::Vector3<f32>,
    mid: cgmath::Vector3<f32>,
    nor: cgmath::Vector3<f32>,
}

impl Edge {
    fn new(line: Line<Vertex>) -> Self {
        let Line{ x: Vertex{ pos: x, normal: nx }, y: Vertex{ pos: y, normal: ny } } = line;
        Edge {
            dir: cgmath::vec3(y[0] - x[0], y[1] - x[1], y[2] - x[2]),
            mid: cgmath::vec3(y[0] + x[0], y[1] + x[1], y[2] + x[2]) * 0.5,
            nor: cgmath::vec3(nx[0] + ny[0], nx[1] + ny[1], nx[2] + ny[2]),
        }
    }

    /// Check that the corner `(self, e)` has outward winding order
    /// (thus, it's normal is in the same hemisphere as it's offset).
    fn check_to(&self, e: &Edge) {
        let normal = self.dir.cross(e.dir);
        let mid = (self.mid + e.mid) * 0.5;
        assert!(normal.dot(mid) > 0.0 && e.nor.dot(mid) > 0.0);
    }
}

/// Make sure that all the polygons in the `poly_iter` have the outward
/// winding order relative to the origin of the coordinate system.
/// This is a simplified (and incomplete) convex shape test.
fn test<P, I>(poly_iter: I) where
    P: EmitLines<Vertex=Vertex>,
    I: Iterator<Item=P>,
{
    let mut edges = Vec::new();
    for poly in poly_iter {
        edges.clear();
        poly.emit_lines(|l| edges.push(Edge::new(l)));
        // check last-first corner first, since it wraps
        edges.last().unwrap().check_to(&edges[0]);
        // check all the non-wrapping corners
        for (a, b) in edges.iter().zip(edges[1..].iter()) {
            a.check_to(b);
        }
    }
}


#[test]
fn wind_plane() {
    test(generators::Plane::new()
        .vertex(|mut v| {v.pos[2] = 1.; v}));
    test(generators::Plane::subdivide(3, 4)
        .vertex(|mut v| {v.pos[2] = 1.; v}));
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
    test(generators::SphereUV::new(4, 3));
}
