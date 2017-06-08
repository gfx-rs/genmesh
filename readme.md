# Genmesh
[![Build Status](https://travis-ci.org/gfx-rs/genmesh.svg?branch=master)](https://travis-ci.org/gfx-rs/genmesh)
[![Docs](https://docs.rs/genmesh/badge.svg)](https://docs.rs/genmesh)
[![Crates.io](https://img.shields.io/crates/v/genmesh.svg?maxAge=2592000)](https://crates.io/crates/genmesh)

`genmesh` is a library for building vertex pipelines. The goal is help facilitate polygon assembly. This is done by building on top of the of the `Iterator` trait. A pipeline stage is a lazy iterator the consumes the input, and produces a new polygon based on the stage.

This also provides some `generators` for creating primitives at runtime.

**Currently supported stages**
 - `vertex` maps a function to each vertex in a polygon
 - `triangulate` triangles Quads to Triangles
 - `vertices` turns a poly pipeline into a vertices pipeline

**Primitive generators**
 - `Cone`
 - `Cube`
 - `Cylinder`
 - `Plane`
 - `SphereUV`
 - `Torus`

 **Vertex attributes**
 - `pos`: position
 - `normal`: normal

**Utility**
 - `LruIndexer` translate a vertex into a index, emitting a new vertex if
 the current vertex is not in the `Lru` cache.
 - `Neighbors` work with mesh as a whole by querying normals and neighbors
on either vertex or polygon levels.

**Primitives**
 - `Triangle`
 - `Quad`
 - `Polygon` an enum of both `Triangle` and `Quad`

## Example

```rust
    let vertex_data: Vec<MyVertex> = Cube::new()
        .vertex(|v| MyVertex::new(v.pos, [0., 0.]))
        .map(|Quad{x: v0, y: v1, z: v2, w: v3}| {
            Quad::new(MyVertex::new(v0.a_Pos, [0., 0.]),
                      MyVertex::new(v1.a_Pos, [1., 0.]),
                      MyVertex::new(v2.a_Pos, [1., 1.]),
                      MyVertex::new(v3.a_Pos, [0., 1.]))
        })
        .triangulate()
        .vertices()
        .collect();

```

Here `Cube` generates six faces, one per side this is presented as a `Quad<Vertex3<f32>>`.

`vertex` maps a function to each vertex in each face, in this case we want to convert from `genmes`'s internal vertex format to our own. We now have a `Quad<MyVertex>>`.

We can do a polygon level transform and modify the polygon as a whole. In the example we add a valid texture coordinate to each face. Since genmesh is just an extension of iterators we can do a polygon level transform using just `map`.

`triangulate` will convert the `Quad<MyVertex>` to a `Triangle<MyVertex>>`. This will produce two polygons and six vertices. Some of the verticies are cloned in order to complete this operation.

`verticies` now unwraps each triangle and returns the vertices in-order. This will obviously produce 3 results for each polygon.

`collect` is a standard Iterator operation.
