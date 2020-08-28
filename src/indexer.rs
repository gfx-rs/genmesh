/// A trait defining how to defined an Indexer. An indexer is a object
/// that collects verticies and emits indexes for the vertex. The intent
/// is that an Indexer can find redundent vertexes and deduplicate them
/// by returning aliased indexes.
pub trait Indexer<T> {
    /// convert a vertex into an index.
    fn index(&mut self, v: T) -> usize;
}

/// An `LruIndexer` is useful for creating indexed steam from a stream of
/// vertices. Each vertex that is index is only compared against the vetices
/// contained in the cache. If a vertex is not found the LruIndexer will `emit`
/// a new vertex and return the index of that new vertex.
///
/// The oldest sample by time used will be dropped if a new vertex is found.
pub struct LruIndexer<T, F: FnMut(usize, T)> {
    index: usize,
    max: usize,
    cache: Vec<(T, usize)>,
    emit: F,
}

impl<T, F: FnMut(usize, T)> LruIndexer<T, F> {
    /// create a new `LruIndexer`, the window size is limited by the `size` parameter
    /// it is recommended to keep this small since lookup is done in N time
    ///
    /// if a new vertex is found, `emit` will be called. emit will be supplied with a
    /// vertex and a index that was used.
    pub fn new(size: usize, emit: F) -> LruIndexer<T, F> {
        LruIndexer {
            index: 0,
            max: size,
            cache: Vec::new(),
            emit,
        }
    }
}

impl<T: PartialEq + Clone, F: FnMut(usize, T)> Indexer<T> for LruIndexer<T, F> {
    fn index(&mut self, new: T) -> usize {
        let mut found = None;
        for (i, &(ref v, idx)) in self.cache.iter().enumerate() {
            if v == &new {
                found = Some((idx, i));
                break;
            }
        }

        match found {
            Some((index, i)) => {
                let item = self.cache.remove(i);
                self.cache.push(item);
                index
            }
            None => {
                if self.cache.len() >= self.max {
                    self.cache.remove(0);
                }
                let index = self.index;
                self.index += 1;
                self.cache.push((new.clone(), index));
                (self.emit)(index, new);
                index
            }
        }
    }
}
