//   Copyright Colin Sherratt 2014
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

extern crate genmesh;

use genmesh::{
    Quad,
    EmitTriangles,
    Triangle,
    MapToVertices,
    LruIndexer,
    Indexer,
    Vertices,
    Triangulate
};

use genmesh::generators::{Cube, Plane};

#[test]
fn test_quad_vertex() {
    let input = &[Quad::new(0u, 1, 2, 3),
                  Quad::new(1u, 2, 3, 4)];

    let output = &[Quad::new(false, true, false, true),
                   Quad::new(true, false, true, false)];

    let transformed = input.iter().map(|x| x.clone())
                                  .vertex(|v| v % 2 != 0);

    for (x, y) in transformed.zip(output.iter().map(|x| x.clone())) {
        assert_eq!(x, y);
    }
}

#[test]
fn test_quad_vertex_two_stages() {
    let input = &[Quad::new(0u, 1, 2, 3),
                  Quad::new(1u, 2, 3, 4)];

    let output = &[Quad::new(false, true, false, true),
                   Quad::new(true, false, true, false)];

    let transformed = input.iter().map(|x| x.clone())
                                  .vertex(|v| v as u8)
                                  .vertex(|v| v % 2 != 0);

    for (x, y) in transformed.zip(output.iter().map(|x| x.clone())) {
        assert_eq!(x, y);
    }
}

#[test]
fn test_quad_poly_simple() {
    let input = &[Quad::new(0u, 1, 2, 3),
                  Quad::new(1u, 2, 3, 4)];

    let output = &[Quad::new(0i, 1, 2, 0),
                   Quad::new(0i, 2, 3, 0)];

    let transformed = input.iter().map(|x| x.clone())
                                  .map(|v| Quad::new(0i, v.y as int, v.z as int, 0));

    for (x, y) in transformed.zip(output.iter().map(|x| x.clone())) {
        assert_eq!(x, y);
    }
}

#[test]
fn test_triangle_vertex() {
    let input = &[Triangle::new(0u, 1, 2),
                  Triangle::new(1u, 2, 3)];

    let output = &[Triangle::new(false, true, false),
                   Triangle::new(true, false, true)];

    let transformed = input.iter().map(|x| x.clone())
                           .vertex(|v| v % 2 != 0);

    for (x, y) in transformed.zip(output.iter().map(|x| x.clone())) {
        assert_eq!(x, y);
    }
}

#[test]
fn test_triangle_vertex_two_stages() {
    let input = &[Triangle::new(0u, 1, 2),
                  Triangle::new(1u, 2, 3)];

    let output = &[Triangle::new(false, true, false),
                   Triangle::new(true, false, true)];

    let transformed = input.iter().map(|x| x.clone())
                           .vertex(|v| v as u8)
                           .vertex(|v| v % 2 != 0);

    for (x, y) in transformed.zip(output.iter().map(|x| x.clone())) {
        assert_eq!(x, y);
    }
}

#[test]
fn test_triangle_poly_simple() {
    let input = &[Triangle::new(0u, 1, 2),
                  Triangle::new(1u, 2, 3)];

    let output = &[Triangle::new(0i, 1, 2),
                   Triangle::new(0i, 2, 3)];

    let transformed = input.iter().map(|x| x.clone())
                           .map(|v| Triangle::new(0i, v.y as int, v.z as int));

    for (x, y) in transformed.zip(output.iter().map(|x| x.clone())) {
        assert_eq!(x, y);
    }
}

#[test]
fn test_to_triangles() {
    let q = Quad::new(0u, 1, 2, 3);
    let mut result = Vec::new();
    q.emit_triangles(|v| result.push(v));

    assert_eq!(result, vec![Triangle::new(0u, 1, 2),
                            Triangle::new(2u, 3, 0)]);

    let t = Triangle::new(0u, 1, 2);
    let mut result = Vec::new();
    t.emit_triangles(|v| result.push(v));

    assert_eq!(result, vec![Triangle::new(0u, 1, 2)]);
}

#[test]
fn test_plane() {
    let mut plane = Plane::new();

    let a = plane.next().unwrap();

    let Quad {
        x: (ax, ay),
        y: (bx, by),
        z: (cx, cy),
        w: (dx, dy)
    } = a;

    assert_eq!(ax, -1.); assert_eq!(ay, -1.);
    assert_eq!(bx, -1.); assert_eq!(by,  1.);
    assert_eq!(cx,  1.); assert_eq!(cy,  1.);
    assert_eq!(dx,  1.); assert_eq!(dy, -1.);
}

#[test]
fn test_lru_indexer() {
    let mut vectices = Vec::new();
    let indexes: Vec<uint> = {
        let mut indexer = LruIndexer::new(8, |_, v| vectices.push(v));

        Cube::new().vertex(|v| indexer.index(v))
                   .vertices()
                   .collect()
    };

    assert_eq!(8, vectices.len());
    assert_eq!(6*4, indexes.len());

    let mut vectices = Vec::new();
    let indexes: Vec<uint> = {
        let mut indexer = LruIndexer::new(4, |_, v| vectices.push(v));

        Cube::new().triangulate()
                   .vertex(|v| indexer.index(v))
                   .vertices()
                   .collect()
    };

    assert_eq!(20, vectices.len());
    assert_eq!(3*6*2, indexes.len());
}