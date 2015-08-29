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

use std::marker::PhantomData;
use std::collections::VecDeque;

/// A polygon with 4 points. Maps to `GL_QUADS`
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct Quad<T> {
    /// the first point of a quad
    pub x: T,
    /// the second point of a quad
    pub y: T,
    /// the third point of a quad
    pub z: T,
    /// the fourth point of a quad
    pub w: T,
}

impl<T> Quad<T> {
    /// create a new `Quad` with supplied vertices
    pub fn new(v0: T, v1: T, v2: T, v3: T) -> Quad<T> {
        Quad {
            x: v0,
            y: v1,
            z: v2,
            w: v3
        }
    }
}

/// A polygon with 3 points. Maps to `GL_TRIANGLE`
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct Triangle<T> {
    /// the first point of a triangle
    pub x: T,
    /// the second point of a triangle
    pub y: T,
    /// the third point of a triangle
    pub z: T,
}

impl<T> Triangle<T> {
    /// create a new `Triangle` with supplied vertcies
    pub fn new(v0: T, v1: T, v2: T) -> Triangle<T> {
        Triangle {
            x: v0,
            y: v1,
            z: v2
        }
    }
}

/// This is All-the-types container. This exists since some generators
/// produce both `Triangles` and `Quads`.
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Polygon<T> {
    /// A wraped triangle
    PolyTri(Triangle<T>),
    /// A wraped quad
    PolyQuad(Quad<T>)
}

/// The core mechanism of `Vertices` trait. This is a mechanism for unwraping
/// a polygon extracting all of the vertices that it bound together.
pub trait EmitVertices<T> {
    /// Consume a polygon, each
    /// vertex is emitted to the parent function by calling the supplied
    /// lambda function
    fn emit_vertices<F>(self, mut emit: F) where F: FnMut(T);
}

impl<T> EmitVertices<T> for Triangle<T> {
    fn emit_vertices<F>(self, mut emit: F) where F: FnMut(T) {
        let Triangle{x, y, z} = self;
        emit(x);
        emit(y);
        emit(z);
    }
}

impl<T> EmitVertices<T> for Quad<T> {
    fn emit_vertices<F>(self, mut emit: F) where F: FnMut(T) {
        let Quad{x, y, z, w} = self;
        emit(x);
        emit(y);
        emit(z);
        emit(w);
    }
}

impl<T> EmitVertices<T> for Polygon<T> {
    fn emit_vertices<F>(self, emit: F) where F: FnMut(T) {
        use self::Polygon::{ PolyQuad, PolyTri };

        match self {
            PolyTri(p) => p.emit_vertices(emit),
            PolyQuad(p) => p.emit_vertices(emit)
        }
    }
}

/// Supplies a way to convert an iterator of polygons to an iterator
/// of vertices. Useful for when you need to write the vertices into
/// a graphics pipeline.
pub trait Vertices<SRC, V> {
    /// Convert a polygon iterator to a vertices iterator.
    fn vertices(self) -> VerticesIterator<SRC, V>;
}

impl<V, P: EmitVertices<V>, T: Iterator<Item=P>> Vertices<T, V> for T {
    fn vertices(self) -> VerticesIterator<T, V> {
        VerticesIterator {
            source: self,
            buffer: VecDeque::new()
        }
    }
}

/// an iterator that breaks a polygon down into its individual
/// verticies.
pub struct VerticesIterator<SRC, V> {
    source: SRC,
    buffer: VecDeque<V>
}

impl<V, U: EmitVertices<V>, SRC: Iterator<Item=U>> Iterator for VerticesIterator<SRC, V> {
    type Item = V;

    fn next(&mut self) -> Option<V> {
        loop {
            match self.buffer.pop_front() {
                Some(v) => return Some(v),
                None => ()
            }

            match self.source.next() {
                Some(p) => p.emit_vertices(|v| self.buffer.push_back(v)),
                None => return None
            }
        }
    }
}

/// equivalent of `map` but per-vertex
pub trait MapVertex<T, U> {
    /// `Output` should be a a container of the same shape of the type
    /// It's internal values should reflect any transformation the map did.
    type Output;
    /// map a function to each vertex in polygon creating a new polygon
    fn map_vertex<F>(self, mut map: F) -> Self::Output where F: FnMut(T) -> U;
}

impl<T: Clone, U> MapVertex<T, U> for Triangle<T> {
    type Output = Triangle<U>;

    fn map_vertex<F>(self, mut map: F) -> Triangle<U> where F: FnMut(T) -> U {
        let Triangle{x, y, z} = self;
        Triangle {
            x: map(x),
            y: map(y),
            z: map(z)
        }
    }
}

impl<T: Clone, U> MapVertex<T, U> for Quad<T> {
    type Output = Quad<U>;

    fn map_vertex<F>(self, mut map: F) -> Quad<U> where F: FnMut(T) -> U {
        let Quad{x, y, z, w} = self;
        Quad {
            x: map(x),
            y: map(y),
            z: map(z),
            w: map(w)
        }
    }
}

impl<T: Clone, U> MapVertex<T, U> for Polygon<T> {
    type Output = Polygon<U>;

    fn map_vertex<F>(self, map: F) -> Polygon<U> where F: FnMut(T) -> U {
        use self::Polygon::{ PolyTri, PolyQuad };

        match self {
            PolyTri(p) => PolyTri(p.map_vertex(map)),
            PolyQuad(p) => PolyQuad(p.map_vertex(map))
        }
    }
}

/// This acts very similar to a vertex shader. It gives a way to manipulate
/// and modify the vertices in a polygon. This is useful if you need to scale
/// the mesh using a matrix multiply, or just for modifying the type of each
/// vertex.
pub trait MapToVertices<T, U> {
    /// `Output` should be a a container of the same shape of the type
    /// It's internal values should reflect any transformation the map did.
    type Output;

    /// from a iterator of polygons, produces a iterator of polygons. Each
    /// vertex in the process is modified with the suppled function.
    fn vertex<F>(self, map: F) -> MapToVerticesIter<Self, T, U, F>
        where F: FnMut(T) -> U;
}

impl<VIn, VOut,
    P, POut: MapVertex<VIn, VOut, Output=P>,
    T: Iterator<Item=POut>>
    MapToVertices<VIn, VOut> for T {
    type Output = P;

    fn vertex<F>(self, map: F) -> MapToVerticesIter<T, VIn, VOut, F>
            where F: FnMut(VIn) -> VOut {

        MapToVerticesIter {
            src: self,
            f: map,
            phantom_t: PhantomData,
            phantom_u: PhantomData
        }
    }
}

struct MapToVerticesIter<SRC, T, U, F: FnMut(T) -> U> {
    src: SRC,
    f: F,
    phantom_t: PhantomData<T>,
    phantom_u: PhantomData<U>
}

impl<'a, P,
         POut: MapVertex<T, U, Output=P>,
         SRC: Iterator<Item=POut>,
         T, U, F: FnMut(T) -> U> Iterator for MapToVerticesIter<SRC, T, U, F> {
    type Item = P;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.src.size_hint()
    }

    fn next(&mut self) -> Option<P> {
        self.src.next().map(|x| x.map_vertex(|x| (self.f)(x)))
    }
}

/// Represents a line
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct Line<T> {
    /// the first point
    pub x: T,
    /// The second point
    pub y: T
}

impl<T> Line<T> {
    /// Create a new line using point x and y
    pub fn new(x: T, y: T) -> Line<T> {
        Line{x: x, y: y}
    }
}

/// Convert a Polygon into it's fragments
pub trait EmitLines {
    /// The Vertex defines the corners of a Polygon
    type Vertex;

    /// convert a polygon into lines, each line is emitted via
    /// calling of the callback of `emit` This allow for
    /// a variable amount of lines to be returned
    fn emit_lines<E>(self, emit: E)
        where E: FnMut(Line<Self::Vertex>);
}

impl<T: Clone> EmitLines for Triangle<T> {
    type Vertex = T;

    fn emit_lines<E>(self, mut emit: E)
        where E: FnMut(Line<T>)
    {
        emit(Line::new(self.x.clone(), self.y.clone()));
        emit(Line::new(self.y        , self.z.clone()));
        emit(Line::new(self.z        , self.x        ));
    }
}

impl<T: Clone> EmitLines for Quad<T> {
    type Vertex = T;

    fn emit_lines<E>(self, mut emit: E)
        where E: FnMut(Line<T>)
    {
        emit(Line::new(self.x.clone(), self.y.clone()));
        emit(Line::new(self.y        , self.z.clone()));
        emit(Line::new(self.z        , self.w.clone()));
        emit(Line::new(self.w        , self.x        ));
    }
}

impl<T: Clone> EmitLines for Polygon<T> {
    type Vertex = T;

    fn emit_lines<E>(self, emit: E)
        where E: FnMut(Line<T>)
    {
        match self {
            Polygon::PolyTri(x) => x.emit_lines(emit),
            Polygon::PolyQuad(x) => x.emit_lines(emit)
        }
    }
}

/// Creates an LinesIterator from another Iterator
pub trait Lines {
    /// The type of each point in the lines
    type Vertex;

    /// Convert the iterator into a LinesIterator
    fn lines(self) -> LinesIterator<Self, Self::Vertex>;
}

impl<T, P, V> Lines for T
    where T: Iterator<Item=P>,
          P: EmitLines<Vertex=V>
{
    type Vertex = V;

    fn lines(self) -> LinesIterator<T, V> {
        LinesIterator {
            source: self,
            buffer: VecDeque::new()
        }
    }
}

/// An iterator that turns Polygons into an Iterator of Lines
pub struct LinesIterator<I, V> {
    source: I,
    buffer: VecDeque<Line<V>>,
}

impl<I, P, V> Iterator for LinesIterator<I, V>
    where I: Iterator<Item=P>,
          P: EmitLines<Vertex=V>

{
    type Item = Line<V>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (n, _) = self.source.size_hint();
        (n, None)
    }

    fn next(&mut self) -> Option<Line<V>> {
        loop {
            match self.buffer.pop_front() {
                Some(v) => return Some(v),
                None => ()
            }

            match self.source.next() {
                Some(p) => p.emit_lines(|v| self.buffer.push_back(v)),
                None => return None
            }
        }
    }
}
