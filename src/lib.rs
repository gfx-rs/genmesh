
extern crate core;
extern crate debug;

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

pub use polygon::{
    PolygonVertexMap,
    PolygonPolyMap,
    PolygonPipeline,
    PolygonGenerator
};

mod cube;
mod poly;
mod quad;
mod triangle;
mod polygon;

pub mod generators {
    pub use cube::Cube;
}
