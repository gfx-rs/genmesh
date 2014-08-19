use super::Quad;

pub trait QuadPipeline<'a, T> : Iterator<Quad<T>> {
    fn vertex<'a,U>(self, f: |T|:'a -> U) -> QuadVertexMap<'a, Self, T, U> {
        QuadVertexMap {
            source: self,
            f: f
        }
    }

    fn polygon<'a, U>(self, f: |Quad<T>|:'a -> Quad<U>) -> QuadPolyMap<'a, Self, T, U> {
        QuadPolyMap {
            source: self,
            f: f
        }
    }
}

pub struct QuadVertexMap<'a, SRC, T, U> {
    source: SRC,
    f: |T|:'a -> U
}

impl<'a, SRC: QuadPipeline<'a, T>, T: Clone, U> Iterator<Quad<U>> for QuadVertexMap<'a, SRC, T, U> {
    fn next(&mut self) -> Option<Quad<U>> {
        self.source.next().map(|quad| {
            quad.map_vertex(|v| (self.f)(v))
        })
    }
}

impl<'a, SRC: QuadPipeline<'a, T>, T: Clone, U> QuadPipeline<'a, U> for QuadVertexMap<'a, SRC, T, U> {}

pub struct QuadPolyMap<'a, SRC, T, U> {
    source: SRC,
    f: |Quad<T>|:'a -> Quad<U>
}

impl<'a, SRC: QuadPipeline<'a, T>, T, U> Iterator<Quad<U>> for QuadPolyMap<'a, SRC, T, U> {
    fn next(&mut self) -> Option<Quad<U>> {
        self.source.next().map(|q| (self.f)(q))
    }
}

impl<'a, SRC: QuadPipeline<'a, T>, T, U> QuadPipeline<'a, U> for QuadPolyMap<'a, SRC, T, U> {}


pub struct QuadGenerator<SRC> {
    source: SRC
}

impl<T, SRC: Iterator<Quad<T>>> QuadGenerator<SRC> {
    pub fn new(src: SRC) -> QuadGenerator<SRC> {
        QuadGenerator {
            source: src
        }
    }
}

impl<'a, T, SRC: Iterator<Quad<T>>> Iterator<Quad<T>> for QuadGenerator<SRC> {
    fn next(&mut self) -> Option<Quad<T>> {
        self.source.next()
    }
} 

impl<'a, T, SRC: Iterator<Quad<T>>> QuadPipeline<'a, T> for QuadGenerator<SRC> {}