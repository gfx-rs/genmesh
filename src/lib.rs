
extern crate core;
extern crate debug;

pub use poly::{
    Quad,
    Triangle,
    Polygon,
    PolyTri,
    PolyQuad,
    Vertices,
    VerticesIterator,
    MapToVertices
};

pub use triangulate::{
    EmitTriangles,
    Triangulate,
    TriangulateIterator
};

mod triangulate;
mod poly;

mod cube;
mod plane;

pub mod generators {
    pub use cube::Cube;
    pub use plane::Plane;
}
