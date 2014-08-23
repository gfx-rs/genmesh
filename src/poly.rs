use std::collections::{RingBuf, Deque};

pub struct Vector1<T>(pub [T, ..1]);
pub struct Vector2<T>(pub [T, ..2]);
pub struct Vector3<T>(pub [T, ..3]);
pub struct Vector4<T>(pub [T, ..4]);

impl<T: Clone> Clone for Vector1<T> {
    fn clone(&self) -> Vector1<T> {
        let &Vector1(ref v) = self;
        Vector1([v[0].clone()])
    }
}

impl<T: Clone> Clone for Vector2<T> {
    fn clone(&self) -> Vector2<T> {
        let &Vector2(ref v) = self;
        Vector2([v[0].clone(),
                 v[1].clone()])
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

impl<T: Clone> Clone for Vector4<T> {
    fn clone(&self) -> Vector4<T> {
        let &Vector4(ref v) = self;
        Vector4([v[0].clone(),
                 v[1].clone(),
                 v[2].clone(),
                 v[3].clone()])
    }
}

#[deriving(Clone, Show, PartialEq, Eq)]
pub struct Quad<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

impl<T> Quad<T> {
    pub fn new(v0: T, v1: T, v2: T, v3: T) -> Quad<T> {
        Quad {
            x: v0,
            y: v1,
            z: v2,
            w: v3
        }
    }
}

impl<T: Clone> Quad<T> {
    pub fn map_vertex<U>(&self, f: |T| -> U) -> Quad<U> {
        Quad::new(f(self.x.clone()),
                  f(self.y.clone()),
                  f(self.z.clone()),
                  f(self.w.clone()))
    }
}

#[deriving(Clone, Show, PartialEq, Eq)]
pub struct Triangle<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Triangle<T> {
    pub fn new(v0: T, v1: T, v2: T) -> Triangle<T> {
        Triangle {
            x: v0,
            y: v1,
            z: v2
        }
    }
}

impl<T: Clone> Triangle<T> {
    pub fn map_vertex<U>(&self, f: |T| -> U) -> Triangle<U> {
        Triangle::new(f(self.x.clone()),
                      f(self.y.clone()),
                      f(self.z.clone()))
    }
}

#[deriving(Clone, PartialEq)]
pub enum Polygon<T> {
    PolyTri(Triangle<T>),
    PolyQuad(Quad<T>)
}

impl<T: Clone> Polygon<T> {
    pub fn map_vertex<U>(&self, f: |T| -> U) -> Polygon<U> {
        match self {
            &PolyTri(ref t) => PolyTri(t.map_vertex(f)),
            &PolyQuad(ref q) => PolyQuad(q.map_vertex(f))
        }
    }
}

pub trait Vertices<T> {
    fn emit_vertices(self, f: |T|);
}

impl<T> Vertices<T> for Triangle<T> {
    fn emit_vertices(self, emit: |T|) {
        let Triangle{x: x, y: y, z: z} = self;
        emit(x);
        emit(y);
        emit(z);
    }
}

impl<T> Vertices<T> for Quad<T> {
    fn emit_vertices(self, emit: |T|) {
        let Quad{x: x, y: y, z: z, w: w} = self;
        emit(x);
        emit(y);
        emit(z);
        emit(w);
    }
}

impl<T> Vertices<T> for Polygon<T> {
    fn emit_vertices(self, emit: |T|) {
        match self {
            PolyTri(p) => p.emit_vertices(emit),
            PolyQuad(p) => p.emit_vertices(emit)
        }
    }
}

pub trait AsVertices<SRC, V> {
    fn vertices(self) -> VerticesPipeline<SRC, V>;
}

impl<V, P: Vertices<V>, T: Iterator<P>> AsVertices<T, V> for T {
    fn vertices(self) -> VerticesPipeline<T, V> {
        VerticesPipeline {
            source: self,
            buffer: RingBuf::new()
        }
    }    
}

pub struct VerticesPipeline<SRC, V> {
    source: SRC,
    buffer: RingBuf<V>
}

impl<V, U: Vertices<V>, SRC: Iterator<U>> Iterator<V> for VerticesPipeline<SRC, V> {
    fn next(&mut self) -> Option<V> {
        loop {
            match self.buffer.pop_front() {
                Some(v) => return Some(v),
                None => ()
            }

            match self.source.next() {
                Some(p) => p.emit_vertices(|v| self.buffer.push(v)),
                None => return None
            }
        }
    }
}

pub trait MapVertex<T, U, P> {
    fn map_vertex(self, f: |T| -> U) -> P;
}

impl<T: Clone, U> MapVertex<T, U, Triangle<U>> for Triangle<T> {
    fn map_vertex(self, map: |T| -> U) -> Triangle<U> {
        let Triangle{x: x, y: y, z: z} = self;
        Triangle {
            x: map(x),
            y: map(y),
            z: map(z)
        }
    }
}

impl<T: Clone, U> MapVertex<T, U, Quad<U>> for Quad<T> {
    fn map_vertex(self, map: |T| -> U) -> Quad<U> {
        let Quad{x: x, y: y, z: z, w: w} = self;
        Quad {
            x: map(x),
            y: map(y),
            z: map(z),
            w: map(w)
        }
    }
}

impl<T: Clone, U> MapVertex<T, U, Polygon<U>> for Polygon<T> {
    fn map_vertex(self, map: |T| -> U) -> Polygon<U> {
        match self {
            PolyTri(p) => PolyTri(p.map_vertex(map)),
            PolyQuad(p) => PolyQuad(p.map_vertex(map))
        }
    }
}

pub trait MapToVertices<T, U> {
    fn vertex<'a>(self, map: |T|:'a -> U) -> MapToVerticesIter<'a, Self, T, U>;
}

impl<V_IN, V_OUT, P, P_IN: MapVertex<V_IN, V_OUT, P>, T: Iterator<P_IN>>
    MapToVertices<V_IN, V_OUT> for T {
    fn vertex<'a>(self, map: |V_IN|:'a -> V_OUT) -> MapToVerticesIter<'a, T, V_IN, V_OUT> {
        MapToVerticesIter {
            src: self,
            f: map
        }
    }
}

struct MapToVerticesIter<'a, SRC, T, U> {
    src: SRC,
    f: |T|:'a -> U
}

impl<'a, P_IN: MapVertex<T, U, P>,
         SRC: Iterator<P_IN>, T, U, P> Iterator<P> for MapToVerticesIter<'a, SRC, T, U> {
    fn next(&mut self) -> Option<P> {
        self.src.next().map(|x| x.map_vertex(|x| (self.f)(x)))
    }
}