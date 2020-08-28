use std::f32::consts::PI;

use crate::generators::{IndexedPolygon, SharedVertex};
use crate::Polygon::{self, PolyTri};
use crate::{Triangle, Vertex};

/// Represents a circle in the XY plane with radius of 1, centered at (0, 0, 0)
#[derive(Clone, Copy)]
pub struct Circle {
    u: usize,
    sub_u: usize,
}

impl Circle {
    /// Create a new sphere.
    /// `u` is the number of points around the circle, must be > 3
    pub fn new(u: usize) -> Self {
        assert!(u > 3);
        Circle { u: 1, sub_u: u }
    }

    fn vert(&self, u: usize) -> Vertex {
        if u == 0 {
            Vertex {
                pos: [0., 0., 0.].into(),
                normal: [0., 0., 1.].into(),
            }
        } else {
            let u = ((u - 1) as f32 / self.sub_u as f32) * PI * 2.;

            let p = [u.cos(), u.sin(), 0.];
            Vertex {
                pos: p.into(),
                normal: [0., 0., 1.].into(),
            }
        }
    }
}

impl Iterator for Circle {
    type Item = Polygon<Vertex>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }

    fn next(&mut self) -> Option<Self::Item> {
        use std::cmp::Ordering;
        match self.u.cmp(&self.sub_u) {
            Ordering::Less => {
                self.u += 1;
                Some(PolyTri(Triangle::new(
                    self.vert(0),
                    self.vert(self.u - 1),
                    self.vert(self.u),
                )))
            }
            Ordering::Equal => {
                self.u += 1;
                Some(PolyTri(Triangle::new(
                    self.vert(0),
                    self.vert(self.u - 1),
                    self.vert(1),
                )))
            }
            Ordering::Greater => None,
        }
    }
}

impl ExactSizeIterator for Circle {
    fn len(&self) -> usize {
        self.sub_u - self.u + 1
    }
}

impl SharedVertex<Vertex> for Circle {
    fn shared_vertex(&self, idx: usize) -> Vertex {
        self.vert(idx)
    }

    fn shared_vertex_count(&self) -> usize {
        self.sub_u + 1
    }
}

impl IndexedPolygon<Polygon<usize>> for Circle {
    fn indexed_polygon(&self, idx: usize) -> Polygon<usize> {
        if idx == self.sub_u - 1 {
            PolyTri(Triangle::new(0, self.sub_u, 1))
        } else {
            PolyTri(Triangle::new(
                0,
                (idx + 1) % (self.sub_u + 1),
                (idx + 2) % (self.sub_u + 1),
            ))
        }
    }

    fn indexed_polygon_count(&self) -> usize {
        self.sub_u
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_circle() {
        let circle = Circle::new(8);
        assert_eq!((8, Some(8)), circle.size_hint());
        assert_eq!(9, circle.shared_vertex_count());
        assert_eq!(8, circle.indexed_polygon_count());
        assert_eq!(
            Some(&Vertex {
                pos: [0.707107, -0.70710653, 0.0].into(),
                normal: [0., 0., 1.].into()
            }),
            circle.shared_vertex_iter().collect::<Vec<_>>().last()
        );
        let polys = circle.indexed_polygon_iter().collect::<Vec<_>>();
        assert_eq!(PolyTri(Triangle { x: 0, y: 1, z: 2 }), polys[0]);
        assert_eq!(Some(&PolyTri(Triangle { x: 0, y: 8, z: 1 })), polys.last());
    }
}
