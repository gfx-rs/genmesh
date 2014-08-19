pub struct Vector1<T>(pub [T, ..1]);
pub struct Vector2<T>(pub [T, ..2]);
pub struct Vector3<T>(pub [T, ..3]);
pub struct Vector4<T>(pub [T, ..4]);

impl<T: Clone> Clone for Vector1<T> {
    fn clone(&self) -> Vector1<T> {
        let &Vector1(ref v) = self;
        Vector1([v[0].clone()])
    }
}

impl<T: Clone> Clone for Vector2<T> {
    fn clone(&self) -> Vector2<T> {
        let &Vector2(ref v) = self;
        Vector2([v[0].clone(),
                 v[1].clone()])
    }
}

impl<T: Clone> Clone for Vector3<T> {
    fn clone(&self) -> Vector3<T> {
        let &Vector3(ref v) = self;
        Vector3([v[0].clone(),
                 v[1].clone(),
                 v[2].clone()])
    }
}

impl<T: Clone> Clone for Vector4<T> {
    fn clone(&self) -> Vector4<T> {
        let &Vector4(ref v) = self;
        Vector4([v[0].clone(),
                 v[1].clone(),
                 v[2].clone(),
                 v[3].clone()])
    }
}

#[deriving(Clone, Show, PartialEq, Eq)]
pub struct Quad<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

impl<T> Quad<T> {
    pub fn new(v0: T, v1: T, v2: T, v3: T) -> Quad<T> {
        Quad {
            x: v0,
            y: v1,
            z: v2,
            w: v3
        }
    }
}

impl<T: Clone> Quad<T> {
    pub fn map_vertex<U>(&self, f: |T| -> U) -> Quad<U> {
        Quad::new(f(self.x.clone()),
                  f(self.y.clone()),
                  f(self.z.clone()),
                  f(self.w.clone()))
    }
}

#[deriving(Clone, Show, PartialEq, Eq)]
pub struct Triangle<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Triangle<T> {
    pub fn new(v0: T, v1: T, v2: T) -> Triangle<T> {
        Triangle {
            x: v0,
            y: v1,
            z: v2
        }
    }
}

impl<T: Clone> Triangle<T> {
    pub fn map_vertex<U>(&self, f: |T| -> U) -> Triangle<U> {
        Triangle::new(f(self.x.clone()),
                      f(self.y.clone()),
                      f(self.z.clone()))
    }
}

#[deriving(Clone, PartialEq)]
pub enum Polygon<T> {
    PolyTri(Triangle<T>),
    PolyQuad(Quad<T>)
}

impl<T: Clone> Polygon<T> {
    pub fn map_vertex<U>(&self, f: |T| -> U) -> Polygon<U> {
        match self {
            &PolyTri(ref t) => PolyTri(t.map_vertex(f)),
            &PolyQuad(ref q) => PolyQuad(q.map_vertex(f))
        }
    }
}

pub trait ToTriangles<T> {
    fn to_triangles(&self, emit: |Triangle<T>|);
}

impl<T: Clone> ToTriangles<T> for Quad<T> {
    fn to_triangles(&self, emit: |Triangle<T>|) {
        let &Quad{x: ref x, y: ref y, z: ref z, w: ref w} = self;
        emit(Triangle::new(x.clone(), y.clone(), z.clone()));
        emit(Triangle::new(z.clone(), w.clone(), x.clone()));
    }
}

impl<T: Clone> ToTriangles<T> for Triangle<T> {
    fn to_triangles(&self, emit: |Triangle<T>|) {
        emit(self.clone());
    }
}

impl<T: Clone> ToTriangles<T> for Polygon<T> {
    fn to_triangles(&self, emit: |Triangle<T>|) {
        match self {
            &PolyTri(ref t) => t.to_triangles(emit),
            &PolyQuad(ref q) => q.to_triangles(emit),
        }
    }
}