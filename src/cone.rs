use std::f32::consts::{self, FRAC_1_SQRT_2};

use super::generators::{IndexedPolygon, SharedVertex};
use super::{MapVertex, Triangle, Vertex};

const TWO_PI: f32 = consts::PI * 2.;

#[derive(Debug)]
enum VertexSection {
    Tip(usize),
    TopRadius(usize),
    BottomRadius(usize),
    BottomCenter,
}

/// The `Cone` mesh will create a mesh that goes from 1 to -1.
/// The bottom will be a circle around [0, 0, -1] with a radius
/// of 1, all coords on the bottom will follow the plane equation `-z-1=0`.
/// The tip of the cone will always be at coord [0, 0, 1].
pub struct Cone {
    u: usize,
    sub_u: usize,
}

impl Cone {
    /// Creates a new cone.
    ///
    /// # Arguments
    ///
    /// - `u` is the number of subdivisions around the radius of the cone,
    ///     it must be at least 2
    ///
    /// # Panics
    ///
    /// This function panics if `u` is less than 2.
    pub fn new(u: usize) -> Self {
        assert!(u >= 2);
        Cone { u: 0, sub_u: u }
    }

    fn vertex(&self, sec: VertexSection) -> Vertex {
        let divisions = TWO_PI / self.sub_u as f32;

        match sec {
            VertexSection::Tip(i) => {
                // the normal is in the middle of the two divisions
                // so we add half a subdivision
                let pos = divisions * i as f32 + divisions / 2.;
                Vertex {
                    pos: [0., 0., 1.].into(),
                    normal: [
                        pos.cos() * FRAC_1_SQRT_2,
                        pos.sin() * FRAC_1_SQRT_2,
                        -FRAC_1_SQRT_2,
                    ]
                    .into(),
                }
            }
            VertexSection::TopRadius(i) => {
                let pos = divisions * i as f32;
                Vertex {
                    pos: [pos.cos(), pos.sin(), -1.].into(),
                    normal: [
                        pos.cos() * FRAC_1_SQRT_2,
                        pos.sin() * FRAC_1_SQRT_2,
                        -FRAC_1_SQRT_2,
                    ]
                    .into(),
                }
            }
            VertexSection::BottomRadius(i) => {
                let pos = divisions * i as f32;
                Vertex {
                    pos: [pos.cos(), pos.sin(), -1.].into(),
                    normal: [0., 0., -1.].into(),
                }
            }
            VertexSection::BottomCenter => Vertex {
                pos: [0., 0., -1.].into(),
                normal: [0., 0., -1.].into(),
            },
        }
    }

    fn index(&self, sec: VertexSection) -> usize {
        match sec {
            VertexSection::Tip(i) => i,
            VertexSection::TopRadius(i) => i + self.sub_u,
            VertexSection::BottomRadius(i) => i + self.sub_u * 2,
            VertexSection::BottomCenter => self.sub_u * 3,
        }
    }

    fn rev_index(&self, idx: usize) -> VertexSection {
        if idx < self.sub_u {
            VertexSection::Tip(idx)
        } else if idx < self.sub_u * 2 {
            VertexSection::TopRadius(idx - self.sub_u)
        } else if idx < self.sub_u * 3 {
            VertexSection::BottomRadius(idx - self.sub_u * 2)
        } else {
            VertexSection::BottomCenter
        }
    }
}

impl Iterator for Cone {
    type Item = Triangle<Vertex>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.u < self.sub_u * 2 {
            let idx = self.u;
            self.u += 1;
            Some(
                self.indexed_polygon(idx)
                    .map_vertex(|i| self.shared_vertex(i)),
            )
        } else {
            None
        }
    }
}

impl ExactSizeIterator for Cone {
    fn len(&self) -> usize {
        self.sub_u * 2 - self.u
    }
}

impl SharedVertex<Vertex> for Cone {
    fn shared_vertex(&self, idx: usize) -> Vertex {
        self.vertex(self.rev_index(idx))
    }

    fn shared_vertex_count(&self) -> usize {
        // a unique vertex for every subdivide at the top
        // a unique vertex for every radius, top
        // a unique vertex for every radius, bottom
        // one for the bottom most vertex
        self.sub_u * 3 + 1
    }
}

impl IndexedPolygon<Triangle<usize>> for Cone {
    fn indexed_polygon(&self, idx: usize) -> Triangle<usize> {
        // top
        if idx < self.sub_u {
            let next = if idx != self.sub_u - 1 { idx + 1 } else { 0 };
            Triangle::new(
                self.index(VertexSection::Tip(idx)),
                self.index(VertexSection::TopRadius(idx)),
                self.index(VertexSection::TopRadius(next)),
            )
        // bottom
        } else {
            let idx = idx - self.sub_u;
            let next = if idx != self.sub_u - 1 { idx + 1 } else { 0 };
            Triangle::new(
                self.index(VertexSection::BottomCenter),
                self.index(VertexSection::BottomRadius(next)),
                self.index(VertexSection::BottomRadius(idx)),
            )
        }
    }

    fn indexed_polygon_count(&self) -> usize {
        // a face for every subdivide on the top, and one for every
        // subdivide around the bottom circle.
        self.sub_u * 2
    }
}

#[test]
fn test_cone_len() {
    let mut cone = Cone::new(5);
    assert_eq!(10, cone.len());
    cone.next();
    assert_eq!(9, cone.len());
    assert_eq!(9, cone.count());
}
