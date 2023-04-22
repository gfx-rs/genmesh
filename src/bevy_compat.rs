use glam::Vec3;
use bevy_render::{mesh::{Mesh, Indices}, render_resource::PrimitiveTopology};
use crate::{
    generators::{IndexedPolygon, SharedVertex,
    Circle,
    Cone,
    Cube,
    Cylinder,
    IcoSphere,
    Plane,
    SphereUv,
    Torus,
    },
    EmitTriangles, Triangulate, Vertex, Vertices,
};

impl From<Cube> for Mesh {
    fn from(generator: Cube) -> Self {
        build_mesh(generator)
    }
}

impl From<Circle> for Mesh {
    fn from(generator: Circle) -> Self {
        build_mesh(generator)
    }
}
impl From<Cone> for Mesh {
    fn from(generator: Cone) -> Self {
        build_mesh(generator)
    }
}

impl From<Cylinder> for Mesh {
    fn from(generator: Cylinder) -> Self {
        build_mesh(generator)
    }
}

impl From<IcoSphere> for Mesh {
    fn from(generator: IcoSphere) -> Self {
        build_mesh(generator)
    }
}

impl From<Plane> for Mesh {
    fn from(generator: Plane) -> Self {
        build_mesh(generator)
    }
}

impl From<SphereUv> for Mesh {
    fn from(generator: SphereUv) -> Self {
        build_mesh(generator)
    }
}

impl From<Torus> for Mesh {
    fn from(generator: Torus) -> Self {
        build_mesh(generator)
    }
}

/// Build a bevy mesh from a generator.
pub fn build_mesh<T, P>(generator: T) -> Mesh
where
    T: SharedVertex<Vertex> + IndexedPolygon<P>,
    P: EmitTriangles<Vertex = usize>,
{
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let (vertices, normals): (Vec<Vec3>, Vec<Vec3>) = generator
        .shared_vertex_iter()
        .map(|v| {
            let w: (Vec3, Vec3) = (v.pos.into(), v.normal.into());
            w
        })
        .unzip();
    let indices: Vec<u32> = generator
        .indexed_polygon_iter()
        .triangulate()
        .vertices()
        .map(|u| u as u32)
        .collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}
