
extern crate core;
extern crate debug;

use std::collections::RingBuf;
use std::collections::Deque;

pub use poly::{
    Vector1,
    Vector2,
    Vector3,
    Vector4,
    Quad,
    Triangle,
    Polygon,
    PolyTri,
    PolyQuad,
    ToTriangles
};

pub use quad::{
    QuadVertexMap,
    QuadPolyMap,
    QuadPipeline,
    QuadGenerator
};

pub use triangle::{
    TriangleVertexMap,
    TrianglePolyMap,
    TrianglePipeline,
    TriangleGenerator
};


mod cube;
mod poly;
mod quad;
mod triangle;

pub mod generators {
    pub use cube::Cube;
}
