//   Copyright Colin Sherratt 2014
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

#![feature(test)]

extern crate genmesh;
extern crate test;

use genmesh::*;
use genmesh::generators::{Plane, SphereUV};
use genmesh::generators::{SharedVertex, IndexedPolygon};
use test::{Bencher, black_box};

#[bench]
fn plane(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = Plane::new();
        for i in plane.shared_vertex_iter() {
            black_box(i);
        }
        for i in plane.indexed_polygon_iter() {
            black_box(i);
        }
    });
}

#[bench]
fn plane_16x16_index(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = Plane::subdivide(16, 16);
        for i in plane.indexed_polygon_iter() {
            black_box(i);
        }
    });
}

#[bench]
fn plane_256x256_index(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = Plane::subdivide(256, 256);
        for i in plane.indexed_polygon_iter() {
            black_box(i);
        }
    });
}

#[bench]
fn plane_16x16_vertex(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = Plane::subdivide(16, 16);
        for i in plane.shared_vertex_iter() {
            black_box(i);
        }
    });
}

#[bench]
fn plane_256x256_vertex(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = Plane::subdivide(256, 256);
        for i in plane.shared_vertex_iter() {
            black_box(i);
        }
    });
}

#[bench]
fn plane_16x16_index_triangulate(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = Plane::subdivide(16, 16);
        for i in plane.indexed_polygon_iter()
                      .triangulate() {
            black_box(i);
        }
    });
}

#[bench]
fn plane_256x256_index_triangulate(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = Plane::subdivide(256, 256);
        for i in plane.indexed_polygon_iter()
                      .triangulate() {
            black_box(i);
        }
    });
}

#[bench]
fn sphere_16x16_index(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = SphereUV::new(16, 16);
        for i in plane.indexed_polygon_iter() {
            black_box(i);
        }
    });
}

#[bench]
fn sphere_256x256_index(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = SphereUV::new(256, 256);
        for i in plane.indexed_polygon_iter() {
            black_box(i);
        }
    });
}

#[bench]
fn sphere_16x16_vertex(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = SphereUV::new(16, 16);
        for i in plane.shared_vertex_iter() {
            black_box(i);
        }
    });
}

#[bench]
fn sphere_256x256_vertex(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = SphereUV::new(256, 256);
        for i in plane.shared_vertex_iter() {
            black_box(i);
        }
    });
}

#[bench]
fn sphere_16x16_index_triangulate(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = SphereUV::new(16, 16);
        for i in plane.indexed_polygon_iter()
                      .triangulate() {
            black_box(i);
        }
    });
}

#[bench]
fn sphere_256x256_index_triangulate(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = SphereUV::new(256, 256);
        for i in plane.indexed_polygon_iter()
                      .triangulate() {
            black_box(i);
        }
    });
}
