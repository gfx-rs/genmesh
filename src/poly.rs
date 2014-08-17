use core::slice::Items;

pub struct Vector1<T>(pub [T, ..1]);
pub struct Vector2<T>(pub [T, ..2]);
pub struct Vector3<T>(pub [T, ..3]);
pub struct Vector4<T>(pub [T, ..4]);

impl<T> FromIterator<T> for Vector1<T> {
    fn from_iter<I: Iterator<T>>(iter: I) -> Vector1<T> {
        let mut iter = iter;
        match iter.next() {
            Some(a) => Vector1([a]),
            _ => fail!("should have found 2 vertices to buld a Vector1")
        }
    }
}

impl<T: Clone> Clone for Vector1<T> {
    fn clone(&self) -> Vector1<T> {
        let &Vector1(ref v) = self;
        Vector1([v[0].clone()])
    }
}

impl<T> Poly<T> for Vector1<T> {
    fn as_slice<'a>(&'a self) -> &'a [T] {
        let &Vector1(ref s) = self;
        s.as_slice()
    }
}

impl<T> FromIterator<T> for Vector2<T> {
    fn from_iter<I: Iterator<T>>(iter: I) -> Vector2<T> {
        let mut iter = iter;
        match (iter.next(), iter.next()) {
            (Some(a), Some(b)) => Vector2([a, b]),
            _ => fail!("should have found 2 vertices to buld a Vector2")
        }
    }
}

impl<T: Clone> Clone for Vector2<T> {
    fn clone(&self) -> Vector2<T> {
        let &Vector2(ref v) = self;
        Vector2([v[0].clone(),
                 v[1].clone()])
    }
}

impl<T> Poly<T> for Vector2<T> {
    fn as_slice<'a>(&'a self) -> &'a [T] {
        let &Vector2(ref s) = self;
        s.as_slice()
    }
}

impl<T> FromIterator<T> for Vector3<T> {
    fn from_iter<I: Iterator<T>>(iter: I) -> Vector3<T> {
        let mut iter = iter;
        match (iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), Some(c)) => Vector3([a, b, c]),
            _ => fail!("should have found 3 vertices to buld a Vector3")
        }
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

impl<T> Poly<T> for Vector3<T> {
    fn as_slice<'a>(&'a self) -> &'a [T] {
        let &Vector3(ref s) = self;
        s.as_slice()
    }
}

impl<T> FromIterator<T> for Vector4<T> {
    fn from_iter<I: Iterator<T>>(iter: I) -> Vector4<T> {
        let mut iter = iter;
        match (iter.next(), iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), Some(c), Some(d)) => Vector4([a, b, c, d]),
            _ => fail!("should have found 4 vertices to buld a quad")
        }
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

impl<T> Poly<T> for Vector4<T> {
    fn as_slice<'a>(&'a self) -> &'a [T] {
        let &Vector4(ref s) = self;
        s.as_slice()
    }
}

pub trait Poly<T> : FromIterator<T> {
    fn as_slice<'a>(&'a self) -> &'a [T];

    fn iter<'a>(&'a self) -> Items<T> {
        self.as_slice().iter()
    }
}

pub struct Triangle<T>(Vector3<T>);

impl<T> Triangle<T> {
    pub fn new(v0: T, v1: T, v2: T) -> Triangle<T> {
        Triangle(Vector3([v0, v1, v2]))
    }
}

impl<T: Clone> Clone for Triangle<T> {
    fn clone(&self) -> Triangle<T> {
        let &Triangle(ref v) = self;
        Triangle(v.clone()) 
    }
}

impl<T> FromIterator<T> for Triangle<T> {
    fn from_iter<I: Iterator<T>>(iter: I) -> Triangle<T> {
        let mut iter = iter;
        match (iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), Some(c)) => Triangle(Vector3([a, b, c])),
            _ => fail!("should have found 3 vertices to buld a triangle")
        }
    }
}

impl<T> Poly<T> for Triangle<T> {
    fn as_slice<'a>(&'a self) -> &'a [T] {
        let &Triangle(Vector3(ref s)) = self;
        s.as_slice()
    }
}

pub struct Quad<T>(pub Vector4<T>);

impl<T> Quad<T> {
    pub fn new(v0: T, v1: T, v2: T, v3: T) -> Quad<T> {
        Quad(Vector4([v0, v1, v2, v3]))
    }
}

pub trait ToTriangle<T> {
    fn to_triangles(&self, |T|);
}

impl<T: Clone> ToTriangle<Triangle<T>> for Quad<T> {
    fn to_triangles(&self, emit: |Triangle<T>|) {
        let &Quad(Vector4([ref v0, ref v1, ref v2, ref v3])) = self;
        emit(Triangle::new(v0.clone(), v1.clone(), v2.clone()));
        emit(Triangle::new(v2.clone(), v3.clone(), v0.clone()));
    }
}

impl<T> FromIterator<T> for Quad<T> {
    fn from_iter<I: Iterator<T>>(iter: I) -> Quad<T> {
        let mut iter = iter;
        match (iter.next(), iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), Some(c), Some(d)) => Quad(Vector4([a, b, c, d])),
            _ => fail!("should have found 4 vertices to buld a quad")
        }
    }
}

impl<T> Poly<T> for Quad<T> {
    fn as_slice<'a>(&'a self) -> &'a [T] {
        let &Quad(Vector4(ref s)) = self;
        s.as_slice()
    }
}

pub enum Polygon<T> {
    PolyTri(Triangle<T>),
    PolyQuad(Quad<T>)
}

impl<T> Poly<T> for Polygon<T> {
    fn as_slice<'a>(&'a self) -> &'a [T] {
        match self {
            &PolyTri(Triangle(ref s)) => s.as_slice(),
            &PolyQuad(Quad(ref s)) => s.as_slice()
        }
    }
}

impl<T> FromIterator<T> for Polygon<T> {
    fn from_iter<I: Iterator<T>>(iter: I) -> Polygon<T> {
        let mut iter = iter;
        match (iter.next(), iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), Some(c), None) => PolyTri(Triangle(Vector3([a, b, c]))),
            (Some(a), Some(b), Some(c), Some(d)) => PolyQuad(Quad(Vector4([a, b, c, d]))),
            _ => fail!("should have found 4 vertices to buld a quad")
        }
    }
}