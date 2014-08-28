//   Copyright Colin Sherratt 2014
//   
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//   
//       http://www.apache.org/licenses/LICENSE-2.0
//   
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

pub trait Indexer<T> {
    /// convert a vertex into an index.
    fn index(&mut self, v: T) -> uint;
}

/// An `LruIndexer` is useful for creating indexed steam from a stream of
/// vertices. Each vertex that is index is only compared against the vetices
/// contained in the cache. If a vertex is not found the LruIndexer will `emit`
/// a new vertex and return the index of that new vertex.
///
/// The oldest sample by time used will be dropped if a new vertex is found.
pub struct LruIndexer<'a, T> {
    index: uint,
    max: uint,
    cache: Vec<(T, uint)>,
    emit: |uint, T|:'a
}

impl<'a, T> LruIndexer<'a, T> {
    /// create a new `LruIndexer`, the window size is limited by the `size` parameter
    /// it is recommended to keep this small since lookup is done in N time
    ///
    /// if a new vertex is found, `emit` will be called. emit will be supplied with a
    /// vertex and a index that was used.
    pub fn new<'a>(size: uint, emit: |uint, T|:'a) -> LruIndexer<'a, T> {
        LruIndexer {
            index: 0,
            max: size,
            cache: Vec::new(),
            emit: emit
        }
    }
}

impl<'a, T: PartialEq + Clone> Indexer<T> for LruIndexer<'a, T> {
    fn index(&mut self, new: T) -> uint {
        let mut found = None;
        for (i, &(ref v, idx)) in self.cache.iter().enumerate() {
            if v == &new {
                found = Some((idx, i));
                break;
            }
        }

        match found {
            Some((index, i)) => {
                let item = self.cache.remove(i).unwrap();
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
