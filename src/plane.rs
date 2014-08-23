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

use super::{Quad};

pub struct Plane {
    subdivide_x: uint,
    subdivide_y: uint,
    x: uint,
    y: uint
}

impl Plane {
    /// create a new cube generator
    pub fn new() -> Plane {
        Plane { 
            subdivide_x: 1,
            subdivide_y: 1,
            x: 0,
            y: 0
        }
    }

    pub fn subdivide(x: uint, y: uint) -> Plane {
        Plane { 
            subdivide_x: x,
            subdivide_y: y,
            x: 0,
            y: 0
        }
    }

    fn vert(&self, x: uint, y: uint) -> (f32, f32) {
        let sx = self.subdivide_x as f32;
        let sy = self.subdivide_y as f32;
        let x = (2. / sx) * x as f32 - 1.;
        let y = (2. / sy) * y as f32 - 1.; 
        (x, y)
    }
} 

impl Iterator<Quad<(f32, f32)>> for Plane {
    fn next(&mut self) -> Option<Quad<(f32, f32)>> {
        if self.x == self.subdivide_x {
            self.x = 0;
            self.y += 1;
            if self.y == self.subdivide_y {
                return None;
            }
        }

        let x = self.vert(self.x,   self.y);
        let y = self.vert(self.x,   self.y+1);
        let z = self.vert(self.x+1, self.y+1);
        let w = self.vert(self.x+1, self.y);
        self.x += 1;

        Some(Quad::new(x, y, z, w))
    }
}
