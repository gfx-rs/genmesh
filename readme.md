# Vertex-RS

[![Build Status](https://travis-ci.org/csherratt/vertex-rs.svg?branch=master)](https://travis-ci.org/csherratt/vertex-rs)

Vertex-rs is a library for building vertex pipelines. The goal is help facilitate polygon assembly. This is done by building on top of the of the `Iterator` trait. A pipeline stage is a lazy iterator the consumes the input, and produces a new polygon based on its output.

This also provides some `Generators` for creating primitives at runtime.

**Currently supported stages**
 - `vertex` maps a function to each vertex in a polygon
 - `polygon` maps a function to each polygon
 - `to_triangles` triangles Quads to Triangles
 - `vertices` terminates the pipeline producing a vertex iterator

**Currently Supported Generators**
 - `Cube`
 - `Plane`

An example:
```rust
    let vertex_data: Vec<MyVertex> = Cube::new()
        .vertex(|vertex::Vector3(pos)| MyVertex::new(pos, [0., 0.]))
        .polygon(|Quad{x: v0, y: v1, z: v2, w: v3}| {
            Quad::new(MyVertex::new(v0.a_Pos, [0., 0.]),
                      MyVertex::new(v1.a_Pos, [1., 0.]),
                      MyVertex::new(v2.a_Pos, [1., 1.]),
                      MyVertex::new(v3.a_Pos, [0., 1.]))
        })
        .to_triangles()
        .vertices()
        .collect();

```

Here `Cube` generates six faces, one per side this is presented as a `Quad<Vertex3<f32>>`.

`vertex` maps a function to each vertex in each face, in this case we want to convert from `vertex-rs`'s internal vertex format to our own. We now have a `Quad<MyVertex>>`.

We can do a polygon level transform and modify the polygon as a whole. In the example we add a valid texture coordinate to each face. the `polygon` transform does not require the vertices to be consistent but the Polygon type must be.

`to_triangles` will convert the `Quad<MyVertex>` to a `Triangle<MyVertex>>`. This will produce two polygons and six vertices. Some of the verticies are cloned in order to complete this operation.

`verticies` now unwraps each triangle and returns the vertices in-order. This will obviously produce 3 results for each polygon.

`collect` is a standard Iterator operation.
