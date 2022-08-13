
use std::collections::{HashMap, hash_map::{Keys, Values}};
use std::hash::Hash;


/// This struct mimicks the behaviour of a python defaultdict. This means alongside the traitbounds
/// that apply on the key and value that are inherited from the [`HashMap`], it also requires the
/// [`Default`] trait be implemented on the value type.
pub struct DefaultHashMap<K, V>
where
    K: Eq + Hash,
    V: Default,
{
    _inner: HashMap<K, V>
}


impl<K: Eq + Hash, V: Default> DefaultHashMap<K, V> {
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
            _inner: HashMap::new()
        }
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


    /// Returns a reference to the value of the key passed in.
    /// Because this hashmap mimicks the python defaultdict, it will also return a reference to a
    /// value if the key is not present. This reference will then be stored in the hashmap and be
    /// the default value.
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
    /// println!("{}", map.get(10));
    /// ```
    #[must_use]
    pub fn get(&mut self, key: K) -> &V
    where
        K: Sized + Eq + Hash + Clone
    {
        #[allow(unused_assignments)]
        let mut rv: Option<&V> = Option::None;
        let mut check: bool = false;
        for _key in self._inner.keys() {
            if &key == _key {
                check = true;
            }
        }

        match check {
            true => {
                rv = self._inner.get(&key);
            },
            false => {
                self.insert(key.clone(), V::default());
                rv = self._inner.get(&key);
            },
        };

        rv.unwrap()
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


    /// Returns an iterator visiting all keys in arbitrary order. The iterator element type is &'a K.
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

}

