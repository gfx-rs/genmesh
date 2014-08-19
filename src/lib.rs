
extern crate core;
extern crate debug;

use std::collections::RingBuf;
use std::collections::Deque;

pub use poly::{
    Vector1,
    Vector2,
    Vector3,
    Vector4,
    Quad,
    Triangle,
    Polygon,
    PolyTri,
    PolyQuad,
    ToTriangles
};

mod cube;
mod poly;
//mod quad;
//mod triangle;

pub mod generators {
    pub use cube::Cube;
}

trait TrainglePipeline<'a, T> : Iterator<Triangle<T>> {
    fn vertex<'a,U>(self, f: |T|:'a -> U) -> TriangleVertexMap<'a, Self, T, U> {
        TriangleVertexMap {
            source: self,
            f: f
        }
    }

    fn polygon<'a, U>(self, f: |Triangle<T>|:'a -> Triangle<U>) -> TrianglePolyMap<'a, Self, T, U> {
        TrianglePolyMap {
            source: self,
            f: f
        }
    }
}

pub struct TriangleVertexMap<'a, SRC, T, U> {
    source: SRC,
    f: |T|:'a -> U
}

impl<'a, SRC: TrainglePipeline<'a, T>, T: Clone, U> Iterator<Triangle<U>> for TriangleVertexMap<'a, SRC, T, U> {
    fn next(&mut self) -> Option<Triangle<U>> {
        self.source.next().map(|quad| {
            quad.map_vertex(|v| (self.f)(v))
        })
    }
}

impl<'a, SRC: TrainglePipeline<'a, T>, T: Clone, U> TrainglePipeline<'a, U> for TriangleVertexMap<'a, SRC, T, U> {}

pub struct TrianglePolyMap<'a, SRC, T, U> {
    source: SRC,
    f: |Triangle<T>|:'a -> Triangle<U>
}

impl<'a, SRC: TrainglePipeline<'a, T>, T, U> Iterator<Triangle<U>> for TrianglePolyMap<'a, SRC, T, U> {
    fn next(&mut self) -> Option<Triangle<U>> {
        self.source.next().map(|q| (self.f)(q))
    }
}

impl<'a, SRC: TrainglePipeline<'a, T>, T, U> TrainglePipeline<'a, U> for TrianglePolyMap<'a, SRC, T, U> {}

pub struct TriangleGenerator<SRC> {
    source: SRC
}

impl<T, SRC: Iterator<Triangle<T>>> TriangleGenerator<SRC> {
    pub fn new(src: SRC) -> TriangleGenerator<SRC> {
        TriangleGenerator {
            source: src
        }
    }
}

impl<'a, T, SRC: Iterator<Triangle<T>>> Iterator<Triangle<T>> for TriangleGenerator<SRC> {
    fn next(&mut self) -> Option<Triangle<T>> {
        self.source.next()
    }
} 

impl<'a, T, SRC: Iterator<Triangle<T>>> TrainglePipeline<'a, T> for TriangleGenerator<SRC> {}

trait QuadPipeline<'a, T> : Iterator<Quad<T>> {
    fn vertex<'a,U>(self, f: |T|:'a -> U) -> QuadVertexMap<'a, Self, T, U> {
        QuadVertexMap {
            source: self,
            f: f
        }
    }

    fn polygon<'a, U>(self, f: |Quad<T>|:'a -> Quad<U>) -> QuadPolyMap<'a, Self, T, U> {
        QuadPolyMap {
            source: self,
            f: f
        }
    }
}

pub struct QuadVertexMap<'a, SRC, T, U> {
    source: SRC,
    f: |T|:'a -> U
}

impl<'a, SRC: QuadPipeline<'a, T>, T: Clone, U> Iterator<Quad<U>> for QuadVertexMap<'a, SRC, T, U> {
    fn next(&mut self) -> Option<Quad<U>> {
        self.source.next().map(|quad| {
            quad.map_vertex(|v| (self.f)(v))
        })
    }
}

impl<'a, SRC: QuadPipeline<'a, T>, T: Clone, U> QuadPipeline<'a, U> for QuadVertexMap<'a, SRC, T, U> {}

pub struct QuadPolyMap<'a, SRC, T, U> {
    source: SRC,
    f: |Quad<T>|:'a -> Quad<U>
}

impl<'a, SRC: QuadPipeline<'a, T>, T, U> Iterator<Quad<U>> for QuadPolyMap<'a, SRC, T, U> {
    fn next(&mut self) -> Option<Quad<U>> {
        self.source.next().map(|q| (self.f)(q))
    }
}

impl<'a, SRC: QuadPipeline<'a, T>, T, U> QuadPipeline<'a, U> for QuadPolyMap<'a, SRC, T, U> {}


pub struct QuadGenerator<SRC> {
    source: SRC
}

impl<T, SRC: Iterator<Quad<T>>> QuadGenerator<SRC> {
    pub fn new(src: SRC) -> QuadGenerator<SRC> {
        QuadGenerator {
            source: src
        }
    }
}

impl<'a, T, SRC: Iterator<Quad<T>>> Iterator<Quad<T>> for QuadGenerator<SRC> {
    fn next(&mut self) -> Option<Quad<T>> {
        self.source.next()
    }
} 

impl<'a, T, SRC: Iterator<Quad<T>>> QuadPipeline<'a, T> for QuadGenerator<SRC> {}

#[cfg(test)]
mod test {
    use {
        QuadGenerator,
        QuadPipeline,
        Quad,
        ToTriangles,
        Triangle,
        TriangleGenerator,
        TrainglePipeline
    };

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
}