use std::marker::PhantomData;
use std::ops::Range;

/// The `SharedVertex` trait is meant to be used with the [`IndexedPolygon`] trait.
/// This trait is meant as a way to calculate the shared vertices that are
/// required to build the implementors mesh.
///
/// [`IndexedPolygon`]: trait.IndexedPolygon.html
pub trait SharedVertex<V>: Sized {
    /// Returns the shared vertex at offset `i`.
    fn shared_vertex(&self, i: usize) -> V;

    /// Returns the number of shared vertices required to represent the mesh.
    fn shared_vertex_count(&self) -> usize;

    /// Create an [`Iterator`] that returns each shared vertex that is required to
    /// build the mesh.
    fn shared_vertex_iter(&self) -> SharedVertexIterator<Self, V> {
        SharedVertexIterator {
            base: self,
            idx: 0..self.shared_vertex_count(),
            phantom_v: PhantomData,
        }
    }
}

/// An [`Iterator`] that yields the shared vertices of the mesh.
///
/// This `struct` is created by the [`shared_vertex_iter`] method on [`SharedVertex`].
///
/// [`shared_vertex_iter`]: trait.SharedVertex.html#method.shared_vertex_iter
/// [`SharedVertex`]: trait.SharedVertex.html
pub struct SharedVertexIterator<'a, T: 'a, V> {
    base: &'a T,
    idx: Range<usize>,
    phantom_v: PhantomData<V>,
}

impl<'a, T: SharedVertex<V>, V> Iterator for SharedVertexIterator<'a, T, V> {
    type Item = V;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.idx.size_hint()
    }

    fn next(&mut self) -> Option<V> {
        self.idx.next().map(|idx| self.base.shared_vertex(idx))
    }
}

impl<'a, T: SharedVertex<V>, V> ExactSizeIterator for SharedVertexIterator<'a, T, V> {
    fn len(&self) -> usize {
        self.idx.len()
    }
}

/// The `IndexedPolygon` trait is used with the [`SharedVertex`] trait in order to build
/// a mesh. `IndexedPolygon` calculates each polygon face required to build an implementors mesh.
/// Each face is always returned in indexed form that points to the correct vertice supplied
/// by the [`SharedVertex`] trait.
///
/// [`SharedVertex`]: trait.SharedVertex.html
pub trait IndexedPolygon<V>: Sized {
    /// Returns a polygon with indices to the shared vertex.
    fn indexed_polygon(&self, i: usize) -> V;

    /// Returns the number of polygons that are needed to represent this mesh.
    fn indexed_polygon_count(&self) -> usize;

    /// Creates an [`Iterator`] that will return a polygon for each face in the source mesh.
    fn indexed_polygon_iter(&self) -> IndexedPolygonIterator<Self, V> {
        IndexedPolygonIterator {
            base: self,
            idx: 0..self.indexed_polygon_count(),
            phantom_v: PhantomData,
        }
    }
}

/// An [`Iterator`] that yields the indices of the mesh
///
/// This `struct` is created by the [`indexed_polygon_iter`] method on [`IndexedPolygon`].
///
/// [`indexed_polygon_iter`]: trait.IndexedPolygon.html#method.indexed_polygon_iter
/// [`IndexedPolygon`]: trait.IndexedPolygon.html
pub struct IndexedPolygonIterator<'a, T: 'a, V> {
    base: &'a T,
    idx: Range<usize>,
    phantom_v: PhantomData<V>,
}

impl<'a, T: IndexedPolygon<V>, V> Iterator for IndexedPolygonIterator<'a, T, V> {
    type Item = V;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.idx.size_hint()
    }

    fn next(&mut self) -> Option<V> {
        self.idx.next().map(|idx| self.base.indexed_polygon(idx))
    }
}

impl<'a, T: IndexedPolygon<V>, V> ExactSizeIterator for IndexedPolygonIterator<'a, T, V> {
    fn len(&self) -> usize {
        self.idx.len()
    }
}

