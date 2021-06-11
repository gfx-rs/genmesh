//! This is a utility to search out and work in the mesh as a whole rather
//! then polygon by polygon.

use std::collections::{HashMap, HashSet};

use crate::poly::{EmitLines, Line, Triangle};
use crate::{math::Vector3, Normal};

/// Neighbors search accelerating structure.
pub struct Neighbors<T> {
    /// Mesh vertices.
    pub vertices: Vec<T>,
    /// Mesh polygons.
    pub polygons: Vec<Triangle<usize>>,
    shares_edge: HashMap<Line<usize>, Vec<usize>>,
    shares_vertex: HashMap<usize, Vec<usize>>,
}

impl<T> Neighbors<T> {
    /// Builds a Neighbors search based on the supplied vertices
    /// and supplied triangle list.
    pub fn new(vertices: Vec<T>, polygons: Vec<Triangle<usize>>) -> Self {
        let mut shares_edge = HashMap::new();
        let mut shares_vertex = HashMap::new();

        for (i, p) in polygons.iter().enumerate() {
            p.clone().emit_lines(|line| {
                shares_vertex.entry(line.x).or_insert_with(Vec::new).push(i);
                shares_vertex.entry(line.y).or_insert_with(Vec::new).push(i);
                shares_edge.entry(line).or_insert_with(Vec::new).push(i);
            });
        }

        Neighbors {
            vertices,
            polygons,
            shares_edge,
            shares_vertex,
        }
    }

    /// Returns the vector and triangle list used to create the Neighbors.
    pub fn split(self) -> (Vec<T>, Vec<Triangle<usize>>) {
        (self.vertices, self.polygons)
    }

    /// Looks up the index of every polygon that contains
    /// vertex `t`, this can be used to calculate new faces.
    pub fn vertex_neighbors(&self, t: &usize) -> Option<&[usize]> {
        self.shares_vertex.get(t).map(|x| &x[..])
    }

    /// Looks up the index of every [`Polygon`] that is a neighbor of the
    /// [`Polygon`] at index `i`. This can be used to prep data for a Geometry
    /// shader (eg triangle_adjacency).
    ///
    /// [`Polygon`]: enum.Polygon.html
    pub fn polygon_neighbors(&self, i: usize) -> Option<HashSet<usize>> {
        self.polygons.get(i).map(|x| {
            let mut v = HashSet::new();
            x.clone().emit_lines(|line| {
                if let Some(x) = self.shares_edge.get(&line) {
                    for &i in x {
                        v.insert(i);
                    }
                }
            });
            v.remove(&i);
            v
        })
    }

    /// Calculate the normal for a face. This is a `flat` shading.
    ///
    /// You must supply a function that can be used to lookup
    /// the position which is needed to calculate the normal.
    pub fn normal_for_face<F>(&self, i: usize, mut f: F) -> Normal
    where
        F: FnMut(&T) -> Normal,
    {
        let Triangle { x, y, z } = self.polygons[i];

        let x = Vector3::from(f(&self.vertices[x]));
        let y = Vector3::from(f(&self.vertices[y]));
        let z = Vector3::from(f(&self.vertices[z]));

        let a = z - x;
        let b = z - y;

        a.cross(b).normalized().into()
    }

    /// Calculate the normal for a vertex based on the average
    /// of its neighbors. This is a `smooth` shading.
    ///
    /// You must supply a function that can be used to lookup
    /// the position which is needed to calculate the normal.
    pub fn normal_for_vertex<F>(&self, i: usize, mut f: F) -> Normal
    where
        F: FnMut(&T) -> Normal,
    {
        let mut normal = Vector3::new(0f32, 0., 0.);

        for &face in &self.shares_vertex[&i] {
            normal += Vector3::from(self.normal_for_face(face, &mut f));
        }

        normal.normalized().into()
    }
}
