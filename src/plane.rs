
use super::{QuadPipeline, Quad, Vector2};

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

    fn vert(&self, x: uint, y: uint) -> Vector2<f32> {
        let sx = self.subdivide_x as f32;
        let sy = self.subdivide_y as f32;
        let x = (2. / sx) * x as f32 - 1.;
        let y = (2. / sy) * y as f32 - 1.; 
        Vector2([x, y])
    }
} 

impl Iterator<Quad<Vector2<f32>>> for Plane {
    fn next(&mut self) -> Option<Quad<Vector2<f32>>> {
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

impl<'a> QuadPipeline<'a, Vector2<f32>> for Plane {}
