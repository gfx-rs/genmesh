use crate::generators::{IndexedPolygon, SharedVertex};
use crate::{Normal, Polygon, Position, Quad, Triangle, Vertex};
use std::f32::consts::PI;

/// Represents a cylinder with radius of 1, height of 2,
/// and centered at (0, 0, 0) pointing up (to 0, 0, 1).
#[derive(Clone, Copy)]
pub struct Cylinder {
    u: usize,
    h: isize,
    sub_u: usize,
    sub_h: isize,
}

const TOP: Vertex = Vertex {
    pos: Position {
        x: 0.,
        y: 0.,
        z: 1.,
    },
    normal: Normal {
        x: 0.,
        y: 0.,
        z: 1.,
    },
};

const BOT: Vertex = Vertex {
    pos: Position {
        x: 0.,
        y: 0.,
        z: -1.,
    },
    normal: Normal {
        x: 0.,
        y: 0.,
        z: -1.,
    },
};

impl Cylinder {
    /// Create a new cylinder.
    /// `u` is the number of points across the radius.
    pub fn new(u: usize) -> Self {
        assert!(u > 1);
        Cylinder {
            u: 0,
            h: -1,
            sub_u: u,
            sub_h: 1,
        }
    }

    /// Create a new subdivided cylinder.
    /// `u` is the number of points across the radius.
    /// `h` is the number of segments across the height.
    pub fn subdivide(u: usize, h: usize) -> Self {
        assert!(u > 1 && h > 0);
        Cylinder {
            u: 0,
            h: -1,
            sub_u: u,
            sub_h: h as isize,
        }
    }

    fn vert(&self, u: usize, h: isize) -> Vertex {
        debug_assert!(u <= self.sub_u);
        let a = (u as f32 / self.sub_u as f32) * PI * 2.;
        let n = [a.cos(), a.sin(), 0.];
        let (hc, normal) = if h < 0 {
            debug_assert_eq!(h, -1);
            (0, [0., 0., -1.])
        } else if h > self.sub_h {
            debug_assert_eq!(h, self.sub_h + 1);
            (self.sub_h, [0., 0., 1.])
        } else {
            (h, n)
        };
        let z = (hc as f32 / self.sub_h as f32) * 2. - 1.;
        Vertex {
            pos: [n[0], n[1], z].into(),
            normal: normal.into(),
        }
    }
}

impl Iterator for Cylinder {
    type Item = Polygon<Vertex>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.u == self.sub_u {
            if self.h >= self.sub_h {
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

        Some(if self.h < 0 {
            let x = self.vert(u, self.h);
            let y = self.vert(u1, self.h);
            Polygon::PolyTri(Triangle::new(x, BOT, y))
        } else if self.h == self.sub_h {
            let x = self.vert(u, self.h + 1);
            let y = self.vert(u1, self.h + 1);
            Polygon::PolyTri(Triangle::new(x, y, TOP))
        } else {
            let x = self.vert(u, self.h);
            let y = self.vert(u1, self.h);
            let z = self.vert(u1, self.h + 1);
            let w = self.vert(u, self.h + 1);
            Polygon::PolyQuad(Quad::new(x, y, z, w))
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl ExactSizeIterator for Cylinder {
    fn len(&self) -> usize {
        self.sub_u * (1 + self.sub_h - self.h) as usize - self.u
    }
}

impl SharedVertex<Vertex> for Cylinder {
    fn shared_vertex(&self, idx: usize) -> Vertex {
        if idx == 0 {
            BOT
        } else if idx == self.shared_vertex_count() - 1 {
            TOP
        } else {
            // skip the bottom center
            let idx = idx - 1;
            let u = idx % self.sub_u;
            let h = (idx / self.sub_u) as isize - 1;
            self.vert(u, h)
        }
    }

    fn shared_vertex_count(&self) -> usize {
        (3 + self.sub_h) as usize * self.sub_u + 2
    }
}

impl IndexedPolygon<Polygon<usize>> for Cylinder {
    fn indexed_polygon(&self, idx: usize) -> Polygon<usize> {
        let u = idx % self.sub_u;
        let u1 = (idx + 1) % self.sub_u;
        let h = (idx / self.sub_u) as isize - 1;
        let base = 1 + idx - u;
        if h < 0 {
            let start = 0;
            Polygon::PolyTri(Triangle::new(base + u, start, base + u1))
        } else if h == self.sub_h {
            // We need to to select the next vertex loop over, which
            // has the correct normals.
            let base = base + self.sub_u;
            let end = self.shared_vertex_count() - 1;
            Polygon::PolyTri(Triangle::new(base + u, base + u1, end))
        } else {
            Polygon::PolyQuad(Quad::new(
                base + u,
                base + u1,
                base + u1 + self.sub_u,
                base + u + self.sub_u,
            ))
        }
    }

    fn indexed_polygon_count(&self) -> usize {
        (2 + self.sub_h) as usize * self.sub_u
    }
}

#[test]
fn test_cylinder_len() {
    let mut cylinder = Cylinder::new(5);
    assert_eq!(15, cylinder.len());
    cylinder.next();
    assert_eq!(14, cylinder.len());
    assert_eq!(14, cylinder.count());
}
