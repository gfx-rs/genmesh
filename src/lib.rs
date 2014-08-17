
extern crate core;

use std::iter::FromIterator;
use core::slice::Items;
use std::collections::RingBuf;
use std::collections::Deque;

mod cube;

pub mod generators {
    pub use cube::Cube;
}

pub struct Vector1<T>(pub [T, ..1]);
pub struct Vector2<T>(pub [T, ..2]);
pub struct Vector3<T>(pub [T, ..3]);
pub struct Vector4<T>(pub [T, ..4]);

impl<T> FromIterator<T> for Vector1<T> {
    fn from_iter<I: Iterator<T>>(iter: I) -> Vector1<T> {
        let mut iter = iter;
        match iter.next() {
            Some(a) => Vector1([a]),
            _ => fail!("should have found 2 vertices to buld a Vector1")
        }
    }
}

impl<T: Clone> Clone for Vector1<T> {
    fn clone(&self) -> Vector1<T> {
        let &Vector1(ref v) = self;
        Vector1([v[0].clone()])
    }
}

impl<T> Poly<T> for Vector1<T> {
    fn as_slice<'a>(&'a self) -> &'a [T] {
        let &Vector1(ref s) = self;
        s.as_slice()
    }
}

impl<T> FromIterator<T> for Vector2<T> {
    fn from_iter<I: Iterator<T>>(iter: I) -> Vector2<T> {
        let mut iter = iter;
        match (iter.next(), iter.next()) {
            (Some(a), Some(b)) => Vector2([a, b]),
            _ => fail!("should have found 2 vertices to buld a Vector2")
        }
    }
}

impl<T: Clone> Clone for Vector2<T> {
    fn clone(&self) -> Vector2<T> {
        let &Vector2(ref v) = self;
        Vector2([v[0].clone(),
                 v[1].clone()])
    }
}

impl<T> Poly<T> for Vector2<T> {
    fn as_slice<'a>(&'a self) -> &'a [T] {
        let &Vector2(ref s) = self;
        s.as_slice()
    }
}

impl<T> FromIterator<T> for Vector3<T> {
    fn from_iter<I: Iterator<T>>(iter: I) -> Vector3<T> {
        let mut iter = iter;
        match (iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), Some(c)) => Vector3([a, b, c]),
            _ => fail!("should have found 3 vertices to buld a Vector3")
        }
    }
}

impl<T: Clone> Clone for Vector3<T> {
    fn clone(&self) -> Vector3<T> {
        let &Vector3(ref v) = self;
        Vector3([v[0].clone(),
                 v[1].clone(),
                 v[2].clone()])
    }
}

impl<T> Poly<T> for Vector3<T> {
    fn as_slice<'a>(&'a self) -> &'a [T] {
        let &Vector3(ref s) = self;
        s.as_slice()
    }
}

impl<T> FromIterator<T> for Vector4<T> {
    fn from_iter<I: Iterator<T>>(iter: I) -> Vector4<T> {
        let mut iter = iter;
        match (iter.next(), iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), Some(c), Some(d)) => Vector4([a, b, c, d]),
            _ => fail!("should have found 4 vertices to buld a quad")
        }
    }
}

impl<T: Clone> Clone for Vector4<T> {
    fn clone(&self) -> Vector4<T> {
        let &Vector4(ref v) = self;
        Vector4([v[0].clone(),
                 v[1].clone(),
                 v[2].clone(),
                 v[3].clone()])
    }
}

impl<T> Poly<T> for Vector4<T> {
    fn as_slice<'a>(&'a self) -> &'a [T] {
        let &Vector4(ref s) = self;
        s.as_slice()
    }
}

trait Poly<T> : FromIterator<T> {
    fn as_slice<'a>(&'a self) -> &'a [T];

    fn iter<'a>(&'a self) -> Items<T> {
        self.as_slice().iter()
    }
}

pub struct Triangle<T>(Vector3<T>);

impl<T> Triangle<T> {
    pub fn new(v0: T, v1: T, v2: T) -> Triangle<T> {
        Triangle(Vector3([v0, v1, v2]))
    }
}

impl<T: Clone> Clone for Triangle<T> {
    fn clone(&self) -> Triangle<T> {
        let &Triangle(ref v) = self;
        Triangle(v.clone()) 
    }
}

impl<T> FromIterator<T> for Triangle<T> {
    fn from_iter<I: Iterator<T>>(iter: I) -> Triangle<T> {
        let mut iter = iter;
        match (iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), Some(c)) => Triangle(Vector3([a, b, c])),
            _ => fail!("should have found 3 vertices to buld a triangle")
        }
    }
}

impl<T> Poly<T> for Triangle<T> {
    fn as_slice<'a>(&'a self) -> &'a [T] {
        let &Triangle(Vector3(ref s)) = self;
        s.as_slice()
    }
}

pub struct Quad<T>(pub Vector4<T>);

impl<T> Quad<T> {
    pub fn new(v0: T, v1: T, v2: T, v3: T) -> Quad<T> {
        Quad(Vector4([v0, v1, v2, v3]))
    }
}

pub trait ToTriangle<T> {
    fn to_triangles(&self) -> T;
}

impl<T: Clone> ToTriangle<Vector2<Triangle<T>>> for Quad<T> {
    fn to_triangles(&self) -> Vector2<Triangle<T>> {
        let &Quad(Vector4([ref v0, ref v1, ref v2, ref v3])) = self;
        Vector2([Triangle::new(v0.clone(), v1.clone(), v2.clone()),
                 Triangle::new(v2.clone(), v3.clone(), v0.clone())])
    }
}

impl<T> FromIterator<T> for Quad<T> {
    fn from_iter<I: Iterator<T>>(iter: I) -> Quad<T> {
        let mut iter = iter;
        match (iter.next(), iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), Some(c), Some(d)) => Quad(Vector4([a, b, c, d])),
            _ => fail!("should have found 4 vertices to buld a quad")
        }
    }
}

impl<T> Poly<T> for Quad<T> {
    fn as_slice<'a>(&'a self) -> &'a [T] {
        let &Quad(Vector4(ref s)) = self;
        s.as_slice()
    }
}

trait VertexPass<IN, OUT> {
    fn vertex_pass(&self, &IN) -> OUT;

    fn transform_primative<A: Poly<IN>, B: Poly<OUT>>(&self, a: &A) -> B {
        a.iter().map(|v| self.vertex_pass(v)).collect()
    }
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

pub trait Generator<T, P: Poly<T>> : Iterator<P> {
    fn vertices(self) -> VertexGenerator<Self, T> {
        VertexGenerator {
            source: self,
            spare: Vec::new()
        }
    }

    fn geometry<'a, OUT_P>(self, max: uint, f: |P, emit: |OUT_P||:'a) -> GeometryMap<'a, Self, P, OUT_P> {
        GeometryMap {
            source: self,
            max: max,
            buffer: Some(RingBuf::with_capacity(max)),
            f: f
        }
    }
}

pub struct GeometryMap<'a, SRC, IN_P, OUT_P> {
    source: SRC,
    max: uint,
    buffer: Option<RingBuf<OUT_P>>,
    f: |IN_P, |OUT_P||:'a
}

impl<'a, IN_P, OUT_P, SRC: Iterator<IN_P>> Iterator<OUT_P> for GeometryMap<'a, SRC, IN_P, OUT_P> {
    fn next(&mut self) -> Option<OUT_P> {
        loop {
            match self.buffer.get_mut_ref().pop_front() {
                p @ Some(_) => return p,
                None => ()
            }

            match self.source.next() {
                Some(p) => {
                    let mut buf = self.buffer.take().unwrap();
                    (self.f)(p, |emited| buf.push(emited));
                    self.buffer = Some(buf);
                }
                None => return None
            }

        }
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        match self.source.size_hint() {
            (base, Some(s)) => {
                (base, Some(s*self.max))
            },
            (base, None) => {
                (base, Some(base*self.max))
            }
        }
    }
}

impl<'a, IN_P, OUT_V, OUT_P: Poly<OUT_V>, SRC: Iterator<IN_P>> Generator<OUT_V, OUT_P> for GeometryMap<'a, SRC, IN_P, OUT_P> {}

impl<'a, IN_V, IN_P: Poly<IN_V>,
         OUT_V, OUT_P: Poly<OUT_V>,
         SRC: Iterator<IN_P>> Generator<OUT_V, OUT_P> for core::iter::Map<'a, IN_P, OUT_P, SRC> {}

pub struct VertexGenerator<SRC, T> {
    source: SRC,
    spare: Vec<T>
}

impl<T: Clone, P: Poly<T>, SRC: Iterator<P>> Iterator<T> for VertexGenerator<SRC, T> {
    fn next(&mut self) -> Option<T> {
        loop {
            match self.spare.remove(0) {
                out @ Some(_) => return out,
                None => ()
            }

            match self.source.next() {
                Some(s) => {
                    self.spare.push_all(s.as_slice());
                }
                None => return None
            }
        }
    }
}


#[cfg(test)]
mod test {
    use IndexDeref;
    use VertexPass;
    use Triangle;
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

        let out: Triangle<uint> = index.transform_primative(&Triangle::new(0u, 1, 2));
        assert_eq!(out.as_slice(), Triangle::new(7u, 8, 9).as_slice())
    }
}