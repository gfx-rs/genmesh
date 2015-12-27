//   Copyright Colin Sherratt 2015
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

//! This is a utility to search out and work in the mesh as a whole rather
//! then polygon by polygon.

use std::collections::{HashMap, HashSet};
use cgmath::{Vector3, EuclideanVector};

use poly::{Triangle, Line, EmitLines};

pub struct Neighbors<T> {
    pub vertices: Vec<T>,
    pub polygons: Vec<Triangle<usize>>,
    shares_edge: HashMap<Line<usize>, Vec<usize>>,
    shares_vertex: HashMap<usize, Vec<usize>>
}

impl<T> Neighbors<T> {

    /// Build a Neighbors search based on the supplied vertices
    /// and supplied triangle list.
    pub fn new(vertices: Vec<T>, polygons: Vec<Triangle<usize>>) -> Neighbors<T> {
        let mut shares_edge = HashMap::new();
        let mut shares_vertex = HashMap::new();

        for (i, p) in polygons.iter().enumerate() {
            p.clone().emit_lines(|line| {
                shares_vertex.entry(line.x.clone())
                    .or_insert(Vec::new())
                    .push(i);
                shares_vertex.entry(line.y.clone())
                    .or_insert(Vec::new())
                    .push(i);
                shares_edge.entry(line)
                    .or_insert(Vec::new())
                    .push(i);
            });
        }

        Neighbors {
            vertices: vertices,
            shares_vertex: shares_vertex,
            shares_edge: shares_edge,
            polygons: polygons,
        }
    }

    /// return the vector and triangle list used to create the Neighbors
    pub fn split(self) -> (Vec<T>, Vec<Triangle<usize>>) {
        (self.vertices, self.polygons)
    }

    /// looks up the index of every polygon that contains
    /// vertex t, this can be used to calculate new faces
    pub fn vertex_neighbors(&self, t: &usize) -> Option<&[usize]> {
        self.shares_vertex.get(t)
            .map(|x| &x[..])
    }

    /// looks up the index of every polygon that is a neighbor of 
    /// polygon at index i. This can be used to prep data for a Geometry
    /// shader (eg trinagle_adjacency)
    pub fn polygon_neighbors(&self, i: usize) -> Option<HashSet<usize>> {
        self.polygons.get(i)
            .map(|x| {
                let mut v = HashSet::new();
                x.clone().emit_lines(|line| {
                    self.shares_edge.get(&line)
                        .map(|x| {
                            for &i in x { v.insert(i); }
                        });
                });
                v.remove(&i);
                v
            })

    }

    /// Calculate the normal for face. This is a `flat` shading
    ///
    /// You must supply a function that can be used to lookup
    /// The position which is needed to calculate the normal
    pub fn normal_for_face<F>(&self, i: usize, mut f: F) -> [f32; 3]
        where F: FnMut(&T) -> [f32; 3]
    {
        let Triangle{x, y, z} = self.polygons[i];

        let x = to_vec3(f(&self.vertices[x]));
        let y = to_vec3(f(&self.vertices[y]));
        let z = to_vec3(f(&self.vertices[z]));

        let a = z - x;
        let b = z - y;

        a.cross(b).normalize().into()
    }

    /// Calculate the normal for an vertex based on the average
    /// of it's Neighbors this is a `smooth` shading
    ///
    /// You must supply a function that can be used to lookup
    /// The position which is needed to calculate the normal
    pub fn normal_for_vertex<F>(&self, i: usize, mut f: F) -> [f32; 3]
        where F: FnMut(&T) -> [f32; 3]
    {
        let mut normal: Vector3<f32> = Vector3::new(0., 0., 0.);

        for i in &self.shares_vertex[&i] {
            normal = normal + to_vec3(self.normal_for_face(*i, &mut f));
        }

        normal.normalize().into()
    }
}

fn to_vec3(x: [f32; 3]) -> Vector3<f32> {
    Vector3::new(x[0], x[1], x[2])
}