
extern crate core;

use std::iter::FromIterator;
use core::slice::Items;

pub mod generators;

trait Poly<T> : FromIterator<T> {
    fn as_slice<'a>(&'a self) -> &'a [T];

    fn iter<'a>(&'a self) -> Items<T> {
        self.as_slice().iter()
    }
}

pub struct Trinagle<T>([T, ..3]);

impl<T> Trinagle<T> {
    pub fn new(v0: T, v1: T, v2: T) -> Trinagle<T> {
        Trinagle([v0, v1, v2])
    }
}

impl<T> FromIterator<T> for Trinagle<T> {
    fn from_iter<I: Iterator<T>>(iter: I) -> Trinagle<T> {
        let mut iter = iter;
        match (iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), Some(c)) => Trinagle([a, b, c]),
            _ => fail!("should have found 3 vertices to buld a triangle")
        }
    }
}

impl<T> Poly<T> for Trinagle<T> {
    fn as_slice<'a>(&'a self) -> &'a [T] {
        let &Trinagle(ref s) = self;
        s.as_slice()
    }
}

pub struct Quad<T>([T, ..4]);

impl<T> Quad<T> {
    pub fn new(v0: T, v1: T, v2: T, v3: T) -> Quad<T> {
        Quad([v0, v1, v2, v3])
    }
}

impl<T: Clone> Quad<T> {
    pub fn to_triangles(&self) -> [Trinagle<T>, ..2] {
        let &Quad([ref v0, ref v1, ref v2, ref v3]) = self;
        [Trinagle::new(v0.clone(), v1.clone(), v2.clone()),
         Trinagle::new(v2.clone(), v3.clone(), v0.clone())]
    }
}

impl<T> FromIterator<T> for Quad<T> {
    fn from_iter<I: Iterator<T>>(iter: I) -> Quad<T> {
        let mut iter = iter;
        match (iter.next(), iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), Some(c), Some(d)) => Quad([a, b, c, d]),
            _ => fail!("should have found 4 vertices to buld a quad")
        }
    }
}

impl<T> Poly<T> for Quad<T> {
    fn as_slice<'a>(&'a self) -> &'a [T] {
        let &Quad(ref s) = self;
        s.as_slice()
    }
}

trait VertexPass<IN, OUT> {
    fn vertex_pass(&self, &IN) -> OUT;

    fn transform_primative<A: Poly<IN>, B: Poly<OUT>>(&self, a: &A) -> B {
        a.iter().map(|v| self.vertex_pass(v)).collect()
    }
}

trait GeometryPass<T, U, IN: Poly<T>, OUT: Poly<U>> {
    fn geometry_pass(&self, &IN) -> OUT;
}

pub struct IndexDeref<'a, T> {
    vertices: &'a [T]
}

impl<'a, T> IndexDeref<'a, T> {
    pub fn new(vertices: &'a [T]) -> IndexDeref<'a, T> {
        IndexDeref {
            vertices: vertices
        }
    }
}

impl<'a, T: Clone> VertexPass<uint, T> for IndexDeref<'a, T> {
    fn vertex_pass(&self, index: &uint) -> T {
        self.vertices[*index].clone()
    }
}

trait Generator<T> : Iterator<T> {}

#[cfg(test)]
mod test {
    use IndexDeref;
    use VertexPass;
    use Trinagle;
    use Poly;

    #[test]
    fn index_deref() {
        let index = &[0, 1, 2, 3];
        let index = IndexDeref::new(index);

        assert_eq!(index.vertex_pass(&0), 0u);
        assert_eq!(index.vertex_pass(&1), 1u);
        assert_eq!(index.vertex_pass(&2), 2u);
        assert_eq!(index.vertex_pass(&3), 3u);
    }

    #[test]
    fn index_geo_deref() {
        let a = &[7u, 8, 9, 10];
        let index = IndexDeref::new(a);

        let out: Trinagle<uint> = index.transform_primative(&Trinagle::new(0u, 1, 2));
        assert_eq!(out.as_slice(), Trinagle::new(7u, 8, 9).as_slice())
    }
}