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

use std::f32::consts::PI;
use super::{Quad, Triangle, Polygon};
use super::Polygon::{PolyTri, PolyQuad};
use super::generators::{SharedVertex, IndexedPolygon};

/// Represents a sphere with radius of 1, centered at (0, 0, 0)
#[derive(Clone, Copy)]
pub struct SphereUV {
    u: usize,
    v: usize,
    sub_u: usize,
    sub_v: usize
}

impl SphereUV {
    /// Create a new sphere.
    /// `u` is the number of points across the equator of the sphere.
    /// `v` is the number of points from pole to pole.
    pub fn new(u: usize, v: usize) -> SphereUV {
        SphereUV {
            u: 0,
            v: 0,
            sub_u: u,
            sub_v: v
        }
    }

    fn vert(&self, u: usize, v: usize) -> (f32, f32, f32) {
        let u = (u as f32 / self.sub_u as f32) * PI * 2.;
        let v = (v as f32 / self.sub_v as f32) * PI;

        (u.cos() * v.sin(),
         u.sin() * v.sin(),
         v.cos())
    }
}

impl Iterator for SphereUV {
    type Item = Polygon<(f32, f32, f32)>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = (self.sub_v - self.v) * self.sub_u + (self.sub_u - self.u);
        (n, Some(n))
    }


    fn next(&mut self) -> Option<Polygon<(f32, f32, f32)>> {
        if self.u == self.sub_u {
            self.u = 0;
            self.v += 1;
            if self.v == self.sub_v {
                return None;
            }
        }

        let x = self.vert(self.u,   self.v);
        let y = self.vert(self.u,   self.v+1);
        let z = self.vert(self.u+1, self.v+1);
        let w = self.vert(self.u+1, self.v);
        let v = self.v;
        self.u += 1;

        if v == 0 {
            Some(PolyTri(Triangle::new(x, y, z)))
        } else if v == self.sub_v - 1 {
            Some(PolyTri(Triangle::new(z, w, x)))
        } else {
            Some(PolyQuad(Quad::new(x, y, z, w)))
        }
    }
}

impl SharedVertex<(f32, f32, f32)> for SphereUV {
    fn shared_vertex(&self, idx: usize) -> (f32, f32, f32) {
        if idx == 0 {
            self.vert(0, 0)
        } else if idx == self.shared_vertex_count() - 1 {
            self.vert(0, self.sub_v)
        } else {
            // since the bottom verts all map to the same
            // we jump over them in index space
            let idx = idx - 1;
            let u = idx % (self.sub_u);
            let v = idx / (self.sub_u);
            self.vert(u, v+1)
        }
    }

    fn shared_vertex_count(&self) -> usize {
        (self.sub_v - 1) * (self.sub_u) + 2
    }
}

impl IndexedPolygon<Polygon<usize>> for SphereUV {
    fn indexed_polygon(&self, idx: usize) -> Polygon<usize> {
        let u = idx % self.sub_u;
        let v = idx / self.sub_u;

        let f = |u: usize, v: usize| {
            if v == 0 {
                0
            } else if self.sub_v == v {
                (self.sub_v-1) * (self.sub_u) + 1
            } else {
                (v-1) * self.sub_u + (u % self.sub_u) + 1
            }
        };

        if v == 0 {
            PolyTri(Triangle::new(f(u,   v),
                                  f(u,   v+1),
                                  f(u+1, v+1)))
        } else if self.sub_v - 1 == v {
            PolyTri(Triangle::new(f(u+1, v+1),
                                  f(u+1, v),
                                  f(u,   v)))
        } else {
            PolyQuad(Quad::new(f(u,   v),
                               f(u,   v+1),
                               f(u+1, v+1),
                               f(u+1, v)))
        }
    }

    fn indexed_polygon_count(&self) -> usize {
        self.sub_v * self.sub_u
    }
}

