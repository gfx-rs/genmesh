
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
    Poly,
    ToTriangle,
    Polygon,
    PolyTri,
    PolyQuad,
};

mod cube;
mod poly;

pub mod generators {
    pub use cube::Cube;
}


trait VertexPass<IN: Clone, OUT> {
    fn vertex_pass(&self, IN) -> OUT;

    fn transform_primative<A: Poly<IN>, B: Poly<OUT>>(&self, a: &A) -> B {
        a.iter().map(|v| self.vertex_pass(v)).collect()
    }
}


pub struct IndexDeref<'a, T> {
    vertices: &'a [T]
}

impl<'a, T> IndexDeref<'a, T> {
    pub fn new(vertices: &'a [T]) -> IndexDeref<'a, T> {
        IndexDeref {
            vertices: vertices
        }
    }
}

impl<'a, T: Clone> VertexPass<uint, T> for IndexDeref<'a, T> {
    fn vertex_pass(&self, index: uint) -> T {
        self.vertices[index].clone()
    }
}

pub trait Generator<IN_V, P: Poly<IN_V>> : Iterator<P> {
    fn vertices(self) -> VertexGenerator<Self, IN_V> {
        VertexGenerator {
            source: self,
            spare: Vec::new()
        }
    }

    fn geometry<'a, OUT_P>(self, max: uint, f: |P, emit: |OUT_P||:'a) -> GeometryMap<'a, Self, P, OUT_P> {
        GeometryMap {
            source: self,
            max: max,
            buffer: Some(RingBuf::with_capacity(max)),
            f: f
        }
    }

    fn to_triangles<OUT_P>(self) -> ConvertTriangles<Self, OUT_P> {
        ConvertTriangles {
            source: self,
            buffer: Some(RingBuf::with_capacity(2)),
        }
    }

    fn vertex_map<'a, OUT_V>(self, f: |IN_V|:'a -> OUT_V) -> VertexMap<'a, Self, IN_V, OUT_V> {
        VertexMap {
            source: self,
            f: f
        }
    }
}

impl<'a, IN_V, IN_P: Poly<IN_V>,
         OUT_V, OUT_P: Poly<OUT_V>,
         SRC: Iterator<IN_P>> Generator<OUT_V, OUT_P> for core::iter::Map<'a, IN_P, OUT_P, SRC> {}

impl<OUT_V, OUT_P: Poly<OUT_V>> Generator<OUT_V, OUT_P> for std::vec::MoveItems<OUT_P> {}

pub struct GeometryMap<'a, SRC, IN_P, OUT_P> {
    source: SRC,
    max: uint,
    buffer: Option<RingBuf<OUT_P>>,
    f: |IN_P, |OUT_P||:'a
}

impl<'a, IN_P, OUT_P, SRC: Iterator<IN_P>> Iterator<OUT_P> for GeometryMap<'a, SRC, IN_P, OUT_P> {
    fn next(&mut self) -> Option<OUT_P> {
        loop {
            match self.buffer.get_mut_ref().pop_front() {
                p @ Some(_) => return p,
                None => ()
            }

            match self.source.next() {
                Some(p) => {
                    let mut buf = self.buffer.take().unwrap();
                    (self.f)(p, |emited| buf.push(emited));
                    self.buffer = Some(buf);
                }
                None => return None
            }

        }
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        match self.source.size_hint() {
            (base, Some(s)) => {
                (base, Some(s*self.max))
            },
            (base, None) => {
                (base, Some(base*self.max))
            }
        }
    }
}

pub struct ConvertTriangles<SRC, OUT_P> {
    source: SRC,
    buffer: Option<RingBuf<OUT_P>>,
}

impl<IN_V, IN_P: ToTriangle<OUT_P>, OUT_P: Poly<IN_V>, SRC: Iterator<IN_P>> Iterator<OUT_P> for ConvertTriangles<SRC, OUT_P> {
    fn next(&mut self) -> Option<OUT_P> {
        loop {
            match self.buffer.get_mut_ref().pop_front() {
                p @ Some(_) => return p,
                None => ()
            }

            match self.source.next() {
                Some(p) => {
                    let mut buf = self.buffer.take().unwrap();
                    p.to_triangles(|emited| buf.push(emited));
                    self.buffer = Some(buf);
                }
                None => return None
            }

        }
    }
}

impl<OUT_V, IN_P: ToTriangle<OUT_P>,
     OUT_P: Poly<OUT_V>, SRC: Iterator<IN_P>> Generator<OUT_V, OUT_P> for ConvertTriangles<SRC, OUT_P> {}

impl<'a, IN_P, OUT_V,
         OUT_P: Poly<OUT_V>,
         SRC: Iterator<IN_P>> Generator<OUT_V, OUT_P> for GeometryMap<'a, SRC, IN_P, OUT_P> {}

pub struct VertexGenerator<SRC, T> {
    source: SRC,
    spare: Vec<T>
}

impl<T: Clone, P: Poly<T>, SRC: Iterator<P>> Iterator<T> for VertexGenerator<SRC, T> {
    fn next(&mut self) -> Option<T> {
        loop {
            match self.spare.remove(0) {
                out @ Some(_) => return out,
                None => ()
            }

            match self.source.next() {
                Some(s) => {
                    self.spare.push_all(s.as_slice());
                }
                None => return None
            }
        }
    }
}

pub struct VertexMap<'a, SRC, IN_V, OUT_V> {
    source: SRC,
    f: |IN_V|:'a  -> OUT_V
}

impl<'a, OUT_P: Poly<OUT_V>, IN_P: Poly<IN_V>, SRC: Iterator<IN_P>, IN_V: Clone, OUT_V> Iterator<OUT_P> for VertexMap<'a, SRC, IN_V, OUT_V> {
    fn next(&mut self) -> Option<OUT_P> {
        let next = self.source.next();

        next.map(|p| {
            let out: OUT_P = p.iter().map(|v| (self.f)(v)).collect();
            out
        })
    }
}

impl<'a, OUT_P: Poly<OUT_V>, IN_P: Poly<IN_V>, SRC: Iterator<IN_P>, IN_V: Clone, OUT_V> Generator<OUT_V, OUT_P> for VertexMap<'a, SRC, IN_V, OUT_V> {}

#[cfg(test)]
mod test {
    use IndexDeref;
    use VertexPass;
    use Triangle;
    use Poly;
    use PolyQuad;
    use Quad;
    use Generator;

    #[test]
    fn index_deref() {
        let index = &[0, 1, 2, 3];
        let index = IndexDeref::new(index);

        assert_eq!(index.vertex_pass(0), 0u);
        assert_eq!(index.vertex_pass(1), 1u);
        assert_eq!(index.vertex_pass(2), 2u);
        assert_eq!(index.vertex_pass(3), 3u);
    }

    #[test]
    fn index_geo_deref() {
        let a = &[7u, 8, 9, 10];
        let index = IndexDeref::new(a);

        let out: Triangle<uint> = index.transform_primative(&Triangle::new(0u, 1, 2));
        assert_eq!(out.as_slice(), Triangle::new(7u, 8, 9).as_slice())
    }

    #[test]
    fn polyquad_test() {
        let poly = &[PolyQuad(Quad::new(0u8, 0, 0, 0))];
        let _ = poly.to_vec().move_iter().vertex_map(|x| x as u32);
    }
}