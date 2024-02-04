#![deny(missing_docs)]

use std::borrow::Borrow;
use std::collections::{
    btree_map::{
        Entry, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, OccupiedEntry, Range, RangeMut,
        Values, ValuesMut,
    },
    BTreeMap,
};
use std::default::Default;
use std::ops::{Index, RangeBounds};

/// This struct mimicks the behaviour of a python defaultdict. This means alongside the traitbounds
/// that apply on the key and value that are inherited from the [`BTreeMap`], it also requires the
/// [`Default`] trait be implemented on the value type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefaultBTreeMap<K, V>
where
    K: Eq + Ord,
    V: Default,
{
    _inner: BTreeMap<K, V>,
    _default: V,
}

impl<K, V> DefaultBTreeMap<K, V>
where
    K: Eq + Ord,
    V: Default,
{
    /// Creates an empty [`DefaultBTreeMap`].
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let map = DefaultBTreeMap::<i8, i8>::new();
    /// assert_eq!(&0, map.get(&123));
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            _inner: BTreeMap::new(),
            _default: V::default(),
        }
    }

    /// Moves all elements from other into self, leaving other empty.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map1 = DefaultBTreeMap::<i8, i8>::new();
    /// let mut map2 = DefaultBTreeMap::<i8, i8>::new();
    ///
    /// for i in 0..10 {
    ///     if i % 2 == 0 {
    ///         map1.insert(i, i);
    ///     } else {
    ///         map2.insert(i, i);
    ///     }
    /// }
    ///
    /// assert_eq!(&3, map2.get(&3));
    /// map1.append(&mut map2);
    /// assert_eq!(&3, map1.get(&3));
    /// ```
    #[inline]
    pub fn append(&mut self, other: &mut DefaultBTreeMap<K, V>)
    where
        K: Ord,
        V: Clone,
    {
        self._inner.append(&mut other._inner);
    }

    /// Clears the map, removing all elements.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    ///
    /// for i in 0..10 {
    ///     map.insert(i, i);
    /// }
    ///
    /// map.clear();
    ///
    /// assert_eq!(&0, map.get(&3));
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self._inner.clear()
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
    /// assert!(map.contains_key(&10));
    /// ```
    #[inline]
    pub fn contains_key(&self, key: &K) -> bool {
        self._inner.contains_key(key)
    }

    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    ///
    /// # Example
    /// ```
    /// use std::collections::btree_map::Entry;
    /// use defaultdict::{DefaultBTreeMap, defaultbtreemap};
    ///
    /// let mut map: DefaultBTreeMap<i8, i8> = defaultbtreemap!(
    ///     (1, 2),
    /// );
    ///
    /// if let Entry::Occupied(inner) = map.entry(1) {
    ///     *inner.into_mut() += 10;
    /// };
    ///
    /// assert_eq!(&12, map.get(&1));
    /// ```
    #[inline]
    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        self._inner.entry(key)
    }

    /// Returns the first entry in the map for in-place manipulation. The key of this entry is the
    /// minimum key in the map.
    ///
    /// This method will return a None variant if the BTreeMap is empty.
    ///
    /// # Example
    /// ```
    /// use defaultdict::{DefaultBTreeMap, defaultbtreemap};
    ///
    /// let mut map: DefaultBTreeMap<i8, i8> = defaultbtreemap!(
    ///     (1, 2),
    ///     (2, 3),
    /// );
    ///
    /// if let Some(inner) = map.first_entry() {
    ///     *inner.into_mut() += 10;
    /// };
    ///
    /// assert_eq!(&12, map.get(&1));
    /// ```
    #[inline]
    pub fn first_entry(&mut self) -> Option<OccupiedEntry<K, V>>
    where
        K: Ord,
    {
        self._inner.first_entry()
    }

    /// Returns the first key-value pair in the map. The key in this pair is the minimum key in the
    /// map.
    ///
    /// This method will return a None variant if the BTreeMap is empty.
    ///
    /// # Example
    /// ```
    /// use defaultdict::{DefaultBTreeMap, defaultbtreemap};
    ///
    /// let mut map: DefaultBTreeMap<i8, i8> = defaultbtreemap!(
    ///     (1, 2),
    ///     (2, 3),
    /// );
    ///
    /// if let Some((key, value)) = map.first_key_value() {
    ///     assert_eq!(&1, key);
    ///     assert_eq!(&2, value);
    /// };
    /// ```
    #[inline]
    pub fn first_key_value(&self) -> Option<(&K, &V)>
    where
        K: Ord,
    {
        self._inner.first_key_value()
    }

    /// Returns a reference to the value of the key passed in.
    /// Because this btreemap mimicks the python defaultdict, it will also return a reference to a
    /// value if the key is not present.
    ///
    /// The key type must implement Eq.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    /// map.insert(10, 20);
    ///
    /// assert_eq!(&0, map.get(&1));
    /// assert_eq!(&20, map.get(&10));
    /// ```
    #[must_use]
    pub fn get(&self, key: &K) -> &V {
        self._inner.get(key).unwrap_or(&self._default)
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// The supplied key may be any borrowed form of the map’s key type, but the ordering on the
    /// borrowed form must match the ordering on the key type.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    /// map.insert(10, 20);
    ///
    /// let (key, value) = map.get_key_value(&10);
    ///
    /// assert_eq!(&10, key);
    /// assert_eq!(&20, value);
    /// ```
    #[inline]
    pub fn get_key_value<'a>(&'a self, key: &'a K) -> (&'a K, &'a V) {
        self._inner
            .get_key_value(key)
            .unwrap_or((key, &self._default))
    }

    /// Returns a mutable reference to the value corresponding to the key.
    /// If the key is not present in the hashmap it will return the default value and insert it in
    /// the map.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    /// let number = map.get_mut(&10);
    /// *number = 100;
    ///
    /// assert_eq!(&100, map.get(&10));
    /// ```
    #[must_use]
    pub fn get_mut(&mut self, key: &K) -> &mut V
    where
        K: Eq + Clone + Ord,
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
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    /// map.insert(10, 20);
    ///
    /// assert_eq!(&20, map.get(&10));
    /// ```
    #[inline]
    pub fn insert(&mut self, key: K, value: V) {
        let _ = &self._inner.insert(key, value);
    }

    /// Creates a consuming iterator visiting all the keys in sorted order. The map cannot be used
    /// after calling this. The iterator element type is `K`.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    /// map.insert(10, 20);
    /// map.insert(30, 40);
    ///
    /// let mut vec: Vec<i8> = map.into_keys().collect();
    ///
    /// let golden: Vec<i8> = vec![10, 30];
    ///
    /// assert_eq!(vec, golden);
    /// ```
    #[inline]
    pub fn into_keys(self) -> IntoKeys<K, V> {
        self._inner.into_keys()
    }

    /// Creates a consuming iterator visiting all the values in sorted order. The map cannot be used
    /// after calling this. The iterator element type is `V`.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    /// map.insert(10, 20);
    /// map.insert(30, 40);
    ///
    /// let mut vec: Vec<i8> = map.into_values().collect();
    ///
    /// let golden: Vec<i8> = vec![20, 40];
    ///
    /// assert_eq!(vec, golden);
    /// ```
    #[inline]
    pub fn into_values(self) -> IntoValues<K, V> {
        self._inner.into_values()
    }

    /// Returns true if the map does not contain any keys.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    ///
    /// assert!(map.is_empty());
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
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    /// map.insert(10, 20);
    /// map.insert(11, 21);
    /// map.insert(12, 22);
    /// map.insert(13, 23);
    ///
    /// let vec: Vec<&i8> = map.keys().collect();
    ///
    /// let golden: Vec<&i8> = vec![&10, &11, &12, &13];
    ///
    /// assert_eq!(vec, golden);
    /// ```
    #[inline]
    pub fn keys(&self) -> Keys<'_, K, V> {
        self._inner.keys()
    }

    /// Returns the last entry in the map for in-place manipulation. The key of this entry is the
    /// maximum key in the map.
    ///
    /// This method will return a None variant if the BTreeMap is empty.
    ///
    /// # Example
    /// ```
    /// use defaultdict::{DefaultBTreeMap, defaultbtreemap};
    ///
    /// let mut map: DefaultBTreeMap<i8, i8> = defaultbtreemap!(
    ///     (1, 2),
    ///     (3, 4)
    /// );
    ///
    /// if let Some(inner) = map.last_entry() {
    ///     *inner.into_mut() += 10;
    /// };
    ///
    /// assert_eq!(&14, map.get(&3));
    /// ```
    #[inline]
    pub fn last_entry(&mut self) -> Option<OccupiedEntry<K, V>>
    where
        K: Ord,
    {
        self._inner.last_entry()
    }

    /// Returns the last key-value pair in the map. The key in this pair is the maximum key in the
    /// map.
    ///
    /// This method will return a None variant if the BTreeMap is empty.
    ///
    /// # Example
    /// ```
    /// use defaultdict::{DefaultBTreeMap, defaultbtreemap};
    ///
    /// let mut map: DefaultBTreeMap<i8, i8> = defaultbtreemap!(
    ///     (1, 2),
    /// );
    ///
    /// let entry = map.last_key_value();
    ///
    /// assert_eq!(entry, Option::Some((&1, &2)))
    /// ```
    pub fn last_key_value(&self) -> Option<(&K, &V)>
    where
        K: Ord,
    {
        self._inner.last_key_value()
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
    /// assert_eq!(4, map.len());
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self._inner.len()
    }

    /// Removes and returns the first element in the map. The key of this element is the minimum key
    /// that was in the map.
    ///
    /// This method will return a None variant if the BTreeMap is empty.
    ///
    /// # Example
    /// ```
    /// use defaultdict::{DefaultBTreeMap, defaultbtreemap};
    ///
    /// let mut map: DefaultBTreeMap<i8, i8> = defaultbtreemap!(
    ///     (1, 2),
    /// );
    ///
    /// let first = map.pop_first();
    /// let second = map.pop_first();
    ///
    /// assert_eq!((1, 2), first.unwrap());
    /// assert!(second.is_none());
    /// ```
    pub fn pop_first(&mut self) -> Option<(K, V)>
    where
        K: Ord,
    {
        self._inner.pop_first()
    }

    /// Removes and returns the last element in the map. The key of this element is the maximum key
    /// that was in the map.
    ///
    /// This method will return a None variant if the BTreeMap is empty.
    ///
    /// # Example
    /// ```
    /// use defaultdict::{DefaultBTreeMap, defaultbtreemap};
    ///
    /// let mut map: DefaultBTreeMap<i8, i8> = defaultbtreemap!(
    ///     (1, 2),
    /// );
    ///
    /// let first = map.pop_last();
    /// let second = map.pop_last();
    ///
    /// assert_eq!((1, 2), first.unwrap());
    /// assert!(second.is_none());
    /// ```
    pub fn pop_last(&mut self) -> Option<(K, V)>
    where
        K: Ord,
    {
        self._inner.pop_last()
    }

    /// Constructs a double-ended iterator over a sub-range of elements in the map. The simplest way
    /// is to use the range syntax `min..max`, thus `range(min..max)` will yield elements from min
    /// (inclusive) to max (exclusive). The range may also be entered as `(Bound<T>, Bound<T>)`, so
    /// for example `range((Excluded(4), Included(10)))` will yield a left-exclusive, right-inclusive
    /// range from 4 to 10.
    ///
    /// # Panics
    /// Panics if range `start > end`. Panics if range `start == end` and both bounds are
    /// `Excluded`.
    ///
    /// # Example
    /// ```
    /// use defaultdict::{DefaultBTreeMap, defaultbtreemap};
    /// use std::ops::Bound::{Included, Excluded};
    ///
    /// let mut map: DefaultBTreeMap<i8, i8> = defaultbtreemap!();
    ///
    /// for i in 0..20 {
    ///     map.insert(i, i);
    /// }
    /// let vec: Vec<(&i8, &i8)> = map.range((Included(2), Excluded(10))).collect();
    /// let golden: Vec<(&i8, &i8)> =  vec![
    ///     (&2, &2),
    ///     (&3, &3),
    ///     (&4, &4),
    ///     (&5, &5),
    ///     (&6, &6),
    ///     (&7, &7),
    ///     (&8, &8),
    ///     (&9, &9),
    /// ];
    ///
    /// assert_eq!(vec, golden);
    /// ```
    #[inline]
    pub fn range<T, R>(&self, range: R) -> Range<K, V>
    where
        T: Ord + ?Sized,
        K: Borrow<T> + Ord,
        R: RangeBounds<T>,
    {
        self._inner.range(range)
    }

    /// Constructs a mutable double-ended iterator over a sub-range of elements in the map. The
    /// simplest way is to use the range syntax `min..max`, thus `range(min..max)` will yield
    /// elements from min (inclusive) to max (exclusive). The range may also be entered as
    /// `(Bound<T>, Bound<T>)`, so for example `range((Excluded(4), Included(10)))` will yield a
    /// left-exclusive, right-inclusive range from 4 to 10.
    ///
    /// # Panics
    /// Panics if range `start > end`. Panics if range `start == end` and both bounds are
    /// `Excluded`.
    ///
    /// # Example
    /// ```
    /// use defaultdict::{DefaultBTreeMap, defaultbtreemap};
    /// use std::ops::Bound::{Included, Excluded};
    ///
    /// let mut map: DefaultBTreeMap<i8, i8> = defaultbtreemap!();
    ///
    /// for i in 0..20 {
    ///     map.insert(i, i);
    /// }
    ///
    /// for (key, value) in map.range_mut(2..10) {
    ///     *value += 1;
    ///     println!("key: {}\tvalue: {}", key, value);
    /// }
    /// ```
    #[inline]
    pub fn range_mut<T, R>(&mut self, range: R) -> RangeMut<K, V>
    where
        T: Ord + ?Sized,
        K: Borrow<T> + Ord,
        R: RangeBounds<T>,
    {
        self._inner.range_mut(range)
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
    /// assert_eq!(20, map.remove(&10));
    /// assert_eq!(0, map.remove(&90));
    /// ```
    #[must_use]
    pub fn remove(&mut self, key: &K) -> V {
        self._inner.remove(key).unwrap_or_default()
    }

    /// Removes a key from the map, returning the stored key and value if the key was previously in
    /// the map. If the key is not present in the map, a default value will be returned.
    ///
    /// The key may be any borrowed form of the map’s key type, but the ordering on the borrowed
    /// form must match the ordering on the key type.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    ///
    /// for i in 0..10 {
    ///     map.insert(i, 20);
    /// }
    ///
    /// let entry = map.remove_entry(&0);
    ///
    /// assert_eq!((5, 20), map.remove_entry(&5));
    /// assert_eq!((5, 0) , map.remove_entry(&5));
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
    /// use defaultdict::{DefaultBTreeMap, defaultbtreemap};
    ///
    /// let mut map = defaultbtreemap!();
    /// for i in 0..10 {
    ///     map.insert(i, i);
    /// }
    ///
    /// map.retain(|key, value| {
    ///     key <= &2
    /// });
    /// let golden = defaultbtreemap!(
    ///     (0, 0),
    ///     (1, 1),
    ///     (2, 2),
    /// );
    ///
    /// assert_eq!(golden, map);
    /// ```
    pub fn retain<F>(&mut self, func: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self._inner.retain(func);
    }

    /// TODO
    #[inline]
    pub fn split_off<Q>(&mut self, key: &Q) -> DefaultBTreeMap<K, V>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q> + Ord + Clone,
    {
        self._inner.split_off(key).into()
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
    /// let vec: Vec<&i8> = map.values().collect();
    /// let golden: Vec<&i8> = vec![&20, &21, &22, &23];
    ///
    ///
    /// assert_eq!(golden, vec);
    /// ```
    #[inline]
    pub fn values(&self) -> Values<'_, K, V> {
        self._inner.values()
    }

    /// Gets a mutable iterator over the values of the map, in order by key.
    ///
    /// # Example
    /// ```
    /// use defaultdict::DefaultBTreeMap;
    ///
    /// let mut map = DefaultBTreeMap::<i8, i8>::new();
    ///
    /// for i in 0..10 {
    ///     map.insert(i, i);
    /// }
    ///
    /// for value in map.values_mut() {
    ///     *value += 1;
    /// }
    ///
    /// for i in 0..10 {
    ///     assert_eq!(&(i + 1), map.get(&i))
    /// }
    /// ```
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        self._inner.values_mut()
    }
}

impl<K, V> Default for DefaultBTreeMap<K, V>
where
    K: Eq + Ord,
    V: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> IntoIterator for DefaultBTreeMap<K, V>
where
    K: Eq + Ord + Clone,
    V: Default,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self._inner.into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a DefaultBTreeMap<K, V>
where
    K: Eq + Ord,
    V: Default,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self._inner.iter()
    }
}

impl<K, V> Index<&K> for DefaultBTreeMap<K, V>
where
    K: Eq + Ord,
    V: Default,
{
    type Output = V;

    fn index(&self, key: &K) -> &V {
        self._inner.get(key).unwrap_or(&self._default)
    }
}

impl<'a, K, V> IntoIterator for &'a mut DefaultBTreeMap<K, V>
where
    K: Eq + Ord,
    V: Default,
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self._inner.iter_mut()
    }
}

impl<K, V> From<BTreeMap<K, V>> for DefaultBTreeMap<K, V>
where
    K: Eq + Ord,
    V: Default,
{
    fn from(btree: BTreeMap<K, V>) -> Self {
        Self {
            _inner: btree,
            _default: V::default(),
        }
    }
}

impl<K, V> From<DefaultBTreeMap<K, V>> for BTreeMap<K, V>
where
    K: Eq + Ord,
    V: Default,
{
    fn from(btree: DefaultBTreeMap<K, V>) -> Self {
        btree._inner
    }
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
    ( ) => {
        {
            DefaultBTreeMap::new()
        }
    };

    // match 2
    ( $( ($key:expr, $val:expr) ),* $(,)? ) => {
        {
            let mut map = DefaultBTreeMap::new();
            $(
                let _ = map.insert($key, $val);
            )*
            map
        }
    };

    // match 3
    ( $( $key:expr ),* $(,)? ) => {
        {
            let mut map = DefaultBTreeMap::new();
            $(
                let _ = map.get_mut(&$key);
            )*
            map
        }
    };

}
