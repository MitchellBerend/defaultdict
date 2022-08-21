//! This library exposes a struct that mimicks the behaviour of the python
//! [defaultdict](https://docs.python.org/3/library/collections.html#collections.defaultdict).
//!
//! This behaviour does require that the type of the value does have the [`Default`] implemented.


mod default_btree;
mod default_hashmap;

pub use default_hashmap::DefaultHashMap;
pub use default_btree::DefaultBTreeMap;


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn default_value_i8_hashmap() {
        let mut map = DefaultHashMap::<i8, i8>::new();
        assert_eq!(map.get(&1), &0);
    }

    #[test]
    fn default_new_hashmap() {
        let map1: DefaultHashMap<i8, i8> = DefaultHashMap::new();

        let map2: DefaultHashMap<i8, i8> = DefaultHashMap::default();

        assert_eq!(map1, map2);
    }

    #[test]
    fn non_default_value_i8_hashmap() {
        let mut map = DefaultHashMap::<i8, i8>::new();
        map.insert(1,2);
        assert_eq!(map.get(&1), &2);
    }

    #[test]
    fn insert_remove_default_hashmap() {
        let mut map = DefaultHashMap::<i8, i8>::new();
        map.insert(1,123);
        let _ = map.remove(&1);
        assert_eq!(map.get(&1), &0);
    }

    #[test]
    fn remove_default_hashmap() {
        let mut map = DefaultHashMap::<i8, i8>::new();
        let i = map.remove(&1);
        assert_eq!(i, 0);
    }

    #[test]
    fn collect_keys_hashmap() {
        let mut map = DefaultHashMap::<i8, i8>::new();
        map.insert(1,2);
        map.insert(3,4);

        let mut keys: Vec<&i8> = map.keys().collect();
        keys.sort();

        assert_eq!(keys, vec!(&1, &3));
    }

    #[test]
    fn collect_values_hashmap() {
        let mut map = DefaultHashMap::<i8, i8>::new();
        map.insert(1,2);
        map.insert(3,4);

        let mut values: Vec<&i8> = map.values().collect();
        values.sort();

        assert_eq!(values, vec!(&2, &4));
    }

    #[test]
    fn check_len_hashmap() {
        let mut map = DefaultHashMap::<i8, i8>::new();
        map.insert(1,2);
        map.insert(3,4);

        assert_eq!(map.len(), 2);
    }

    #[test]
    fn contains_key_hashmap() {
        let mut map = DefaultHashMap::<i8, i8>::new();
        map.insert(1,2);
        map.insert(3,4);

        assert_eq!(map.contains_key(&1), true);
        assert_eq!(map.contains_key(&2), false);
    }

    #[test]
    fn get_mut_hashmap() {
        let mut map = DefaultHashMap::<i8, Vec<i8>>::new();

        let v1 = map.get_mut(&1);

        v1.push(10);

        assert_eq!(map.get(&1), &vec!(10));
    }

    #[test]
    fn retain_hashmap() {
        let mut map: DefaultHashMap<i8, u8> = defaulthashmap!(
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
        );

        map.retain(|key, _| {
            key <= &2
        });

        let correct_map: DefaultHashMap<i8, u8> = defaulthashmap!(
            (1, 1),
            (2, 2),
        );

        assert_eq!(correct_map, map);
    }

    #[test]
    fn is_empty_hashmap() {
        let mut map = DefaultHashMap::<i8, Vec<i8>>::new();

        let _ = map.get_mut(&1);

        let map1 = DefaultHashMap::<i8, Vec<i8>>::new();

        assert_eq!(map.is_empty(), false);
        assert_eq!(map1.is_empty(), true);

    }

    #[test]
    fn into_iter_no_default_hashmap() {
        let map: DefaultHashMap<i8, u8> = defaulthashmap!(
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
        );

        let mut v: Vec<(i8, u8)> = map.into_iter().collect();
        v.sort();

        let correct_v: Vec<(i8, u8)> = vec!(
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
        );

        assert_eq!(v, correct_v);
    }


    #[test]
    fn into_iter_default_hashmap() {
        let mut map: DefaultHashMap<i8, u8> = DefaultHashMap::new();

        for i in 0..10 {
            let _  = map.get(&i);
        }

        let mut v: Vec<(i8, u8)> = map.into_iter().collect();
        let mut correct_v: Vec<(i8, u8)> = vec!();
        v.sort();

        for i in 0..10 {
            correct_v.push((i as i8, 0 as u8));
        }

        assert_eq!(v, correct_v);
    }

    #[test]
    fn borrow_loop_over_default_hashmap() {
        let mut map: DefaultHashMap<i8, u8> = DefaultHashMap::new();

        for i in 0..10 {
            let _  = map.get(&i);
        }

        let mut v: Vec<(&i8, &u8)> = vec!();
        let correct_v: Vec<(&'static i8, &'static u8)> = vec!(
            (&0, &0),
            (&1, &0),
            (&2, &0),
            (&3, &0),
            (&4, &0),
            (&5, &0),
            (&6, &0),
            (&7, &0),
            (&8, &0),
            (&9, &0),
        );

        for (key, value) in &map {
            v.push((key, value));
        }
        v.sort();

        assert_eq!(correct_v, v);
    }

    #[test]
    fn borrow_loop_over_mut_default_hashmap() {
        let mut map: DefaultHashMap<i8, u8> = defaulthashmap!(
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (7, 0),
            (8, 0),
            (9, 0),
        );

        let mut v: Vec<(&i8, &u8)> = vec!();
        let correct_v: Vec<(&'static i8, &'static u8)> = vec!(
            (&0, &1),
            (&1, &1),
            (&2, &1),
            (&3, &1),
            (&4, &1),
            (&5, &1),
            (&6, &1),
            (&7, &1),
            (&8, &1),
            (&9, &1),
        );

        for (key, value) in &mut map {
            *value += 1;
            v.push((key, value));
        }
        v.sort();

        assert_eq!(correct_v, v);
    }


    #[test]
    fn index_hashmap() {
        let map: DefaultHashMap<i8, i8> = defaulthashmap!(1,2,3,);
        let val = map[&1];

        assert_eq!(0, val);
    }

    #[test]
    #[should_panic]
    fn index_hashmap_panic() {
        let map: DefaultHashMap<i8, i8> = defaulthashmap!(1,2,3,);

        // this should panic since there is no 0 key
        let _ = map[&0];
    }

    #[test]
    fn macro_test_hashmap() {
        let map: DefaultHashMap<i8, i8> = defaulthashmap!(1,2,3,);

        let map1: DefaultHashMap<i8, i8> = defaulthashmap!(
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
        );

        let mut _map: DefaultHashMap<i8, i8> = DefaultHashMap::new();
        for i in 1..4 {
            let _ = _map.get(&i);
        }

        let mut _map1: DefaultHashMap<i8, i8> = DefaultHashMap::new();
        for i in 1..5 {
            let _ = _map1.insert(i, i);
        }

        assert_eq!(map, _map);
        assert_eq!(map1, _map1);
    }

    #[test]
    fn macro_test_slight_change_hashmap() {
        let map: DefaultHashMap<i8, i8> = defaulthashmap!(1, 2, 3);

        let map1: DefaultHashMap<i8, i8> = defaulthashmap!(
            (1,1),
            (2,2),
            (3,3),
            (4,4)
        );

        let mut _map: DefaultHashMap<i8, i8> = DefaultHashMap::new();
        for i in 1..4 {
            let _ = _map.get(&i);
        }

        let mut _map1: DefaultHashMap<i8, i8> = DefaultHashMap::new();
        for i in 1..5 {
            let _ = _map1.insert(i, i);
        }

        assert_eq!(map, _map);
        assert_eq!(map1, _map1);
    }



    // start btree tests

    #[test]
    fn default_value_i8_btree() {
        let mut map = DefaultBTreeMap::<i8, i8>::new();
        assert_eq!(map.get(&1), &0);
    }

    #[test]
    fn default_new_btree() {
        let map1: DefaultBTreeMap<i8, i8> = DefaultBTreeMap::new();

        let map2: DefaultBTreeMap<i8, i8> = DefaultBTreeMap::default();

        assert_eq!(map1, map2);
    }
    #[test]
    fn non_default_value_i8_btree() {
        let mut map = DefaultBTreeMap::<i8, i8>::new();
        map.insert(1,2);
        assert_eq!(map.get(&1), &2);
    }

    #[test]
    fn insert_remove_default_btree() {
        let mut map = DefaultBTreeMap::<i8, i8>::new();
        map.insert(1,123);
        let _ = map.remove(&1);
        assert_eq!(map.get(&1), &0);
    }

    #[test]
    fn remove_default_btree() {
        let mut map = DefaultBTreeMap::<i8, i8>::new();
        let i = map.remove(&1);
        assert_eq!(i, 0);
    }

    #[test]
    fn collect_keys_btree() {
        let mut map = DefaultBTreeMap::<i8, i8>::new();
        map.insert(1,2);
        map.insert(3,4);

        let mut keys: Vec<&i8> = map.keys().collect();
        keys.sort();

        assert_eq!(keys, vec!(&1, &3));
    }

    #[test]
    fn collect_values_btree() {
        let mut map = DefaultBTreeMap::<i8, i8>::new();
        map.insert(1,2);
        map.insert(3,4);

        let mut values: Vec<&i8> = map.values().collect();
        values.sort();

        assert_eq!(values, vec!(&2, &4));
    }

    #[test]
    fn check_len_btree() {
        let mut map = DefaultBTreeMap::<i8, i8>::new();
        map.insert(1,2);
        map.insert(3,4);

        assert_eq!(map.len(), 2);
    }

    #[test]
    fn contains_key_btree() {
        let mut map = DefaultBTreeMap::<i8, i8>::new();
        map.insert(1,2);
        map.insert(3,4);

        assert_eq!(map.contains_key(&1), true);
        assert_eq!(map.contains_key(&2), false);
    }

    #[test]
    fn get_mut_btree() {
        let mut map = DefaultBTreeMap::<i8, Vec<i8>>::new();

        let v1 = map.get_mut(&1);

        v1.push(10);

        assert_eq!(map.get(&1), &vec!(10));
    }

    #[test]
    fn retain_btree() {
        let mut map: DefaultBTreeMap<i8, u8> = defaultbtreemap!(
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
        );

        map.retain(|key, _| {
            key <= &2
        });

        let correct_map: DefaultBTreeMap<i8, u8> = defaultbtreemap!(
            (1, 1),
            (2, 2),
        );

        assert_eq!(correct_map, map);
    }

    #[test]
    fn is_empty_btree() {
        let mut map = DefaultBTreeMap::<i8, Vec<i8>>::new();

        let _ = map.get_mut(&1);

        let map1 = DefaultBTreeMap::<i8, Vec<i8>>::new();

        assert_eq!(map.is_empty(), false);
        assert_eq!(map1.is_empty(), true);

    }

    #[test]
    fn into_iter_no_default_btreemap() {
        let map: DefaultBTreeMap<i8, u8> = defaultbtreemap!(
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
        );

        let v: Vec<(i8, u8)> = map.into_iter().collect();

        let correct_v: Vec<(i8, u8)> = vec!(
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
        );

        assert_eq!(v, correct_v);
    }

    #[test]
    fn into_iter_default_btree() {
        let mut map: DefaultBTreeMap<i8, u8> = DefaultBTreeMap::new();

        for i in 0..10 {
            let _  = map.get(&i);
        }

        let mut v: Vec<(i8, u8)> = map.into_iter().collect();
        let mut correct_v: Vec<(i8, u8)> = vec!();
        v.sort();

        for i in 0..10 {
            correct_v.push((i as i8, 0 as u8));
        }

        assert_eq!(v, correct_v);
    }

    #[test]
    fn borrow_loop_over_default_btree() {
        let mut map: DefaultBTreeMap<i8, u8> = DefaultBTreeMap::new();

        for i in 0..10 {
            let _  = map.get(&i);
        }

        let mut v: Vec<(&i8, &u8)> = vec!();
        let correct_v: Vec<(&'static i8, &'static u8)> = vec!(
            (&0, &0),
            (&1, &0),
            (&2, &0),
            (&3, &0),
            (&4, &0),
            (&5, &0),
            (&6, &0),
            (&7, &0),
            (&8, &0),
            (&9, &0),
        );

        for (key, value) in &map {
            v.push((key, value));
        }

        assert_eq!(correct_v, v);
    }

    #[test]
    fn borrow_loop_over_mut_default_btree() {
        let mut map: DefaultBTreeMap<i8, u8> = defaultbtreemap!(
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (7, 0),
            (8, 0),
            (9, 0),
        );

        let mut v: Vec<(&i8, &u8)> = vec!();
        let correct_v: Vec<(&'static i8, &'static u8)> = vec!(
            (&0, &1),
            (&1, &1),
            (&2, &1),
            (&3, &1),
            (&4, &1),
            (&5, &1),
            (&6, &1),
            (&7, &1),
            (&8, &1),
            (&9, &1),
        );

        for (key, value) in &mut map {
            *value += 1;
            v.push((key, value));
        }
        v.sort();

        assert_eq!(correct_v, v);
    }

    #[test]
    fn index_btree() {
        let map: DefaultBTreeMap<i8, i8> = defaultbtreemap!(1,2,3,);
        let val = map[&1];

        assert_eq!(0, val);
    }

    #[test]
    #[should_panic]
    fn index_btree_panic() {
        let map: DefaultBTreeMap<i8, i8> = defaultbtreemap!(1,2,3,);

        // this should panic since there is no 0 key
        let _ = map[&0];
    }

    #[test]
    fn macro_test_btree() {
        let map: DefaultBTreeMap<i8, i8> = defaultbtreemap!(1,2,3,);

        let map1: DefaultBTreeMap<i8, i8> = defaultbtreemap!(
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
        );

        let mut _map: DefaultBTreeMap<i8, i8> = DefaultBTreeMap::new();
        for i in 1..4 {
            let _ = _map.get(&i);
        }

        let mut _map1: DefaultBTreeMap<i8, i8> = DefaultBTreeMap::new();
        for i in 1..5 {
            let _ = _map1.insert(i, i);
        }

        assert_eq!(map, _map);
        assert_eq!(map1, _map1);
    }

    #[test]
    fn macro_test_slight_change_btree() {
        let map: DefaultBTreeMap<i8, i8> = defaultbtreemap!(1, 2, 3);

        let map1: DefaultBTreeMap<i8, i8> = defaultbtreemap!(
            (1,1),
            (2,2),
            (3,3),
            (4,4)
        );

        let mut _map: DefaultBTreeMap<i8, i8> = DefaultBTreeMap::new();
        for i in 1..4 {
            let _ = _map.get(&i);
        }

        let mut _map1: DefaultBTreeMap<i8, i8> = DefaultBTreeMap::new();
        for i in 1..5 {
            let _ = _map1.insert(i, i);
        }

        assert_eq!(map, _map);
        assert_eq!(map1, _map1);
    }
}
