//   Copyright Dzmitry Malyshau 2017
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

use std::f32::consts::PI;
use super::{Quad, Polygon, Triangle};
use super::generators::{SharedVertex, IndexedPolygon};

/// Represents a cylinder with radius of 1, height of 2,
/// and centered at (0, 0, 0) pointing up (to 0, 0, 1).
#[derive(Clone, Copy)]
pub struct Cylinder {
    u: usize,
    h: isize,
    sub_u: usize,
}

impl Cylinder {
    /// Create a new cylinder.
    /// `u` is the number of points across the radius.
    pub fn new(u: usize) -> Self {
        assert!(u > 1);
        Cylinder {
            u: 0,
            h: -1,
            sub_u: u,
        }
    }

    fn vert(&self, u: usize, h: isize) -> (f32, f32, f32) {
        let a = (u as f32 / self.sub_u as f32) * PI * 2.;
        (a.cos(), a.sin(), h as f32)
    }
}

impl Iterator for Cylinder {
    type Item = Polygon<(f32, f32, f32)>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.sub_u * (2 - self.h) as usize - self.u;
        (n, Some(n))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.u == self.sub_u {
            if self.h == 1 {
                return None;
            }
            self.u = 0;
            self.h += 1;
        }

        let u = self.u;
        self.u += 1;
        // mathematically, reaching `u + 1 == sub_u` should trivially resolve,
        // because sin(2pi) == sin(0), but rounding errors go in the way.
        let u1 = self.u % self.sub_u;

        let x = self.vert(u, -1);
        let y = self.vert(u1, -1);
        let z = self.vert(u1, 1);
        let w = self.vert(u, 1);

        Some(match self.h {
            -1 => Polygon::PolyTri(Triangle::new(x, (0., 0., -1.), y)),
            0  => Polygon::PolyQuad(Quad::new(x, y, z, w)),
            1  => Polygon::PolyTri(Triangle::new(w, z, (0., 0., 1.))),
            _ => unreachable!()
        })
    }
}

impl SharedVertex<(f32, f32, f32)> for Cylinder {
    fn shared_vertex(&self, idx: usize) -> (f32, f32, f32) {
        if idx == 0 {
            (0., 0., -1.)
        } else if idx == self.shared_vertex_count() - 1 {
            (0., 0., 1.)
        } else {
            // skip the bottom center
            let idx = idx - 1;
            let u = idx % self.sub_u;
            let h = (idx / self.sub_u) as isize * 2 - 1;
            self.vert(u, h)
        }
    }

    fn shared_vertex_count(&self) -> usize {
        2 * self.sub_u + 2
    }
}

impl IndexedPolygon<Polygon<usize>> for Cylinder {
    fn indexed_polygon(&self, idx: usize) -> Polygon<usize> {
        let u = idx % self.sub_u;
        let u1 = (idx + 1) % self.sub_u;
        match idx / self.sub_u {
            0 => {
                let base = 1;
                let start = 0;
                Polygon::PolyTri(Triangle::new(base + u, start, base + u1))
            },
            1 => {
                let base = 1;
                Polygon::PolyQuad(Quad::new(base + u,
                                            base + u1,
                                            base + u1 + self.sub_u,
                                            base + u + self.sub_u))
            },
            2 => {
                let base = 1 + self.sub_u;
                let end = self.shared_vertex_count() - 1;
                Polygon::PolyTri(Triangle::new(base + u, base + u1, end))
            },
            _ => unreachable!()
        }
    }

    fn indexed_polygon_count(&self) -> usize {
        3 * self.sub_u
    }
}
