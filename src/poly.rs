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

use std::collections::{RingBuf, Deque};

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

pub trait EmitVertices<T> {
    fn emit_vertices(self, f: |T|);
}

impl<T> EmitVertices<T> for Triangle<T> {
    fn emit_vertices(self, emit: |T|) {
        let Triangle{x: x, y: y, z: z} = self;
        emit(x);
        emit(y);
        emit(z);
    }
}

impl<T> EmitVertices<T> for Quad<T> {
    fn emit_vertices(self, emit: |T|) {
        let Quad{x: x, y: y, z: z, w: w} = self;
        emit(x);
        emit(y);
        emit(z);
        emit(w);
    }
}

impl<T> EmitVertices<T> for Polygon<T> {
    fn emit_vertices(self, emit: |T|) {
        match self {
            PolyTri(p) => p.emit_vertices(emit),
            PolyQuad(p) => p.emit_vertices(emit)
        }
    }
}

pub trait Vertices<SRC, V> {
    fn vertices(self) -> VerticesIterator<SRC, V>;
}

impl<V, P: EmitVertices<V>, T: Iterator<P>> Vertices<T, V> for T {
    fn vertices(self) -> VerticesIterator<T, V> {
        VerticesIterator {
            source: self,
            buffer: RingBuf::new()
        }
    }    
}

pub struct VerticesIterator<SRC, V> {
    source: SRC,
    buffer: RingBuf<V>
}

impl<V, U: EmitVertices<V>, SRC: Iterator<U>> Iterator<V> for VerticesIterator<SRC, V> {
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