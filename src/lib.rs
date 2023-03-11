//! This library exposes a struct that mimicks the behaviour of the python
//! [defaultdict](https://docs.python.org/3/library/collections.html#collections.defaultdict).
//!
//! This behaviour does require that the type of the value does have the [`Default`] implemented.

mod default_btree;
mod default_hashmap;

pub use default_btree::DefaultBTreeMap;
pub use default_hashmap::DefaultHashMap;
