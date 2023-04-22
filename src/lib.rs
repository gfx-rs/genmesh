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

#![deny(missing_docs)]
#![allow(clippy::many_single_char_names)]

pub use poly::{
    EmitLines, EmitVertices, Line, Lines, LinesIterator, MapToVertices, MapToVerticesIter,
    MapVertex, Polygon, Quad, Triangle, Vertices, VerticesIterator,
};

pub use triangulate::{EmitTriangles, Triangulate, TriangulateIterator};

pub use indexer::{Indexer, LruIndexer};

pub use neighbors::Neighbors;

mod math;

mod generator;
mod indexer;
mod neighbors;
mod poly;
mod triangulate;

mod circle;
mod cone;
mod cube;
mod cylinder;
mod icosphere;
mod plane;
mod sphere;
mod torus;

/// A collection of utilties that can be used to build
/// meshes programmatically.
pub mod generators {
    pub use super::circle::Circle;
    pub use super::cone::Cone;
    pub use super::cube::Cube;
    pub use super::cylinder::Cylinder;
    pub use super::generator::{
        IndexedPolygon, IndexedPolygonIterator, SharedVertex, SharedVertexIterator,
    };
    pub use super::icosphere::IcoSphere;
    pub use super::plane::Plane;
    pub use super::sphere::SphereUv;
    pub use super::torus::Torus;
}

/// Common vertex position type.
pub type Position = mint::Vector3<f32>;
/// Common vertex normal type.
pub type Normal = mint::Vector3<f32>;
/// Common vertex type.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    /// Vertex position
    pub pos: Position,
    /// Vertex normal
    pub normal: Normal,
}

/// Provides generator into bevy mesh functionality.
#[cfg(feature = "into_bevy_mesh")]
pub mod bevy_compat;
