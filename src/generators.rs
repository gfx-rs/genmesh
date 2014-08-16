
use core::iter::Range;
use super::{Generator, Quad};

pub struct Cube {
    range: Range<uint>
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            range: range(0, 6)
        }
    }

    pub fn vert(&self, idx: uint) -> [f32, ..3] {
        let x = if idx & 1 == 1 { 1.} else { -1. };
        let y = if idx & 2 == 2 { 1.} else { -1. };
        let z = if idx & 4 == 4 { 1.} else { -1. };
        [x, y, z]
    }

    pub fn face(&self, idx: uint) -> Quad<[f32, ..3]> {
        match idx {
            0 => Quad::new(self.vert(0), self.vert(1), self.vert(2), self.vert(3)),
            1 => Quad::new(self.vert(7), self.vert(6), self.vert(5), self.vert(4)),
            2 => Quad::new(self.vert(0), self.vert(2), self.vert(4), self.vert(6)),
            3 => Quad::new(self.vert(7), self.vert(5), self.vert(3), self.vert(1)),
            5 => Quad::new(self.vert(0), self.vert(4), self.vert(1), self.vert(5)),
            6 => Quad::new(self.vert(7), self.vert(3), self.vert(6), self.vert(1)),
            idx => fail!("invalid face {}", idx)
        }
    }
} 

impl Iterator<Quad<[f32, ..3]>> for Cube {
    fn next(&mut self) -> Option<Quad<[f32, ..3]>> {
        self.range.next().map(|idx| self.face(idx))
    }
}

impl Generator<Quad<[f32, ..3]>> for Cube {}