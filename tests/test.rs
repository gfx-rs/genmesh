extern crate genmesh;

use genmesh::{
    EmitTriangles, Indexer, LruIndexer, MapToVertices, Quad, Triangle, Triangulate, Vertex,
    Vertices,
};

use genmesh::generators::Plane;

#[test]
fn quad_vertex() {
    let input = &[Quad::new(0usize, 1, 2, 3), Quad::new(1usize, 2, 3, 4)];

    let output = &[
        Quad::new(false, true, false, true),
        Quad::new(true, false, true, false),
    ];

    let transformed = input.iter().cloned().vertex(|v| v % 2 != 0);

    for (x, y) in transformed.zip(output.iter().cloned()) {
        assert_eq!(x, y);
    }
}

#[test]
fn quad_vertex_two_stages() {
    let input = &[Quad::new(0usize, 1, 2, 3), Quad::new(1usize, 2, 3, 4)];

    let output = &[
        Quad::new(false, true, false, true),
        Quad::new(true, false, true, false),
    ];

    let transformed = input
        .iter()
        .cloned()
        .vertex(|v| v as u8)
        .vertex(|v| v % 2 != 0);

    for (x, y) in transformed.zip(output.iter().cloned()) {
        assert_eq!(x, y);
    }
}

#[test]
fn quad_poly_simple() {
    let input = &[Quad::new(0usize, 1, 2, 3), Quad::new(1usize, 2, 3, 4)];

    let output = &[Quad::new(0isize, 1, 2, 0), Quad::new(0isize, 2, 3, 0)];

    let transformed = input
        .iter()
        .cloned()
        .map(|v| Quad::new(0isize, v.y as isize, v.z as isize, 0));

    for (x, y) in transformed.zip(output.iter().cloned()) {
        assert_eq!(x, y);
    }
}

#[test]
fn triangle_vertex() {
    let input = &[Triangle::new(0usize, 1, 2), Triangle::new(1usize, 2, 3)];

    let output = &[
        Triangle::new(false, true, false),
        Triangle::new(true, false, true),
    ];

    let transformed = input.iter().cloned().vertex(|v| v % 2 != 0);

    for (x, y) in transformed.zip(output.iter().cloned()) {
        assert_eq!(x, y);
    }
}

#[test]
fn triangle_vertex_two_stages() {
    let input = &[Triangle::new(0usize, 1, 2), Triangle::new(1usize, 2, 3)];

    let output = &[
        Triangle::new(false, true, false),
        Triangle::new(true, false, true),
    ];

    let transformed = input
        .iter()
        .cloned()
        .vertex(|v| v as u8)
        .vertex(|v| v % 2 != 0);

    for (x, y) in transformed.zip(output.iter().cloned()) {
        assert_eq!(x, y);
    }
}

#[test]
fn triangle_poly_simple() {
    let input = &[Triangle::new(0usize, 1, 2), Triangle::new(1usize, 2, 3)];

    let output = &[Triangle::new(0isize, 1, 2), Triangle::new(0isize, 2, 3)];

    let transformed = input
        .iter()
        .cloned()
        .map(|v| Triangle::new(0isize, v.y as isize, v.z as isize));

    for (x, y) in transformed.zip(output.iter().cloned()) {
        assert_eq!(x, y);
    }
}

#[test]
fn to_triangles() {
    let q = Quad::new(0usize, 1, 2, 3);
    let mut result = Vec::new();
    q.emit_triangles(|v| result.push(v));

    assert_eq!(
        result,
        vec![Triangle::new(0usize, 1, 2), Triangle::new(2usize, 3, 0)]
    );

    let t = Triangle::new(0usize, 1, 2);
    let mut result = Vec::new();
    t.emit_triangles(|v| result.push(v));

    assert_eq!(result, vec![Triangle::new(0usize, 1, 2)]);
}

#[test]
fn plane() {
    let mut plane = Plane::new();
    let a = plane.next().unwrap();

    assert_eq!(a.x.pos, [-1f32, -1., 0.].into());
    assert_eq!(a.y.pos, [1f32, -1., 0.].into());
    assert_eq!(a.z.pos, [1f32, 1., 0.].into());
    assert_eq!(a.w.pos, [-1f32, 1., 0.].into());
}

//TODO: LRU tests changed once the normals got introduced to the `Cube`.
// these tests may need to be revised now.
#[test]
fn lru_indexer() {
    let mut vectices: Vec<Vertex> = Vec::new();
    let indexes: Vec<usize> = {
        let mut indexer = LruIndexer::new(8, |_, v| vectices.push(v));

        Plane::subdivide(1, 3)
            .vertex(|v| indexer.index(v))
            .vertices()
            .collect()
    };

    assert_eq!(8, vectices.len());
    assert_eq!(3 * 4, indexes.len());

    let mut vectices: Vec<Vertex> = Vec::new();
    let indexes: Vec<usize> = {
        let mut indexer = LruIndexer::new(4, |_, v| vectices.push(v));

        Plane::subdivide(1, 3)
            .triangulate()
            .vertex(|v| indexer.index(v))
            .vertices()
            .collect()
    };

    assert_eq!(8, vectices.len());
    assert_eq!(3 * 3 * 2, indexes.len());
}

#[test]
fn emit_lines() {
    use genmesh::{EmitLines, Line, Lines};

    let mut lines = Vec::new();
    let triangle = Triangle::new(0i8, 1, 2);
    triangle.emit_lines(|x| lines.push(x));

    assert_eq!(3, lines.len());
    assert_eq!(Line::new(0, 1), lines[0]);
    assert_eq!(Line::new(1, 2), lines[1]);
    assert_eq!(Line::new(2, 0), lines[2]);

    let mut lines = Vec::new();
    let quad = Quad::new(0i8, 1, 2, 3);
    quad.emit_lines(|x| lines.push(x));

    assert_eq!(4, lines.len());
    assert_eq!(Line::new(0, 1), lines[0]);
    assert_eq!(Line::new(1, 2), lines[1]);
    assert_eq!(Line::new(2, 3), lines[2]);
    assert_eq!(Line::new(3, 0), lines[3]);

    let quads = [Quad::new(0i8, 1, 2, 3), Quad::new(4i8, 5, 6, 7)];
    let lines: Vec<Line<i8>> = quads.iter().copied().lines().collect();

    assert_eq!(8, lines.len());
    assert_eq!(Line::new(0, 1), lines[0]);
    assert_eq!(Line::new(1, 2), lines[1]);
    assert_eq!(Line::new(2, 3), lines[2]);
    assert_eq!(Line::new(3, 0), lines[3]);
}
