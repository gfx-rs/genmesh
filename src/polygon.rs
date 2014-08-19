use Polygon;

pub trait PolygonPipeline<'a, T> : Iterator<Polygon<T>> {
    fn vertex<'a,U>(self, f: |T|:'a -> U) -> PolygonVertexMap<'a, Self, T, U> {
        PolygonVertexMap {
            source: self,
            f: f
        }
    }

    fn polygon<'a, U>(self, f: |Polygon<T>|:'a -> Polygon<U>) -> PolygonPolyMap<'a, Self, T, U> {
        PolygonPolyMap {
            source: self,
            f: f
        }
    }
}

pub struct PolygonVertexMap<'a, SRC, T, U> {
    source: SRC,
    f: |T|:'a -> U
}

impl<'a, SRC: PolygonPipeline<'a, T>, T: Clone, U> Iterator<Polygon<U>> for PolygonVertexMap<'a, SRC, T, U> {
    fn next(&mut self) -> Option<Polygon<U>> {
        self.source.next().map(|quad| {
            quad.map_vertex(|v| (self.f)(v))
        })
    }
}

impl<'a, SRC: PolygonPipeline<'a, T>, T: Clone, U> PolygonPipeline<'a, U> for PolygonVertexMap<'a, SRC, T, U> {}

pub struct PolygonPolyMap<'a, SRC, T, U> {
    source: SRC,
    f: |Polygon<T>|:'a -> Polygon<U>
}

impl<'a, SRC: PolygonPipeline<'a, T>, T, U> Iterator<Polygon<U>> for PolygonPolyMap<'a, SRC, T, U> {
    fn next(&mut self) -> Option<Polygon<U>> {
        self.source.next().map(|q| (self.f)(q))
    }
}

impl<'a, SRC: PolygonPipeline<'a, T>, T, U> PolygonPipeline<'a, U> for PolygonPolyMap<'a, SRC, T, U> {}

pub struct PolygonGenerator<SRC> {
    source: SRC
}

impl<T, SRC: Iterator<Polygon<T>>> PolygonGenerator<SRC> {
    pub fn new(src: SRC) -> PolygonGenerator<SRC> {
        PolygonGenerator {
            source: src
        }
    }
}

impl<'a, T, SRC: Iterator<Polygon<T>>> Iterator<Polygon<T>> for PolygonGenerator<SRC> {
    fn next(&mut self) -> Option<Polygon<T>> {
        self.source.next()
    }
} 

impl<'a, T, SRC: Iterator<Polygon<T>>> PolygonPipeline<'a, T> for PolygonGenerator<SRC> {}
