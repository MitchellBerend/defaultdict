#![deny(missing_docs)]

use std::borrow::Borrow;
use std::collections::{
    hash_map::{
        Drain, Entry, IntoKeys, IntoValues, Iter, IterMut, Keys, RandomState, Values, ValuesMut,
    },
    HashMap,
};
use std::default::Default;
use std::hash::{BuildHasher, Hash};
use std::ops::Index;
/// This struct mimicks the behaviour of a python defaultdict. This means alongside the traitbounds
/// that apply on the key and value that are inherited from the [`HashMap`], it also requires the
/// [`Default`] trait be implemented on the value type.
#[derive(Clone, Debug)]
pub struct DefaultHashMap<K, V, S = RandomState>
where
    K: Eq + Hash,
    V: Default,
{
    _inner: HashMap<K, V, S>,
    _default: V,
}

impl<K, V> DefaultHashMap<K, V, RandomState>
where
    K: Eq + Hash,
    V: Default,
{
    /// Creates an empty [`DefaultHashMap`].
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let map = DefaultHashMap::<i8, i8>::new();
    ///
    /// // This map is empty
    /// let collected: Vec<(i8, i8)> = map.into_iter().collect();
    /// let empty: Vec<(i8, i8)> = vec![];
    ///
    /// assert_eq!(empty, collected);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            _inner: HashMap::new(),
            _default: V::default(),
        }
    }
}

impl<K, V, S> DefaultHashMap<K, V, S>
where
    K: Eq + Hash,
    V: Default,
    S: BuildHasher,
{
    /// Creates an empty [`DefaultHashMap`] which will use the given hash builder to hash
    /// keys.
    ///
    /// Warning: `hash_builder` is normally randomly generated, and
    /// is designed to allow HashMaps to be resistant to attacks that
    /// cause many collisions and very poor performance. Setting it
    /// manually using this function can expose a DoS attack vector.
    ///
    /// The `hash_builder` passed should implement the [`BuildHasher`] trait for
    /// the HashMap to be useful, see its documentation for details.
    ///
    /// # Examples
    ///
    /// ```
    /// use defaultdict::DefaultHashMap;
    /// use std::collections::hash_map::RandomState;
    ///
    /// let s = RandomState::new();
    /// let mut map = DefaultHashMap::with_hasher(s);
    /// map.insert(1, 2);
    /// ```
    #[inline]
    pub fn with_hasher(hash_builder: S) -> Self {
        DefaultHashMap {
            _inner: HashMap::with_hasher(hash_builder),
            _default: V::default(),
        }
    }

    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// This number is a lower bound; the `HashMap<K, V>` might be able to hold more, but is
    /// guaranteed to be able to hold at least this many.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// map.insert(10, 20);
    ///
    /// println!("{}", map.capacity());
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self._inner.capacity()
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// map.insert(10, 20);
    /// map.clear();
    ///
    /// // This map is empty
    /// let collected: Vec<(i8, i8)> = map.into_iter().collect();
    /// let empty: Vec<(i8, i8)> = vec![];
    ///
    /// assert_eq!(empty, collected);
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self._inner.clear()
    }

    /// Returns `true` if the key passed in exists in the HashMap.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// map.insert(10, 20);
    ///
    /// assert!(map.contains_key(&10));
    /// ```
    #[inline]
    pub fn contains_key(&self, key: &K) -> bool
    where
        K: Eq + Hash,
    {
        self._inner.contains_key(key)
    }

    /// Clears the map, returning all key-value pairs as an iterator. Keeps the allocated memory for
    /// reuse.
    ///
    /// If the returned iterator is dropped before being fully consumed, it drops the remaining
    /// key-value pairs. The returned iterator keeps a mutable borrow on the map to optimize its
    /// implementation.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// map.insert(10, 20);
    ///
    /// let contents: Vec<(i8, i8)> = map.drain().into_iter().collect();
    /// assert_eq!(vec![(10, 20)], contents);
    ///
    ///
    /// // The map is empty
    /// let collected: Vec<(i8, i8)> = map.into_iter().collect();
    /// let empty: Vec<(i8, i8)> = vec![];
    ///
    /// assert_eq!(empty, collected);
    /// ```
    #[inline]
    pub fn drain(&mut self) -> Drain<K, V> {
        self._inner.drain()
    }

    /// Gets the given key’s corresponding entry in the map for in-place manipulation.
    ///
    /// # Example
    /// ```
    /// use std::collections::hash_map::Entry;
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// map.insert(10, 20);
    ///
    /// assert_eq!(&20, map.get(&10));
    ///
    /// if let Entry::Occupied(inner) = map.entry(10) {
    ///     *inner.into_mut() += 10;
    /// };
    ///
    /// assert_eq!(&30, map.get(&10));
    /// ```
    #[inline]
    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        self._inner.entry(key)
    }

    /// Returns a reference to the value of the key passed in.
    /// Because this hashmap mimicks the python defaultdict, it will also return a reference to a
    /// value if the key is not present.
    ///
    /// The key type must implement [`Hash`] and [`Eq`].
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// map.insert(10, 20);
    ///
    /// assert_eq!(&20, map.get(&10));
    /// assert_eq!(&0, map.get(&20));
    /// ```
    #[must_use]
    pub fn get<Q>(&self, key: &Q) -> &V
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self._inner.get(key).unwrap_or(&self._default)
    }

    /// Returns the key-value pair corresponding to the supplied key.
    /// The supplied key may be any borrowed form of the map’s key type, but [`Hash`] and [`Eq`] on
    /// the borrowed form must match those for the key type.Returns a reference to the value of the
    /// key passed in.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// map.insert(10, 20);
    ///
    /// let key_value: (&i8, &i8) = map.get_key_value(&10);
    ///
    /// assert_eq!((&10, &20), key_value);
    /// ```
    #[must_use]
    pub fn get_key_value<'a>(&'a self, key: &'a K) -> (&K, &V)
    where
        K: Eq + Hash,
    {
        self._inner
            .get_key_value(key)
            .unwrap_or((key, &self._default))
    }

    /// Returns a mutable reference to the value corresponding to the key.
    /// If the key is not present in the hashmap it will return the default value and insert it in
    /// the map.
    ///
    /// The key may be any borrowed form of the map’s key type, but [`Hash`] and [`Eq`] on the
    /// borrowed form must match those for the key type.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// let number = map.get_mut(&10);
    ///
    /// *number = 100;
    ///
    /// assert_eq!(&100, map.get(&10));
    /// ```
    #[must_use]
    pub fn get_mut(&mut self, key: &K) -> &mut V
    where
        K: Hash + Eq + Clone,
    {
        let exists = self._inner.keys().any(|k| key == k);
        if !exists {
            self.insert(key.clone(), V::default());
        }
        self._inner.get_mut(key).unwrap()
    }

    /// Inserts a key value pair into the map. If the map did not have this key present, `None` is
    /// returned.
    ///
    /// If the map had the key already present it will be overwritten.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// map.insert(10, 20);
    ///
    /// let old_value = map.insert(10, 30).unwrap();
    ///
    /// assert_eq!(&30, map.get(&10));
    /// assert_eq!(20, old_value);
    /// ```
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self._inner.insert(key, value)
    }

    /// Creates a consuming iterator visiting all the keys in arbitrary order. The map cannot be
    /// used after calling this. The iterator element type is `K`.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// map.insert(10, 20);
    /// map.insert(30, 40);
    ///
    /// let mut collected: Vec<i8> = map.into_keys().collect();
    /// collected.sort();
    ///
    /// assert_eq!(vec![10, 30], collected);
    /// ```
    #[inline]
    pub fn into_keys(self) -> IntoKeys<K, V> {
        self._inner.into_keys()
    }

    /// Creates a consuming iterator visiting all the values in arbitrary order. The map cannot be
    /// used after calling this. The iterator element type is `V`.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// map.insert(10, 20);
    /// map.insert(30, 40);
    ///
    /// let mut collected: Vec<i8> = map.into_values().collect();
    /// collected.sort();
    ///
    /// assert_eq!(vec![20, 40], collected);
    /// ```
    #[inline]
    pub fn into_values(self) -> IntoValues<K, V> {
        self._inner.into_values()
    }

    /// Returns `true` if the map does not contain any keys.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let map = DefaultHashMap::<i8, i8>::new();
    ///
    /// assert!(map.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self._inner.is_empty()
    }

    /// Returns an iterator visiting all keys in arbitrary order. The iterator element type is
    /// `&'a K`.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// map.insert(10, 20);
    /// map.insert(11, 21);
    /// map.insert(12, 22);
    /// map.insert(13, 23);
    ///
    /// let mut collected: Vec<&i8> = map.keys().collect();
    /// collected.sort();
    ///
    /// assert_eq!(vec![&10, &11, &12, &13], collected);
    /// ```
    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        self._inner.keys()
    }

    /// Returns the length of the keys in the map.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// map.insert(10, 20);
    /// map.insert(11, 21);
    /// map.insert(12, 22);
    /// map.insert(13, 23);
    ///
    /// assert_eq!(4, map.len());
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self._inner.len()
    }

    /// Removes a key from the map, returning the value at the key if the key was previously in the
    /// map. If the key is not present in the map it will return the default value.
    ///
    /// The key may be any borrowed form of the map’s key type, but [`Hash`] and [`Eq`] on the
    /// borrowed form must match those for the key type.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// map.insert(10, 20);
    ///
    /// println!("{}", map.remove(&10));
    ///
    /// println!("{}", map.remove(&90));
    ///
    /// assert!(map.is_empty());
    /// ```
    #[must_use]
    pub fn remove<Q>(&mut self, key: &Q) -> V
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self._inner.remove(key).unwrap_or_default()
    }

    /// Removes a key from the map, returning the stored key and value if the key was previously in
    /// the map. If the key is not present in the map, a default value will be returned.
    ///
    /// The key may be any borrowed form of the map’s key type, but [`Hash`] and [`Eq`] on the
    /// borrowed form must match those for the key type.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    ///
    /// for i in 0..10 {
    ///     map.insert(i, 20);
    /// }
    ///
    /// let entry = map.remove_entry(&0);
    ///
    /// let default_entry = map.remove_entry(&0);
    ///
    /// assert_eq!((1, 20), map.remove_entry(&1));
    /// assert_eq!((1, 0), map.remove_entry(&1));
    /// ```
    #[must_use]
    pub fn remove_entry(&mut self, key: &K) -> (K, V)
    where
        K: Clone,
        V: Clone,
    {
        self._inner
            .remove_entry(key)
            .unwrap_or((key.clone(), self._default.to_owned()))
    }

    /// Retains only the elements specified by the predicate.
    /// In other words, remove all pairs (k, v) for which f(&k, &mut v) returns false. The elements
    /// are visited in unsorted (and unspecified) order.
    ///
    /// # Example
    /// ```
    /// use defaultdict::{DefaultHashMap, defaulthashmap};
    ///
    /// let mut map: DefaultHashMap<i8, i8> = defaulthashmap!();
    ///
    /// for i in 0..10 {
    ///     map.insert(i, i);
    /// }
    ///
    /// map.retain(|key, value| {
    ///     key <= &2
    /// });
    ///
    /// let mut collected: Vec<(i8, i8)> = map.into_iter().collect();
    /// collected.sort();
    /// let golden: Vec<(i8, i8)> = vec![(0, 0), (1, 1), (2, 2)];
    ///
    /// assert_eq!(golden, collected);
    /// ```
    pub fn retain<F>(&mut self, func: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self._inner.retain(func);
    }

    /// Returns an iterator visiting all values in arbitrary order. The iterator element type is
    /// &'a V.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// map.insert(10, 20);
    /// map.insert(11, 21);
    /// map.insert(12, 22);
    /// map.insert(13, 23);
    ///
    /// let mut collected: Vec<&i8> = map.values().collect();
    /// collected.sort();
    /// let golden: Vec<&i8> = vec![&20, &21, &22, &23];
    ///
    /// assert_eq!(golden, collected);
    /// ```
    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        self._inner.values()
    }

    /// Gets a mutable iterator over the values of the map, in order by key.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    ///
    /// for i in 0..10 {
    ///     map.insert(i, i);
    /// }
    ///
    /// for value in map.values_mut() {
    ///     *value += 1;
    /// }
    ///
    /// let mut collected: Vec<&i8> = map.values().collect();
    /// collected.sort();
    /// let golden: Vec<&i8> = vec![&1, &2, &3, &4, &5, &6, &7, &8, &9, &10];
    ///
    /// assert_eq!(golden, collected);
    /// ```
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        self._inner.values_mut()
    }
}

impl<K, V> Default for DefaultHashMap<K, V>
where
    K: Eq + Hash,
    V: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, S> PartialEq for DefaultHashMap<K, V, S>
where
    K: Eq + Hash,
    V: PartialEq + Default,
    S: BuildHasher,
{
    fn eq(&self, other: &DefaultHashMap<K, V, S>) -> bool {
        self._inner == other._inner && self._default == other._default
    }
}

impl<K, V, S> Eq for DefaultHashMap<K, V, S>
where
    K: Eq + Hash,
    V: Eq + Default,
    S: BuildHasher,
{
}

impl<K, V, S> IntoIterator for DefaultHashMap<K, V, S>
where
    K: Eq + Hash + Ord + Clone,
    V: Default,
    S: BuildHasher,
{
    type Item = (K, V);
    type IntoIter = DefaultHashMapIter<K, V, S>;

    fn into_iter(self) -> Self::IntoIter {
        let mut keys: Vec<K> = vec![];
        for k in self.keys() {
            keys.push(k.to_owned());
        }

        DefaultHashMapIter {
            _defaulthashmap: self,
            keys,
        }
    }
}

impl<'a, K, V, S> IntoIterator for &'a DefaultHashMap<K, V, S>
where
    K: Eq + Hash + Ord + Clone,
    V: Default,
    S: BuildHasher,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self._inner.iter()
    }
}

impl<K, V, S> Index<&K> for DefaultHashMap<K, V, S>
where
    K: Eq + Hash + Ord + Clone,
    V: Default,
    S: BuildHasher,
{
    type Output = V;

    fn index(&self, key: &K) -> &V {
        self._inner.get(key).unwrap_or(&self._default)
    }
}

impl<'a, K, V, S> IntoIterator for &'a mut DefaultHashMap<K, V, S>
where
    K: Eq + Hash + Ord + Clone,
    V: Default,
    S: BuildHasher,
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self._inner.iter_mut()
    }
}

impl<K, V, S> From<HashMap<K, V, S>> for DefaultHashMap<K, V, S>
where
    K: Eq + Hash + Ord + Clone,
    V: Default,
    S: BuildHasher,
{
    fn from(hashmap: HashMap<K, V, S>) -> Self {
        Self {
            _inner: hashmap,
            _default: V::default(),
        }
    }
}

impl<K, V, S> From<DefaultHashMap<K, V, S>> for HashMap<K, V, S>
where
    K: Eq + Hash + Ord + Clone,
    V: Default,
    S: BuildHasher,
{
    fn from(hashmap: DefaultHashMap<K, V, S>) -> Self {
        hashmap._inner
    }
}

impl<K, V, S> Iterator for DefaultHashMapIter<K, V, S>
where
    K: Eq + Hash + Ord + Clone,
    V: Default,
    S: BuildHasher,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        match self.keys.pop() {
            Some(key) => {
                let val = self._defaulthashmap.remove(&key);
                Option::Some((key, val))
            }
            None => Option::None,
        }
    }
}

pub struct DefaultHashMapIter<K, V, S>
where
    K: Eq + Hash + Ord,
    V: Default,
    S: BuildHasher,
{
    _defaulthashmap: DefaultHashMap<K, V, S>,
    keys: Vec<K>,
}

#[macro_export]
/// A quick way to instantiate a HashMap.
///
/// A trailing comma is allowed but not required here
///
/// # Example
/// ```
/// use defaultdict::DefaultHashMap;
///
///
/// let default_map: DefaultHashMap<i8, i8> = defaultdict::defaulthashmap!(1,2,3,);
///
/// let custom_map: DefaultHashMap<i8, i8> = defaultdict::defaulthashmap!(
///     (1, 1),
///     (2, 2),
/// );
/// ```
macro_rules! defaulthashmap {

    // match 1
    ( ) => {
        {
            DefaultHashMap::new()
        }
    };

    // match 2
    ( $( ($key:expr, $val:expr) ),* $(,)? ) => {
        {
            let mut map = DefaultHashMap::new();
            $(
                let _ = map.insert($key, $val);
            )*
            map
        }
    };

    // match 3
    ( $( $key:expr ),* $(,)? ) => {
        {
            let mut map = DefaultHashMap::new();
            $(
                let _ = map.get_mut(&$key);
            )*
            map
        }
    };

}
