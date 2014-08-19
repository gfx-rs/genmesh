use Triangle;

pub trait TrianglePipeline<'a, T> : Iterator<Triangle<T>> {
    fn vertex<'a,U>(self, f: |T|:'a -> U) -> TriangleVertexMap<'a, Self, T, U> {
        TriangleVertexMap {
            source: self,
            f: f
        }
    }

    fn polygon<'a, U>(self, f: |Triangle<T>|:'a -> Triangle<U>) -> TrianglePolyMap<'a, Self, T, U> {
        TrianglePolyMap {
            source: self,
            f: f
        }
    }
}

pub struct TriangleVertexMap<'a, SRC, T, U> {
    source: SRC,
    f: |T|:'a -> U
}

impl<'a, SRC: TrianglePipeline<'a, T>, T: Clone, U> Iterator<Triangle<U>> for TriangleVertexMap<'a, SRC, T, U> {
    fn next(&mut self) -> Option<Triangle<U>> {
        self.source.next().map(|quad| {
            quad.map_vertex(|v| (self.f)(v))
        })
    }
}

impl<'a, SRC: TrianglePipeline<'a, T>, T: Clone, U> TrianglePipeline<'a, U> for TriangleVertexMap<'a, SRC, T, U> {}

pub struct TrianglePolyMap<'a, SRC, T, U> {
    source: SRC,
    f: |Triangle<T>|:'a -> Triangle<U>
}

impl<'a, SRC: TrianglePipeline<'a, T>, T, U> Iterator<Triangle<U>> for TrianglePolyMap<'a, SRC, T, U> {
    fn next(&mut self) -> Option<Triangle<U>> {
        self.source.next().map(|q| (self.f)(q))
    }
}

impl<'a, SRC: TrianglePipeline<'a, T>, T, U> TrianglePipeline<'a, U> for TrianglePolyMap<'a, SRC, T, U> {}

pub struct TriangleGenerator<SRC> {
    source: SRC
}

impl<T, SRC: Iterator<Triangle<T>>> TriangleGenerator<SRC> {
    pub fn new(src: SRC) -> TriangleGenerator<SRC> {
        TriangleGenerator {
            source: src
        }
    }
}

impl<'a, T, SRC: Iterator<Triangle<T>>> Iterator<Triangle<T>> for TriangleGenerator<SRC> {
    fn next(&mut self) -> Option<Triangle<T>> {
        self.source.next()
    }
} 

impl<'a, T, SRC: Iterator<Triangle<T>>> TrianglePipeline<'a, T> for TriangleGenerator<SRC> {}
