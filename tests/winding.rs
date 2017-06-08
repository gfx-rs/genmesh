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

use std::collections::HashSet;

use cgmath::InnerSpace;
use genmesh::{generators, Line, EmitLines, MapToVertices, Vertex, Lines};

#[derive(Debug)]
struct Edge {
    dir: cgmath::Vector3<f32>,
    mid: cgmath::Vector3<f32>,
    nor: cgmath::Vector3<f32>,
}

impl Edge {
    fn new(line: Line<Vertex>) -> Self {
        let Line {
            x: Vertex { pos: x, normal: nx },
            y: Vertex { pos: y, normal: ny },
        } = line;

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
fn test_outward<P, I>(poly_iter: I)
    where P: EmitLines<Vertex = Vertex> + ::std::fmt::Debug,
          I: Iterator<Item = P>
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

// this will check that a primitive is closed, and that every edge of
// every polygon making up the primitive has a paired neighbor. This
// does not test that a polygon has the correcting winding direction
// just that the winding directions are consistent for the polygon.
//
// This is based on ftp://ftp.sgi.com/opengl/contrib/blythe/advanced99/notes/node16.html
//
fn test_closed<P, I>(poly_iter: I)
    where P: EmitLines<Vertex = Vertex> + ::std::fmt::Debug,
          I: Iterator<Item = P>
{
    // convert the vertex to something that we can use to find approximate
    // polygons. This is mostly to get past the fact that f32 is a cursed
    // type in rust and can not be used as a key.
    fn to_checkable(vertex: Vertex) -> [i32; 3] {
        [(vertex.pos[0] * 1000000.) as i32,
         (vertex.pos[1] * 1000000.) as i32,
         (vertex.pos[2] * 1000000.) as i32]
    }

    let mut lines = HashSet::new();
    for line in poly_iter.lines().vertex(to_checkable) {
        // if the line was in the set we found the matching pair
        // which is one less pair that we are looking for
        if !lines.remove(&line) {
            // if didn't find the pair, we flip the line around and put it into
            // the search table.
            lines.insert(Line {
                             x: line.y,
                             y: line.x,
                         });
        }
    }

    // if we found all the pairs, we should be valid and a closed geometry
    // this means that there is no polygon who's neighbor switches winding
    // direction, but it does not mean that the polygon is correct. They
    // all could be backwards. So this still requires a secondary inspection.
    assert!(lines.len() == 0);
}


#[test]
fn wind_plane() {
    // the plane is not closed, so no point in testing for a closed
    // shape.

    test_outward(generators::Plane::new().vertex(|mut v| {
                                                     v.pos[2] = 1.;
                                                     v
                                                 }));
    test_outward(generators::Plane::subdivide(3, 4).vertex(|mut v| {
                                                               v.pos[2] = 1.;
                                                               v
                                                           }));
}

#[test]
fn gen_cube() {
    test_outward(generators::Cube::new());
    test_closed(generators::Cube::new());
}

#[test]
fn gen_cylinder() {
    test_outward(generators::Cylinder::new(5));
    test_closed(generators::Cylinder::new(5));
    test_outward(generators::Cylinder::subdivide(3, 4));
    test_closed(generators::Cylinder::subdivide(3, 4));
}

#[test]
fn gen_sphere_uv() {
    test_outward(generators::SphereUV::new(4, 3));
    test_closed(generators::SphereUV::new(4, 3));
}

#[test]
fn gen_cone() {
    test_outward(generators::Cone::new(8));
    test_closed(generators::Cone::new(8));
}

#[test]
fn gen_torus() {
    // we don't do an outward test because the primitive is not
    // convex and will fail this test.
    test_closed(generators::Torus::new(10.0, 5.0, 8, 8));
}
