#![deny(missing_docs)]


use std::collections::{HashMap, hash_map::{Keys, Values, Iter, IterMut}};
use std::default::Default;
use std::hash::Hash;
use std::ops::Index;


/// This struct mimicks the behaviour of a python defaultdict. This means alongside the traitbounds
/// that apply on the key and value that are inherited from the [`HashMap`], it also requires the
/// [`Default`] trait be implemented on the value type.
#[derive(Debug, PartialEq, Eq)]
pub struct DefaultHashMap<K, V>
where
    K: Eq + Hash,
    V: Default,
{
    _inner: HashMap<K, V>,
    _default: V,
}


impl<K, V> DefaultHashMap<K, V>
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
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            _inner: HashMap::new(),
            _default: V::default(),
        }
    }


    /// Returns true if the key passed in exists in the HashMap.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// map.insert(10, 20);
    ///
    /// println!("{}", map.contains_key(&10));
    /// ```
    #[inline]
    pub fn contains_key(&self, key: &K) -> bool
    where
        K: Eq + Hash
    {
        self._inner.contains_key(key)
    }


    /// Returns a reference to the value of the key passed in.
    /// Because this hashmap mimicks the python defaultdict, it will also return a reference to a
    /// value if the key is not present.
    ///
    /// The key type must implement Hash and Eq.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// map.insert(10, 20);
    ///
    /// println!("{}", map.get(&10));
    /// ```
    #[must_use]
    pub fn get(&self, key: &K) -> &V
    where
        K: Eq + Hash + Clone
    {
        match self._inner.get(key) {
            Some(val) => val,
            None => &self._default,
        }
    }


    /// Returns a mutable reference to the value corresponding to the key.
    /// If the key is not present in the hashmap it will return the default value and insert it in
    /// the map.
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
    /// println!("{}", map.get(&10));
    /// ```
    #[must_use]
    pub fn get_mut(&mut self, key: &K) -> &mut V
    where
        K: Hash + Eq + Clone
    {
        let exists = self._inner.keys().any(|k| key == k);
        if !exists {
            self.insert(key.clone(), V::default());
        }
        self._inner.get_mut(key).unwrap()
    }


    /// Inserts a key value pair into the map.
    ///
    /// If the map had the key already present it will be overwritten.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    /// map.insert(10, 20)
    /// ```
    #[inline]
    pub fn insert(&mut self, key: K, value: V)
    where
        K: Eq + Hash
    {
        let _ = &self._inner.insert(key, value);
    }


    /// Returns true if the map does not contain any keys.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultHashMap;
    ///
    /// let mut map = DefaultHashMap::<i8, i8>::new();
    ///
    /// println!("{}", map.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self._inner.is_empty()
    }


    /// Returns an iterator visiting all keys in arbitrary order. The iterator element type is 
    /// &'a K.
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
    /// println!("{:?}", map.keys());
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
    /// println!("{}", map.len());
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self._inner.len()
    }


    /// Removes a key from the map, returning the value at the key if the key was previously in the
    /// map. If the key is not present in the map it will return the default value.
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
    /// ```
    #[must_use]
    pub fn remove(&mut self, key: &K) -> V
    where
        K: Hash + Eq
    {
        match self._inner.remove(key) {
            Some(value) => value,
            None => V::default()
        }
    }


    /// Retains only the elements specified by the predicate.
    /// In other words, remove all pairs (k, v) for which f(&k, &mut v) returns false. The elements
    /// are visited in unsorted (and unspecified) order.
    ///
    /// # Example
    /// ```
    /// use defaultdict::{DefaultHashMap, defaulthashmap};
    ///
    /// let mut map = defaulthashmap!(
    ///     (0, 0),
    ///     (1, 0),
    ///     (2, 0),
    ///     (3, 0),
    ///     (4, 0),
    ///     (5, 0),
    ///     (6, 0),
    ///     (7, 0),
    ///     (8, 0),
    ///     (9, 0),
    /// );
    ///
    /// map.retain(|key, value| {
    ///     key <= &2
    /// });
    ///
    /// println!("{:?}", map);
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
    /// println!("{:?}", map.values());
    /// ```
    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        self._inner.values()
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


impl<K, V> IntoIterator for DefaultHashMap<K, V>
where
    K: Eq + Hash + Ord + Clone,
    V: Default,
{
    type Item = (K, V);
    type IntoIter = DefaultHashMapIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        let mut keys: Vec<K> = vec!();
        for k in self.keys() {
            keys.push(k.to_owned());
        }

        DefaultHashMapIter {
            _defaulthashmap: self,
            keys,
        }
    }
}


impl<'a, K, V> IntoIterator for &'a DefaultHashMap<K, V>
where
    K: Eq + Hash + Ord + Clone,
    V: Default,
{
    type Item = (&'a K,&'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self._inner.iter()
    }
}


impl<K, V> Index<&K> for DefaultHashMap<K, V>
where
    K: Eq + Hash + Ord + Clone,
    V: Default,
{
    type Output = V;

    fn index(&self, key: &K) -> &V {
        match self._inner.get(key) {
            Some(v) => v,
            None => {
                panic!("no entry found for key")
            }
        }
    }
}


impl<'a, K, V> IntoIterator for &'a mut DefaultHashMap<K, V>
where
    K: Eq + Hash + Ord + Clone,
    V: Default,
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self._inner.iter_mut()
    }
}


impl<K, V> From<HashMap<K, V>> for DefaultHashMap<K, V>
where
    K: Eq + Hash + Ord + Clone,
    V: Default,
{
    fn from(btree: HashMap<K, V>) -> Self {
        Self {
            _inner: btree,
            _default: V::default(),
        }
    }

}


impl<K, V> Into<HashMap<K, V>> for DefaultHashMap<K, V>
where
    K: Eq + Hash + Ord + Clone,
    V: Default,
{
    fn into(self) -> HashMap<K, V> {
        self._inner
    }
}


impl<K, V> Iterator for DefaultHashMapIter<K, V>
where
    K: Eq + Hash + Ord + Clone,
    V: Default,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        match self.keys.pop() {
            Some(key) => {
                let val = self._defaulthashmap.remove(&key);
                Option::Some((key, val))
            },
            None => Option::None,
        }
    }
}


pub struct DefaultHashMapIter<K, V>
where
    K: Eq + Hash + Ord,
    V: Default,
{
    _defaulthashmap: DefaultHashMap<K, V>,
    keys: Vec<K>
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
    ( $( ($key:expr, $val:expr) ),* $(,)? ) => {
        {
            let mut map = DefaultHashMap::new();
            $(
                let _ = map.insert($key, $val);
            )*
            map
        }
    };

    // match 2
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
