use super::generators::{IndexedPolygon, SharedVertex};
use super::Polygon::{PolyQuad, PolyTri};
use super::{Polygon, Quad, Triangle};
use std::f32::consts::PI;
use Vertex;

/// Represents a sphere with radius of 1, centered at (0, 0, 0)
#[derive(Clone, Copy)]
pub struct SphereUv {
    u: usize,
    v: usize,
    sub_u: usize,
    sub_v: usize,
}

impl SphereUv {
    /// Create a new sphere.
    /// `u` is the number of points across the equator of the sphere.
    /// `v` is the number of points from pole to pole.
    pub fn new(u: usize, v: usize) -> Self {
        assert!(u > 1 && v > 1);
        SphereUv {
            u: 0,
            v: 0,
            sub_u: u,
            sub_v: v,
        }
    }

    fn vert(&self, u: usize, v: usize) -> Vertex {
        let u = (u as f32 / self.sub_u as f32) * PI * 2.;
        let v = (v as f32 / self.sub_v as f32) * PI;

        let p = [u.cos() * v.sin(), u.sin() * v.sin(), v.cos()];
        Vertex {
            pos: p.into(),
            normal: p.into(),
        }
    }
}

impl Iterator for SphereUv {
    type Item = Polygon<Vertex>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.u == self.sub_u {
            self.u = 0;
            self.v += 1;
            if self.v == self.sub_v {
                return None;
            }
        }

        // mathematically, reaching `u + 1 == sub_u` should trivially resolve,
        // because sin(2pi) == sin(0), but rounding errors go in the way.
        let u1 = (self.u + 1) % self.sub_u;

        let x = self.vert(self.u, self.v);
        let y = self.vert(self.u, self.v + 1);
        let z = self.vert(u1, self.v + 1);
        let w = self.vert(u1, self.v);
        let v = self.v;
        self.u += 1;

        Some(if v == 0 {
            PolyTri(Triangle::new(x, y, z))
        } else if v == self.sub_v - 1 {
            // overriding z to force u == 0 for consistency
            let z = self.vert(0, self.sub_v);
            PolyTri(Triangle::new(z, w, x))
        } else {
            PolyQuad(Quad::new(x, y, z, w))
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl ExactSizeIterator for SphereUv {
    fn len(&self) -> usize {
        (self.sub_v - self.v - 1) * self.sub_u + (self.sub_u - self.u)
    }
}

impl SharedVertex<Vertex> for SphereUv {
    fn shared_vertex(&self, idx: usize) -> Vertex {
        if idx == 0 {
            self.vert(0, 0)
        } else if idx == self.shared_vertex_count() - 1 {
            self.vert(0, self.sub_v)
        } else {
            // since the bottom verts all map to the same
            // we jump over them in index space
            let idx = idx - 1;
            let u = idx % self.sub_u;
            let v = idx / self.sub_u;
            self.vert(u, v + 1)
        }
    }

    fn shared_vertex_count(&self) -> usize {
        (self.sub_v - 1) * (self.sub_u) + 2
    }
}

impl IndexedPolygon<Polygon<usize>> for SphereUv {
    fn indexed_polygon(&self, idx: usize) -> Polygon<usize> {
        let f = |u: usize, v: usize| {
            if v == 0 {
                0
            } else if self.sub_v == v {
                (self.sub_v - 1) * (self.sub_u) + 1
            } else {
                (v - 1) * self.sub_u + (u % self.sub_u) + 1
            }
        };

        let u = idx % self.sub_u;
        let v = idx / self.sub_u;

        if v == 0 {
            PolyTri(Triangle::new(f(u, v), f(u, v + 1), f(u + 1, v + 1)))
        } else if self.sub_v - 1 == v {
            PolyTri(Triangle::new(f(u + 1, v + 1), f(u + 1, v), f(u, v)))
        } else {
            PolyQuad(Quad::new(
                f(u, v),
                f(u, v + 1),
                f(u + 1, v + 1),
                f(u + 1, v),
            ))
        }
    }

    fn indexed_polygon_count(&self) -> usize {
        self.sub_v * self.sub_u
    }
}

#[test]
fn test_sphere_len() {
    let mut sphere = SphereUv::new(5, 5);
    assert_eq!(25, sphere.len());
    sphere.next();
    assert_eq!(24, sphere.len());
    assert_eq!(24, sphere.count());
}
