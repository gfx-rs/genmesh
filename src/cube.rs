//   Copyright GFX Developers 2014-2017
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

use std::ops::Range;
use {Normal, Position, Vertex};
use super::{MapVertex, Quad};
use super::generators::{SharedVertex, IndexedPolygon};

/// A perfect cube, centered at (0, 0, 0) with each face starting at 1/-1 away from the origin
#[derive(Clone)]
pub struct Cube {
    range: Range<usize>,
}

impl Cube {
    /// create a new cube generator
    pub fn new() -> Self {
        Cube { range: 0..6 }
    }

    fn vert(&self, idx: usize) -> Position {
        let x = if idx & 4 == 4 { 1. } else { -1. };
        let y = if idx & 2 == 2 { 1. } else { -1. };
        let z = if idx & 1 == 1 { 1. } else { -1. };
        [x, y, z]
    }

    fn face_indexed(&self, idx: usize) -> (Normal, Quad<usize>) {
        match idx {
            0 => ([1., 0., 0.], Quad::new(0b110, 0b111, 0b101, 0b100)),
            1 => ([-1., 0., 0.], Quad::new(0b000, 0b001, 0b011, 0b010)),
            2 => ([0., 1., 0.], Quad::new(0b011, 0b111, 0b110, 0b010)),
            3 => ([0., -1., 0.], Quad::new(0b100, 0b101, 0b001, 0b000)),
            4 => ([0., 0., 1.], Quad::new(0b101, 0b111, 0b011, 0b001)),
            5 => ([0., 0., -1.], Quad::new(0b000, 0b010, 0b110, 0b100)),
            idx => panic!("{} face is higher then 6", idx),
        }
    }

    fn face(&self, idx: usize) -> Quad<Vertex> {
        let (no, quad) = self.face_indexed(idx);
        quad.map_vertex(|i| {
                            Vertex {
                                pos: self.vert(i),
                                normal: no,
                            }
                        })
    }
}

impl Iterator for Cube {
    type Item = Quad<Vertex>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }

    fn next(&mut self) -> Option<Quad<Vertex>> {
        self.range.next().map(|idx| self.face(idx))
    }
}

impl SharedVertex<Vertex> for Cube {
    fn shared_vertex(&self, idx: usize) -> Vertex {
        let (no, quad) = self.face_indexed(idx / 4);
        let vid = match idx % 4 {
            0 => quad.x,
            1 => quad.y,
            2 => quad.z,
            3 => quad.w,
            _ => unreachable!(),
        };
        Vertex {
            pos: self.vert(vid),
            normal: no,
        }
    }

    fn shared_vertex_count(&self) -> usize {
        24
    }
}

impl IndexedPolygon<Quad<usize>> for Cube {
    fn indexed_polygon(&self, idx: usize) -> Quad<usize> {
        Quad::new(idx * 4 + 0, idx * 4 + 1, idx * 4 + 2, idx * 4 + 3)
    }

    fn indexed_polygon_count(&self) -> usize {
        6
    }
}
