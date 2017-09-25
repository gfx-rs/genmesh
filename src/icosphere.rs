//! Icosahedral sphere

#![allow(missing_docs)]

use std::collections::HashMap;

use cgmath::{InnerSpace, Vector3};

use Vertex;
use super::Triangle;
use super::generators::{IndexedPolygon, SharedVertex};

/// Icosahedral sphere, no support for subdivision at this point.
#[derive(Clone, Debug)]
pub struct IcoSphere {
    i: usize,
    vertices: Vec<[f32; 3]>,
    faces: Vec<[usize; 3]>,
}

const T: f32 = 1.61803398874989484820;

const VERTICES: [[f32; 3]; 12] = [
    [-1., T, 0.],
    [1., T, 0.],
    [-1., -T, 0.],
    [1., -T, 0.],
    [0., -1., T],
    [0., 1., T],
    [0., -1., -T],
    [0., 1., -T],
    [T, 0., -1.],
    [T, 0., 1.],
    [-T, 0., -1.],
    [-T, 0., 1.],
];

const FACES: [[usize; 3]; 20] = [
    // 5 faces around point 0
    [0, 11, 5],
    [0, 5, 1],
    [0, 1, 7],
    [0, 7, 10],
    [0, 10, 11],
    // 5 faces adjacent to the faces around point 0
    [1, 5, 9],
    [5, 11, 4],
    [11, 10, 2],
    [10, 7, 6],
    [7, 1, 8],
    // 5 faces around point 3
    [3, 9, 4],
    [3, 4, 2],
    [3, 2, 6],
    [3, 6, 8],
    [3, 8, 9],
    // 5 faces adjacent to the faces around point 3
    [4, 9, 5],
    [2, 4, 11],
    [6, 2, 10],
    [8, 6, 7],
    [9, 8, 1],
];

fn base_vertices() -> Vec<[f32; 3]> {
    VERTICES
        .iter()
        .map(|p| Vector3::from(*p).normalize().into())
        .collect::<Vec<[f32; 3]>>()
}

impl IcoSphere {
    pub fn new() -> Self {
        Self {
            i: 0,
            vertices: base_vertices(),
            faces: FACES.to_vec(),
        }
    }

    pub fn subdivide(subdivides: usize) -> Self {
        let mut vertices = base_vertices();
        let mut faces = FACES.to_vec();

        for _ in 0..subdivides {
            let (v, f) = subdivide_impl(vertices, faces);
            vertices = v;
            faces = f;
        }

        Self {
            i: 0,
            vertices,
            faces,
        }
    }

    pub fn vert(&self, index: usize) -> Vertex {
        Vertex {
            pos: self.vertices[index],
            normal: self.vertices[index],
        }
    }
}

fn subdivide_impl(
    mut vertices: Vec<[f32; 3]>,
    faces: Vec<[usize; 3]>,
) -> (Vec<[f32; 3]>, Vec<[usize; 3]>) {
    let mut lookup = HashMap::<(usize, usize), usize>::default();
    let mut new_faces = Vec::<[usize; 3]>::default();
    for face in &faces {
        let mut mid: [usize; 3] = [0; 3];
        for i in 0..3 {
            let pair = (face[i], face[(i + 1) % 3]);
            // find new vertex on the edge
            mid[i] = match lookup.get(&pair) {
                Some(i) => *i,
                None => vertices.len(),
            };
            if mid[i] == vertices.len() {
                lookup.insert(pair, mid[i]);
                lookup.insert((face[(i + 1) % 3], face[i]), mid[i]);
                let new = new_point(vertices[pair.0], vertices[pair.1]);
                vertices.push(new);
            }
        }
        new_faces.push([face[0], mid[0], mid[2]]);
        new_faces.push([face[1], mid[1], mid[0]]);
        new_faces.push([face[2], mid[2], mid[1]]);
        new_faces.push([mid[0], mid[1], mid[2]]);
    }
    (vertices, new_faces)
}

fn new_point(start: [f32; 3], end: [f32; 3]) -> [f32; 3] {
    Vector3::new(start[0] + end[0], start[1] + end[1], start[2] + end[2])
        .normalize()
        .into()
}

impl Iterator for IcoSphere {
    type Item = Triangle<Vertex>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.faces.len(), Some(self.faces.len()))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.faces.len() {
            return None;
        }

        let face = self.faces[self.i];
        let x = self.vert(face[0]);
        let y = self.vert(face[1]);
        let z = self.vert(face[2]);
        self.i += 1;

        Some(Triangle::new(x, y, z))
    }
}

impl SharedVertex<Vertex> for IcoSphere {
    fn shared_vertex_count(&self) -> usize {
        self.vertices.len()
    }

    fn shared_vertex(&self, idx: usize) -> Vertex {
        self.vert(idx)
    }
}

impl IndexedPolygon<Triangle<usize>> for IcoSphere {
    fn indexed_polygon_count(&self) -> usize {
        self.faces.len()
    }

    fn indexed_polygon(&self, idx: usize) -> Triangle<usize> {
        Triangle::new(self.faces[idx][0], self.faces[idx][1], self.faces[idx][2])
    }
}
