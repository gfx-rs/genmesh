# Vertex-RS

[![Build Status](https://travis-ci.org/csherratt/vertex-rs.svg?branch=master)](https://travis-ci.org/csherratt/vertex-rs)

Vertex-rs is a library for building vertex pipelines. The goal is help facilitate polygon assembly. This is done by building on top of the of the `Iterator` trait.

An example:
```rust
    let vertex_data: Vec<Vertex> = Cube::new().polygon(|Quad{x: v0, y: v1, z: v2, w: v3}| {
        Quad::new(Vertex::new(match v0 {vertex::Vector3([x, y, z]) => [x, y, z]}, [0., 0.]),
                  Vertex::new(match v1 {vertex::Vector3([x, y, z]) => [x, y, z]}, [1., 0.]),
                  Vertex::new(match v2 {vertex::Vector3([x, y, z]) => [x, y, z]}, [1., 1.]),
                  Vertex::new(match v3 {vertex::Vector3([x, y, z]) => [x, y, z]}, [0., 1.]))
    }).to_triangles().vertices().collect();

```

`Cube` is a generator, 