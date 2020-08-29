use std::collections::VecDeque;
use std::marker::PhantomData;

/// A polygon with 4 points. Maps to `GL_QUADS`.
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct Quad<T> {
    /// The first point of the quad
    pub x: T,
    /// The second point of the quad
    pub y: T,
    /// The third point of the quad
    pub z: T,
    /// The fourth point of the quad
    pub w: T,
}

impl<T> Quad<T> {
    /// Create a new `Quad` with the supplied vertices.
    pub fn new(v0: T, v1: T, v2: T, v3: T) -> Self {
        Quad {
            x: v0,
            y: v1,
            z: v2,
            w: v3,
        }
    }
}

/// A polygon with 3 points. Maps to `GL_TRIANGLE`.
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct Triangle<T> {
    /// the first point of the triangle
    pub x: T,
    /// the second point of the triangle
    pub y: T,
    /// the third point of the triangle
    pub z: T,
}

impl<T> Triangle<T> {
    /// Create a new `Triangle` with the supplied vertices.
    pub fn new(v0: T, v1: T, v2: T) -> Self {
        Triangle {
            x: v0,
            y: v1,
            z: v2,
        }
    }
}

/// This is All-the-types container. This exists since some generators
/// produce both [`Triangles`] and [`Quads`].
///
/// [`Triangles`]: struct.Triangle.html
/// [`Quads`]: struct.Quad.html
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Polygon<T> {
    /// A wrapped triangle
    PolyTri(Triangle<T>),
    /// A wrapped quad
    PolyQuad(Quad<T>),
}

/// The core mechanism of the [`Vertices`] trait. This is a mechanism for unwrapping
/// a polygon extracting all of the vertices that it bound together.
///
/// [`Vertices`]: trait.Vertices.html
pub trait EmitVertices<T> {
    /// Consume a [`Polygon`], each vertex is emitted to the parent function by
    /// calling the supplied lambda function.
    ///
    /// [`Polygon`]: enum.Polygon.html
    fn emit_vertices<F>(self, f: F)
    where
        F: FnMut(T);
}

impl<T> EmitVertices<T> for Line<T> {
    fn emit_vertices<F>(self, mut emit: F)
    where
        F: FnMut(T),
    {
        let Line { x, y } = self;
        emit(x);
        emit(y);
    }
}

impl<T> EmitVertices<T> for Triangle<T> {
    fn emit_vertices<F>(self, mut emit: F)
    where
        F: FnMut(T),
    {
        let Triangle { x, y, z } = self;
        emit(x);
        emit(y);
        emit(z);
    }
}

impl<T> EmitVertices<T> for Quad<T> {
    fn emit_vertices<F>(self, mut emit: F)
    where
        F: FnMut(T),
    {
        let Quad { x, y, z, w } = self;
        emit(x);
        emit(y);
        emit(z);
        emit(w);
    }
}

impl<T> EmitVertices<T> for Polygon<T> {
    fn emit_vertices<F>(self, emit: F)
    where
        F: FnMut(T),
    {
        use self::Polygon::{PolyQuad, PolyTri};

        match self {
            PolyTri(p) => p.emit_vertices(emit),
            PolyQuad(p) => p.emit_vertices(emit),
        }
    }
}

/// Supplies a way to convert an [`Iterator`] of [`polygons`] to an [`Iterator`]
/// of vertices. Useful for when you need to write the vertices into
/// a graphics pipeline.
///
/// [`polygons`]: enum.Polygon.html
pub trait Vertices<SRC, V> {
    /// Convert a polygon [`Iterator`] to a vertices [`Iterator`].
    fn vertices(self) -> VerticesIterator<SRC, V>;
}

impl<V, P: EmitVertices<V>, T: Iterator<Item = P>> Vertices<T, V> for T {
    fn vertices(self) -> VerticesIterator<T, V> {
        VerticesIterator {
            source: self,
            buffer: VecDeque::new(),
        }
    }
}

/// An [`Iterator`] that breaks a [`Polygon`] down into its individual
/// vertices.
///
/// This `struct` is created by the [`vertices`] method on [`Vertices`].
///
/// [`Polygon`]: enum.Polygon.html
/// [`Vertices`]: trait.Vertices.html
/// [`vertices`]: trait.Vertices.html#method.vertices
pub struct VerticesIterator<SRC, V> {
    source: SRC,
    buffer: VecDeque<V>,
}

impl<V, U: EmitVertices<V>, SRC: Iterator<Item = U>> Iterator for VerticesIterator<SRC, V> {
    type Item = V;

    fn next(&mut self) -> Option<V> {
        loop {
            if let v @ Some(_) = self.buffer.pop_front() {
                break v;
            }

            self.source
                .next()?
                .emit_vertices(|v| self.buffer.push_back(v));
        }
    }
}

/// Equivalent of `map` but per-vertex.
pub trait MapVertex<T, U> {
    /// `Output` should be a container of the same shape of the type.
    /// It's internal values should reflect any transformation the map did.
    type Output;
    /// Map a function to each vertex in a [`Polygon`] creating a new [`Polygon`].
    ///
    /// [`Polygon`]: enum.Polygon.html
    fn map_vertex<F>(self, f: F) -> Self::Output
    where
        F: FnMut(T) -> U;
}

impl<T: Clone, U> MapVertex<T, U> for Line<T> {
    type Output = Line<U>;

    fn map_vertex<F>(self, mut map: F) -> Line<U>
    where
        F: FnMut(T) -> U,
    {
        let Line { x, y } = self;
        Line {
            x: map(x),
            y: map(y),
        }
    }
}

impl<T: Clone, U> MapVertex<T, U> for Triangle<T> {
    type Output = Triangle<U>;

    fn map_vertex<F>(self, mut map: F) -> Triangle<U>
    where
        F: FnMut(T) -> U,
    {
        let Triangle { x, y, z } = self;
        Triangle {
            x: map(x),
            y: map(y),
            z: map(z),
        }
    }
}

impl<T: Clone, U> MapVertex<T, U> for Quad<T> {
    type Output = Quad<U>;

    fn map_vertex<F>(self, mut map: F) -> Quad<U>
    where
        F: FnMut(T) -> U,
    {
        let Quad { x, y, z, w } = self;
        Quad {
            x: map(x),
            y: map(y),
            z: map(z),
            w: map(w),
        }
    }
}

impl<T: Clone, U> MapVertex<T, U> for Polygon<T> {
    type Output = Polygon<U>;

    fn map_vertex<F>(self, map: F) -> Polygon<U>
    where
        F: FnMut(T) -> U,
    {
        use self::Polygon::{PolyQuad, PolyTri};

        match self {
            PolyTri(p) => PolyTri(p.map_vertex(map)),
            PolyQuad(p) => PolyQuad(p.map_vertex(map)),
        }
    }
}

/// This acts very similar to a vertex shader. It gives a way to manipulate
/// and modify the vertices in a [`Polygon`]. This is useful if you need to
/// scale the mesh using a matrix multiply, or just for modifying the type of
/// each vertex.
///
/// [`Polygon`]: enum.Polygon.html
pub trait MapToVertices<T, U>: Sized {
    /// `Output` should be a a container of the same shape of the type.
    /// It's internal values should reflect any transformation the map did.
    type Output;

    /// Produces an [`Iterator`] of mapped polygons from an [`Iterator`] of polygons.
    /// Each vertex in the process is modified with the supplied function.
    fn vertex<F>(self, map: F) -> MapToVerticesIter<Self, T, U, F>
    where
        F: FnMut(T) -> U;
}

impl<VIn, VOut, P, POut: MapVertex<VIn, VOut, Output = P>, T: Iterator<Item = POut>>
    MapToVertices<VIn, VOut> for T
{
    type Output = P;

    fn vertex<F>(self, map: F) -> MapToVerticesIter<T, VIn, VOut, F>
    where
        F: FnMut(VIn) -> VOut,
    {
        MapToVerticesIter {
            src: self,
            f: map,
            phantom: PhantomData,
        }
    }
}

/// An [`Iterator`] that maps vertices with a given function.
///
/// This `struct` is created by the [`vertex`] method on [`MapToVertices`].
///
/// [`vertex`]: trait.MapToVertices.html#method.vertex
/// [`MapToVertices`]: trait.MapToVertices.html
pub struct MapToVerticesIter<SRC, T, U, F: FnMut(T) -> U> {
    src: SRC,
    f: F,
    phantom: PhantomData<(T, U)>,
}

impl<
        'a,
        P,
        POut: MapVertex<T, U, Output = P>,
        SRC: Iterator<Item = POut>,
        T,
        U,
        F: FnMut(T) -> U,
    > Iterator for MapToVerticesIter<SRC, T, U, F>
{
    type Item = P;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.src.size_hint()
    }

    fn next(&mut self) -> Option<P> {
        self.src.next().map(|x| x.map_vertex(|x| (self.f)(x)))
    }
}

/// Represents a line.
#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub struct Line<T> {
    /// The first point
    pub x: T,
    /// The second point
    pub y: T,
}

impl<T> Line<T> {
    /// Create a new line using point x and y.
    pub fn new(x: T, y: T) -> Self {
        Line { x, y }
    }
}

/// Convert a [`Polygon`] into it's fragments.
///
/// [`Polygon`]: enum.Polygon.html
pub trait EmitLines {
    /// The Vertex defines the corners of a [`Polygon`].
    ///
    /// [`Polygon`]: enum.Polygon.html
    type Vertex;

    /// Convert a polygon into lines, each [`Line`] is emitted via
    /// calling of the callback of `emit`. This allows for
    /// a variable amount of lines to be returned.
    ///
    /// [`Polygon`]: enum.Polygon.html
    /// [`Line`]: struct.Line.html
    fn emit_lines<E>(self, emit: E)
    where
        E: FnMut(Line<Self::Vertex>);
}

impl<T: Clone> EmitLines for Triangle<T> {
    type Vertex = T;

    fn emit_lines<E>(self, mut emit: E)
    where
        E: FnMut(Line<T>),
    {
        emit(Line::new(self.x.clone(), self.y.clone()));
        emit(Line::new(self.y, self.z.clone()));
        emit(Line::new(self.z, self.x));
    }
}

impl<T: Clone> EmitLines for Quad<T> {
    type Vertex = T;

    fn emit_lines<E>(self, mut emit: E)
    where
        E: FnMut(Line<T>),
    {
        emit(Line::new(self.x.clone(), self.y.clone()));
        emit(Line::new(self.y, self.z.clone()));
        emit(Line::new(self.z, self.w.clone()));
        emit(Line::new(self.w, self.x));
    }
}

impl<T: Clone> EmitLines for Polygon<T> {
    type Vertex = T;

    fn emit_lines<E>(self, emit: E)
    where
        E: FnMut(Line<T>),
    {
        match self {
            Polygon::PolyTri(x) => x.emit_lines(emit),
            Polygon::PolyQuad(x) => x.emit_lines(emit),
        }
    }
}

/// Supplies a way to convert an [`Iterator`] of [`polygons`] into an [`Iterator`] of
/// the [`polygons`] lines
///
/// [`polygons`]: enum.Polygon.html
pub trait Lines: Sized {
    /// The type of each point in the lines.
    type Vertex;

    /// Convert the [`Iterator`] into a [`LinesIterator`].
    ///
    /// [`LinesIterator`]: struct.LinesIterator.html
    fn lines(self) -> LinesIterator<Self, Self::Vertex>;
}

impl<T, P, V> Lines for T
where
    T: Iterator<Item = P>,
    P: EmitLines<Vertex = V>,
{
    type Vertex = V;

    fn lines(self) -> LinesIterator<T, V> {
        LinesIterator {
            source: self,
            buffer: VecDeque::new(),
        }
    }
}

/// An [`Iterator`] that turns polygons into an [`Iterator`] of lines.
///
/// This `struct` is created by the [`lines`] method on [`Lines`].
///
/// [`lines`]: trait.Lines.html#method.lines
/// [`Lines`]: trait.Lines.html
pub struct LinesIterator<I, V> {
    source: I,
    buffer: VecDeque<Line<V>>,
}

impl<I, P, V> Iterator for LinesIterator<I, V>
where
    I: Iterator<Item = P>,
    P: EmitLines<Vertex = V>,
{
    type Item = Line<V>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (n, _) = self.source.size_hint();
        (n, None)
    }

    fn next(&mut self) -> Option<Line<V>> {
        loop {
            if let v @ Some(_) = self.buffer.pop_front() {
                break v;
            }

            self.source.next()?.emit_lines(|v| self.buffer.push_back(v));
        }
    }
}
