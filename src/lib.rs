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

//! `Genmesh`'s is a library that offers ways to generate and manipulate vertex streams.
//! 
//! The core problem that this library solves is to find a nice way to build meshes that
//! does not just result in throwing all the vertices and indices into a `Vec<T>` and 
//! calling it done. While doing so is simple from a library writers point of view, the
//! consumer will often have to translate that buffer to the format that they need before
//! it can be used. This produces needless buffering that can be avoided.
//!
//! `Genmesh`'s solution is to utilize the `Iterator` trait to build a vertex processing
//! pipeline. The `Iterator` trait has a number of useful functions like `zip`, `map` and
//! `collect` that are useful in themselves. `Genmesh` includes a number of traits that
//! can be used with the built in `Iterator` traits to build the meshes that your engine
//! needs.


extern crate core;

pub use poly::{
    Quad,
    Triangle,
    Polygon,
    PolyTri,
    PolyQuad,
    Vertices,
    VerticesIterator,
    MapToVertices,
    MapVertex
};

pub use triangulate::{
    EmitTriangles,
    Triangulate,
    TriangulateIterator
};

pub use indexer::{
    Indexer,
    LruIndexer
};

mod triangulate;
mod poly;
mod indexer;
mod generator;

mod cube;
mod plane;
mod sphere;

/// a collection of utilties that can be used to build 
/// meshes programmatically
pub mod generators {
    pub use generator::{
        SharedVertex,
        IndexedPolygon
    };
    pub use cube::Cube;
    pub use plane::Plane;
    pub use sphere::SphereUV;
}
