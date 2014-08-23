use std::collections::{RingBuf, Deque};

use {
    Quad,
    Triangle,
    PolyTri,
    PolyQuad,
    Polygon,
};

pub trait EmitTriangles<T> {
    fn emit_triangles(&self, emit: |Triangle<T>|);
}

impl<T: Clone> EmitTriangles<T> for Quad<T> {
    fn emit_triangles(&self, emit: |Triangle<T>|) {
        let &Quad{x: ref x, y: ref y, z: ref z, w: ref w} = self;
        emit(Triangle::new(x.clone(), y.clone(), z.clone()));
        emit(Triangle::new(z.clone(), w.clone(), x.clone()));
    }
}

impl<T: Clone> EmitTriangles<T> for Triangle<T> {
    fn emit_triangles(&self, emit: |Triangle<T>|) {
        emit(self.clone());
    }
}

impl<T: Clone> EmitTriangles<T> for Polygon<T> {
    fn emit_triangles(&self, emit: |Triangle<T>|) {
        match self {
            &PolyTri(ref t) => t.emit_triangles(emit),
            &PolyQuad(ref q) => q.emit_triangles(emit),
        }
    }
}

pub trait Triangulate<T, V> {
    fn triangluate(self) -> TriangulateIterator<T, V>;
}

impl<V, P: EmitTriangles<V>, T: Iterator<P>> Triangulate<T, V> for T {
    fn triangluate(self) -> TriangulateIterator<T, V> {
        TriangulateIterator::new(self)
    }
}

pub struct TriangulateIterator<SRC, V> {
    source: SRC,
    buffer: RingBuf<Triangle<V>>
}

impl<V, U: EmitTriangles<V>, SRC: Iterator<U>> TriangulateIterator<SRC, V> {
    pub fn new(src: SRC) -> TriangulateIterator<SRC, V> {
        TriangulateIterator {
            source: src,
            buffer: RingBuf::new()
        }
    }
}

impl<V, U: EmitTriangles<V>, SRC: Iterator<U>> Iterator<Triangle<V>> for TriangulateIterator<SRC, V> {
    fn next(&mut self) -> Option<Triangle<V>> {
        loop {
            match self.buffer.pop_front() {
                Some(v) => return Some(v),
                None => ()
            }

            match self.source.next() {
                Some(p) => p.emit_triangles(|v| self.buffer.push(v)),
                None => return None
            }
        }
    }
}
