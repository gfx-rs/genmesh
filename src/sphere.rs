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

use std::f32::consts::{PI, PI_2};
use super::{Quad, Triangle, Polygon};
use super::{PolyTri, PolyQuad};

pub struct SphereUV {
    u: uint,
    v: uint,
    sub_u: uint,
    sub_v: uint
}

impl SphereUV {
    pub fn new(u: uint, v: uint) -> SphereUV {
        SphereUV {
            u: 0,
            v: 0,
            sub_u: u,
            sub_v: v / 2
        }
    }

    fn vert(&self, u: uint, v: uint) -> (f32, f32, f32) {
        let u = (u as f32 / self.sub_u as f32) * PI_2;
        let v = (v as f32 / self.sub_v as f32) * PI;

        (u.cos() * v.sin(),
         u.sin() * v.sin(),
         v.cos())
    }
}

impl Iterator<Polygon<(f32, f32, f32)>> for SphereUV {
    fn next(&mut self) -> Option<Polygon<(f32, f32, f32)>> {
        if self.v == self.sub_v {
            self.v = 0;
            self.u += 1;
            if self.u == self.sub_u {
                return None;
            }
        }

        let x = self.vert(self.u,   self.v);
        let y = self.vert(self.u,   self.v+1);
        let z = self.vert(self.u+1, self.v+1);
        let w = self.vert(self.u+1, self.v);
        let v = self.v;
        self.v += 1;

        if v == 0 {
            Some(PolyTri(Triangle::new(x, y, z)))
        } else if v == self.sub_v - 1 {
            Some(PolyTri(Triangle::new(z, w, x)))
        } else {
            Some(PolyQuad(Quad::new(x, y, z, w)))
        }
    }
}