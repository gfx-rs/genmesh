use std::f32::consts::PI;

use cgmath::{Vector3, InnerSpace};

use super::{Quad, Vertex, MapVertex};
use super::generators::{SharedVertex, IndexedPolygon};

///
#[derive(Clone, Copy)]
pub struct Torus {
    idx: usize,
    radius: f32,
    tubular_radius: f32,
    radial_segments: usize,
    tubular_segments: usize,
}

impl Torus {
    /// Create a new Torus Generator.
    /// `radius` is the radius from the center [0, 0, 0] to the center
    ///          of the the tubular radius
    /// `tubular_radius` is the radius to the surface from the toridal
    /// `tubular_segments` the number of segments that wrap around the tube, must be at least 3
    /// `radial_segments` the number of tube segments requested to generate, must be at least 3
    pub fn new(radius: f32,
               tubular_radius: f32,
               radial_segments: usize,
               tubular_segments: usize)
               -> Self {
        assert!(tubular_segments > 2 && radial_segments > 2);
        Torus {
            idx: 0,
            radius: radius,
            tubular_radius: tubular_radius,
            radial_segments: radial_segments,
            tubular_segments: tubular_segments,
        }
    }
}

impl Iterator for Torus {
    type Item = Quad<Vertex>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.indexed_polygon_count() {
            let idx = self.idx;
            self.idx += 1;
            Some(self.indexed_polygon(idx)
                     .map_vertex(|i| self.shared_vertex(i)))
        } else {
            None
        }
    }
}

impl SharedVertex<Vertex> for Torus {
    fn shared_vertex(&self, idx: usize) -> Vertex {
        let (h, u) = ((idx / self.tubular_segments) as f32, (idx % self.tubular_segments) as f32);
        let alpha = u * 2. * PI / self.tubular_segments as f32;
        let beta = h * 2. * PI / self.radial_segments as f32;
        let gamma = self.radius + self.tubular_radius * alpha.cos();

        Vertex {
            pos: [gamma * beta.cos(),
                  self.tubular_radius * alpha.sin(),
                  -gamma * beta.sin()],
            normal: Vector3::new(alpha.cos() * beta.cos(),
                                 alpha.sin(),
                                 -alpha.cos() * beta.sin())
                    .normalize()
                    .into(),
        }
    }

    fn shared_vertex_count(&self) -> usize {
        self.tubular_segments * self.radial_segments + 1
    }
}

impl IndexedPolygon<Quad<usize>> for Torus {
    fn indexed_polygon(&self, idx: usize) -> Quad<usize> {
        // check for wrap around the end end
        let ncol = if self.indexed_polygon_count() - idx > self.tubular_segments {
            self.tubular_segments as isize
        } else {
            -((self.indexed_polygon_count() - self.tubular_segments) as isize)
        };

        // check for wrap around the end end
        let nrow = if idx % self.tubular_segments != self.tubular_segments - 1 {
            1isize
        } else {
            1isize - (self.tubular_segments as isize)
        };

        let idx = idx as isize;
        Quad::new(idx, idx + ncol, idx + nrow + ncol, idx + nrow).map_vertex(|x| x as usize)
    }

    fn indexed_polygon_count(&self) -> usize {
        self.tubular_segments * self.radial_segments
    }
}
