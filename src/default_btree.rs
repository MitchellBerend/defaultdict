#![deny(missing_docs)]

use std::collections::{BTreeMap, btree_map::{Keys, Values}};
use std::default::Default;
use std::hash::Hash;


/// This struct mimicks the behaviour of a python defaultdict. This means alongside the traitbounds
/// that apply on the key and value that are inherited from the [`BTreeMap`], it also requires the
/// [`Default`] trait be implemented on the value type.
#[derive(Debug, PartialEq, Eq)]
pub struct DefaultBTreeMap<K, V>
where
    K: Eq + Hash + Ord,
    V: Default,
{
    _inner: BTreeMap<K, V>,
}


impl<K, V> DefaultBTreeMap<K, V>
where
    K: Eq + Hash + Ord,
    V: Default,
{
    /// Creates an empty [`DefaultBTreeMap`].
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let map = DefaultBTreeMap::<i8, i8>::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            _inner: BTreeMap::new()
        }
    }


    /// Inserts a key value pair into the map.
    ///
    /// If the map had the key already present it will be overwritten.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    /// map.insert(10, 20)
    /// ```
    #[inline]
    pub fn insert(&mut self, key: K, value: V)
    where
        K: Eq + Hash + Ord
    {
        let _ = &self._inner.insert(key, value);
    }


    /// Returns a reference to the value of the key passed in.
    /// Because this btreemap mimicks the python defaultdict, it will also return a reference to a
    /// value if the key is not present. This reference will then be stored in the btreemap and be
    /// the default value.
    ///
    /// The key type must implement Hash and Eq.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    /// map.insert(10, 20);
    ///
    /// println!("{}", map.get(&10));
    /// ```
    #[must_use]
    pub fn get(&mut self, key: &K) -> &V
    where
        K: Eq + Hash + Clone + Ord
    {
        #[allow(unused_assignments)]
        let mut rv: Option<&V> = Option::None;
        let mut check: bool = false;
        for _key in self._inner.keys() {
            if key == _key {
                check = true;
            }
        }

        match check {
            true => {
                rv = self._inner.get(key);
            },
            false => {
                self.insert(key.clone(), V::default());
                rv = self._inner.get(key);
            },
        };

        rv.unwrap()
    }


    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    /// let number = map.get_mut(&10);
    ///
    /// *number = 100;
    ///
    /// println!("{}", map.get(&10));
    /// ```
    #[must_use]
    pub fn get_mut(&mut self, key: &K) -> &mut V
    where
        K: Hash + Eq + Clone + Ord
    {
        #[allow(unused_assignments)]
        let mut rv: Option<&mut V> = Option::None;
        let mut check: bool = false;
        for _key in self._inner.keys() {
            if key == _key {
                check = true;
            }
        }

        match check {
            true => {
                rv = self._inner.get_mut(key);
            },
            false => {
                self.insert(key.clone(), V::default());
                rv = self._inner.get_mut(key);
            },
        };

        rv.unwrap()
    }


    /// Removes a key from the map, returning the value at the key if the key was previously in the
    /// map. If the key is not present in the map it will return the default value.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    /// map.insert(10, 20);
    ///
    /// println!("{}", map.remove(&10));
    ///
    /// println!("{}", map.remove(&90));
    /// ```
    #[must_use]
    pub fn remove(&mut self, key: &K) -> V
    where
        K: Hash + Eq + Ord
    {
        match self._inner.remove(key) {
            Some(value) => value,
            None => V::default()
        }
    }


    /// Returns an iterator visiting all keys in arbitrary order. The iterator element type is
    /// &'a K.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    /// map.insert(10, 20);
    /// map.insert(11, 21);
    /// map.insert(12, 22);
    /// map.insert(13, 23);
    ///
    /// println!("{:?}", map.keys());
    /// ```
    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        self._inner.keys()
    }


    /// Returns an iterator visiting all values in arbitrary order. The iterator element type is
    /// &'a V.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    /// map.insert(10, 20);
    /// map.insert(11, 21);
    /// map.insert(12, 22);
    /// map.insert(13, 23);
    ///
    /// println!("{:?}", map.values());
    /// ```
    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        self._inner.values()
    }


    /// Returns the length of the keys in the map.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    /// map.insert(10, 20);
    /// map.insert(11, 21);
    /// map.insert(12, 22);
    /// map.insert(13, 23);
    ///
    /// println!("{}", map.len());
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self._inner.len()
    }


    /// Returns true if the map does not contain any keys.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    ///
    /// println!("{}", map.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self._inner.is_empty()
    }



    /// Returns true if the key passed in exists in the BTreeMap.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    /// map.insert(10, 20);
    ///
    /// println!("{}", map.contains_key(&10));
    /// ```
    #[inline]
    pub fn contains_key(&self, key: &K) -> bool
    where
        K: Eq + Hash + Ord
    {
        self._inner.contains_key(key)
    }

}


impl<K, V> Default for DefaultBTreeMap<K, V>
where
    K: Eq + Hash + Ord,
    V: Default,
{
    fn default() -> Self {
        Self::new()
    }
}


impl<K, V> IntoIterator for DefaultBTreeMap<K, V>
where
    K: Eq + Hash + Ord + Clone,
    V: Default,
{
    type Item = (K, V);
    type IntoIter = DefaultBTreeMapIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        let mut keys: Vec<K> = vec!();
        for k in self.keys() {
            keys.push(k.to_owned());
        }

        DefaultBTreeMapIter {
            _defaultbtree: self,
            keys,
        }
    }
}


impl<K, V> Iterator for DefaultBTreeMapIter<K, V>
where
    K: Eq + Hash + Ord + Clone,
    V: Default,
{
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        match self.keys.pop() {
            Some(key) => {
                let val = self._defaultbtree.remove(&key);
                Option::Some((key, val))
            },
            None => Option::None,
        }
    }
}


pub struct DefaultBTreeMapIter<K, V>
where
    K: Eq + Hash + Ord,
    V: Default,
{
    _defaultbtree: DefaultBTreeMap<K, V>,
    keys: Vec<K>
}


#[macro_export]
/// A quick way to instantiate a BTreeMap.
///
/// A trailing comma is allowed but not required here
///
/// # Example
/// ```
/// use defaultdict::DefaultBTreeMap;
///
///
/// let default_map: DefaultBTreeMap<i8, i8> = defaultdict::defaultbtreemap!(1,2,3,);
///
/// let custom_map: DefaultBTreeMap<i8, i8> = defaultdict::defaultbtreemap!(
///     (1, 1),
///     (2, 2),
/// );
/// ```
macro_rules! defaultbtreemap {

    // match 1
    ( $( ($key:expr, $val:expr) ),* $(,)? ) => {
        {
            let mut map = DefaultBTreeMap::new();
            $(
                let _ = map.insert($key, $val);
            )*
            map
        }
    };

    // match 2
    ( $( $key:expr ),* $(,)? ) => {
        {
            let mut map = DefaultBTreeMap::new();
            $(
                let _ = map.get(&$key);
            )*
            map
        }
    };

}
