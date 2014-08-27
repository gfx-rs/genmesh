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

pub trait SharedVertex<V> {
    /// return the shared vertex at offset
    fn shared_vertex(&self, idx: uint) -> V;

    /// return the number of shared verties
    fn shared_vertex_count(&self) -> uint;

    /// create an iterator that returns each shared vertex in the generator
    fn shared_vertex_iter<'a>(&'a self) -> ShareVertexIterator<'a, Self, V> {
        ShareVertexIterator {
            base: self,
            idx: range(0, self.shared_vertex_count())
        }
    }
}

pub struct ShareVertexIterator<'a, T, V> {
    base: &'a T,
    idx: Range<uint>
}

impl<'a, T: SharedVertex<V>, V> Iterator<V> for ShareVertexIterator<'a, T, V> {
    fn next(&mut self) -> Option<V> {
        self.idx.next().map(|idx| self.base.shared_vertex(idx))
    }
}

pub trait IndexedPolygon<V> {
    /// return a polygon with indicies to the shared vertex
    fn indexed_polygon(&self, idx: uint) -> V;

    /// return the number of polygons in the generator
    fn indexed_polygon_count(&self) -> uint;

    /// return the number of 
    fn indexed_polygon_iter<'a>(&'a self) -> IndexedPolygonIterator<'a, Self, V> {
        IndexedPolygonIterator {
            base: self,
            idx: range(0, self.indexed_polygon_count())
        }
    }
}

pub struct IndexedPolygonIterator<'a, T, V> {
    base: &'a T,
    idx: Range<uint>
}


impl<'a, T: IndexedPolygon<V>, V> Iterator<V> for IndexedPolygonIterator<'a, T, V> {
    fn next(&mut self) -> Option<V> {
        self.idx.next().map(|idx| self.base.indexed_polygon(idx))
    }
}