use std::collections::{RingBuf, Deque};

use {
    Quad,
    Triangle,
    PolyTri,
    PolyQuad,
    Polygon,
    TrianglePipeline,
};

pub trait ToTriangles<T> {
    fn to_triangles(&self, emit: |Triangle<T>|);
}

impl<T: Clone> ToTriangles<T> for Quad<T> {
    fn to_triangles(&self, emit: |Triangle<T>|) {
        let &Quad{x: ref x, y: ref y, z: ref z, w: ref w} = self;
        emit(Triangle::new(x.clone(), y.clone(), z.clone()));
        emit(Triangle::new(z.clone(), w.clone(), x.clone()));
    }
}

impl<T: Clone> ToTriangles<T> for Triangle<T> {
    fn to_triangles(&self, emit: |Triangle<T>|) {
        emit(self.clone());
    }
}

impl<T: Clone> ToTriangles<T> for Polygon<T> {
    fn to_triangles(&self, emit: |Triangle<T>|) {
        match self {
            &PolyTri(ref t) => t.to_triangles(emit),
            &PolyQuad(ref q) => q.to_triangles(emit),
        }
    }
}

pub struct TriangluateMesh<SRC, V> {
    source: SRC,
    buffer: RingBuf<Triangle<V>>
}

impl<V, U: ToTriangles<V>, SRC: Iterator<U>> TriangluateMesh<SRC, V> {
    pub fn new(src: SRC) -> TriangluateMesh<SRC, V> {
        TriangluateMesh {
            source: src,
            buffer: RingBuf::new()
        }
    }
}

impl<V, U: ToTriangles<V>, SRC: Iterator<U>> Iterator<Triangle<V>> for TriangluateMesh<SRC, V> {
    fn next(&mut self) -> Option<Triangle<V>> {
        loop {
            match self.buffer.pop_front() {
                Some(v) => return Some(v),
                None => ()
            }

            match self.source.next() {
                Some(p) => p.to_triangles(|v| self.buffer.push(v)),
                None => return None
            }
        }
    }
}

impl<V: Clone, U: ToTriangles<V>, SRC: Iterator<U>> TrianglePipeline<V> for TriangluateMesh<SRC, V> {}
