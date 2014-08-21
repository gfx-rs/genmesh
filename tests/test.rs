extern crate vertex;

use vertex::{
    QuadGenerator,
    QuadPipeline,
    Quad,
    ToTriangles,
    Triangle,
    TriangleGenerator,
    TrianglePipeline,
    Vector2
};

use vertex::generators::Plane;

#[test]
fn test_quad_vertex() {
    let input = &[Quad::new(0u, 1, 2, 3),
                  Quad::new(1u, 2, 3, 4)];

    let output = &[Quad::new(false, true, false, true),
                   Quad::new(true, false, true, false)];

    let transformed = QuadGenerator::new(input.iter().map(|x| x.clone()))
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

    let transformed = QuadGenerator::new(input.iter().map(|x| x.clone()))
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

    let transformed = QuadGenerator::new(input.iter().map(|x| x.clone()))
        .polygon(|v| Quad::new(0i, v.y as int, v.z as int, 0));

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

    let transformed = TriangleGenerator::new(input.iter().map(|x| x.clone()))
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

    let transformed = TriangleGenerator::new(input.iter().map(|x| x.clone()))
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

    let transformed = TriangleGenerator::new(input.iter().map(|x| x.clone()))
        .polygon(|v| Triangle::new(0i, v.y as int, v.z as int));

    for (x, y) in transformed.zip(output.iter().map(|x| x.clone())) {
        assert_eq!(x, y);
    }
}

#[test]
fn test_to_triangles() {
    let q = Quad::new(0u, 1, 2, 3);
    let mut result = Vec::new();
    q.to_triangles(|v| result.push(v));

    assert_eq!(result, vec![Triangle::new(0u, 1, 2),
                            Triangle::new(2u, 3, 0)]);

    let t = Triangle::new(0u, 1, 2);
    let mut result = Vec::new();
    t.to_triangles(|v| result.push(v));

    assert_eq!(result, vec![Triangle::new(0u, 1, 2)]);
}

#[test]
fn test_plane() {
    let mut plane = Plane::new();

    let a = plane.next().unwrap();

    let Quad {
        x: Vector2([ax, ay]),
        y: Vector2([bx, by]),
        z: Vector2([cx, cy]),
        w: Vector2([dx, dy])
    } = a;

    assert_eq!(ax, -1.); assert_eq!(ay, -1.);
    assert_eq!(bx, -1.); assert_eq!(by,  1.);
    assert_eq!(cx,  1.); assert_eq!(cy,  1.);
    assert_eq!(dx,  1.); assert_eq!(dy, -1.);
}