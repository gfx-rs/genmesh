//   Copyright GFX Developers 2014-2017
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

use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};

extern crate genmesh;

use genmesh::generators::{IndexedPolygon, SharedVertex};
use genmesh::generators::{Plane, SphereUv};
use genmesh::*;

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

fn plane_16x16_index(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = Plane::subdivide(16, 16);
        for i in plane.indexed_polygon_iter() {
            black_box(i);
        }
    });
}

fn plane_256x256_index(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = Plane::subdivide(256, 256);
        for i in plane.indexed_polygon_iter() {
            black_box(i);
        }
    });
}

fn plane_16x16_vertex(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = Plane::subdivide(16, 16);
        for i in plane.shared_vertex_iter() {
            black_box(i);
        }
    });
}

fn plane_256x256_vertex(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = Plane::subdivide(256, 256);
        for i in plane.shared_vertex_iter() {
            black_box(i);
        }
    });
}

fn plane_16x16_index_triangulate(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = Plane::subdivide(16, 16);
        for i in plane.indexed_polygon_iter().triangulate() {
            black_box(i);
        }
    });
}

fn plane_256x256_index_triangulate(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = Plane::subdivide(256, 256);
        for i in plane.indexed_polygon_iter().triangulate() {
            black_box(i);
        }
    });
}

fn sphere_16x16_index(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = SphereUv::new(16, 16);
        for i in plane.indexed_polygon_iter() {
            black_box(i);
        }
    });
}

fn sphere_256x256_index(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = SphereUv::new(256, 256);
        for i in plane.indexed_polygon_iter() {
            black_box(i);
        }
    });
}

fn sphere_16x16_vertex(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = SphereUv::new(16, 16);
        for i in plane.shared_vertex_iter() {
            black_box(i);
        }
    });
}

fn sphere_256x256_vertex(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = SphereUv::new(256, 256);
        for i in plane.shared_vertex_iter() {
            black_box(i);
        }
    });
}

fn sphere_16x16_index_triangulate(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = SphereUv::new(16, 16);
        for i in plane.indexed_polygon_iter().triangulate() {
            black_box(i);
        }
    });
}

fn sphere_256x256_index_triangulate(bench: &mut Bencher) {
    bench.iter(|| {
        let plane = SphereUv::new(256, 256);
        for i in plane.indexed_polygon_iter().triangulate() {
            black_box(i);
        }
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("plane", plane);
    c.bench_function("plane_16x16_index", plane_16x16_index);
    c.bench_function("plane_16x16_vertex", plane_16x16_vertex);
    c.bench_function(
        "plane_16x16_index_triangulate",
        plane_16x16_index_triangulate,
    );
    c.bench_function("plane_256x256_index", plane_256x256_index);
    c.bench_function("plane_256x256_vertex", plane_256x256_vertex);
    c.bench_function(
        "plane_256x256_index_triangulate",
        plane_256x256_index_triangulate,
    );
    c.bench_function("sphere_16x16_index", sphere_16x16_index);
    c.bench_function("sphere_16x16_vertex", sphere_16x16_vertex);
    c.bench_function(
        "sphere_16x16_index_triangulate",
        sphere_16x16_index_triangulate,
    );
    c.bench_function("sphere_256x256_index", sphere_256x256_index);
    c.bench_function("sphere_256x256_vertex", sphere_256x256_vertex);
    c.bench_function(
        "sphere_256x256_index_triangulate",
        sphere_256x256_index_triangulate,
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
