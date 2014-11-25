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

use std::iter::Range;

/// The `SharedVertex` trait is meant to be used with the `IndexedPolygon` trait.
/// This trait is meant as a way to calculate the shared vertices that are
/// required to build the implementors mesh.
pub trait SharedVertex<V> {
    /// return the shared vertex at offset `i`
    fn shared_vertex(&self, i: uint) -> V;

    /// return the number of shared vertices required to represent the mesh
    fn shared_vertex_count(&self) -> uint;

    /// create an iterator that returns each shared vertex that is required to
    /// build the mesh.
    fn shared_vertex_iter<'a>(&'a self) -> SharedVertexIterator<'a, Self, V> {
        SharedVertexIterator {
            base: self,
            idx: range(0, self.shared_vertex_count())
        }
    }
}

pub struct SharedVertexIterator<'a, T:'a, V> {
    base: &'a T,
    idx: Range<uint>
}

impl<'a, T: SharedVertex<V>, V> Iterator<V> for SharedVertexIterator<'a, T, V> {
    fn next(&mut self) -> Option<V> {
        self.idx.next().map(|idx| self.base.shared_vertex(idx))
    }
}

/// The `IndexedPolygon` trait is used with the `SharedVertex` trait in order to build
/// a mesh. `IndexedPolygon` calculates each polygon face required to build an implementors mesh.
/// each face is always returned in indexed form that points to the correct vertice supplied
/// by the `SharedVertex` trait.
pub trait IndexedPolygon<V> {
    /// return a polygon with indices to the shared vertex
    fn indexed_polygon(&self, i: uint) -> V;

    /// return the number of polygons that are needed to represent this mesh
    fn indexed_polygon_count(&self) -> uint;

    /// create a iterator that will return a polygon for each face in the source mesh
    fn indexed_polygon_iter<'a>(&'a self) -> IndexedPolygonIterator<'a, Self, V> {
        IndexedPolygonIterator {
            base: self,
            idx: range(0, self.indexed_polygon_count())
        }
    }
}

pub struct IndexedPolygonIterator<'a, T:'a, V> {
    base: &'a T,
    idx: Range<uint>
}


impl<'a, T: IndexedPolygon<V>, V> Iterator<V> for IndexedPolygonIterator<'a, T, V> {
    fn next(&mut self) -> Option<V> {
        self.idx.next().map(|idx| self.base.indexed_polygon(idx))
    }
}