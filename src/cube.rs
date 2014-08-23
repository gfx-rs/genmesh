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

use core::iter::Range;
use super::Quad;

pub struct Cube {
    range: Range<uint>
}

impl Cube {
    /// create a new cube generator
    pub fn new() -> Cube {
        Cube { range: range(0, 6) }
    }

    fn vert(&self, idx: uint) -> (f32, f32, f32) {
        let x = if idx & 4 == 4 { 1.} else { -1. };
        let y = if idx & 2 == 2 { 1.} else { -1. };
        let z = if idx & 1 == 1 { 1.} else { -1. };
        (x, y, z)
    }

    fn face(&self, idx: uint) -> Quad<(f32, f32, f32)> {
        match idx {
            0 => Quad::new(self.vert(0b000), self.vert(0b001),
                           self.vert(0b011), self.vert(0b010)),
            1 => Quad::new(self.vert(0b110), self.vert(0b111),
                           self.vert(0b101), self.vert(0b100)),
            2 => Quad::new(self.vert(0b100), self.vert(0b101),
                           self.vert(0b001), self.vert(0b000)),
            3 => Quad::new(self.vert(0b011), self.vert(0b111),
                           self.vert(0b110), self.vert(0b010)),
            4 => Quad::new(self.vert(0b000), self.vert(0b010),
                           self.vert(0b110), self.vert(0b100)),
            5 => Quad::new(self.vert(0b101), self.vert(0b111),
                           self.vert(0b011), self.vert(0b001)),
            idx => fail!("{} face is higher then 6", idx)
        }
    }
} 

impl Iterator<Quad<(f32, f32, f32)>> for Cube {
    fn next(&mut self) -> Option<Quad<(f32, f32, f32)>> {
        self.range.next().map(|idx| self.face(idx))
    }
}