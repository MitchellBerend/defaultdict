use defaultdict::*;

use std::collections::HashMap;

#[test]
fn values_mut_hashmap() {
    let mut map = DefaultHashMap::<i8, i8>::new();

    for i in 0..10 {
        map.insert(i, i);
    }

    for value in map.values_mut() {
        *value += 10;
    }

    let correct_v: Vec<(i8, i8)> = vec![
        (0, 10),
        (1, 11),
        (2, 12),
        (3, 13),
        (4, 14),
        (5, 15),
        (6, 16),
        (7, 17),
        (8, 18),
        (9, 19),
    ];

    let mut map_v: Vec<(i8, i8)> = map.into_iter().collect();
    map_v.sort();

    assert_eq!(correct_v, map_v);
}

#[test]
fn remove_entry_hashmap() {
    let mut map = DefaultHashMap::<i8, i8>::new();

    for i in 0..10 {
        map.insert(i, i);
    }

    let _ = map.remove_entry(&5);
    let default_entry: (i8, i8) = map.remove_entry(&5);

    let mut map_v: Vec<(i8, i8)> = map.into_iter().collect();
    map_v.sort();

    let correct_v: Vec<(i8, i8)> = vec![
        (0, 0),
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4),
        //(5, 5), this entry is removed
        (6, 6),
        (7, 7),
        (8, 8),
        (9, 9),
    ];

    let correct_entry: (i8, i8) = (5, 0);

    assert_eq!(correct_v, map_v);
    assert_eq!(correct_entry, default_entry);
}

#[test]
fn clear_hashmap() {
    let mut map = DefaultHashMap::<i8, i8>::new();
    map.insert(1, 123);
    map.clear();

    let correct_map = DefaultHashMap::<i8, i8>::new();
    assert_eq!(correct_map, map);
}

#[test]
fn capacity_hashmap() {
    let mut map = DefaultHashMap::<i8, i8>::new();
    for i in 0..10 {
        map.insert(i, i);
    }
    map.clear();

    assert_eq!(map.capacity(), 14);
}

#[test]
fn drain_hashmap() {
    let mut map = DefaultHashMap::<i8, i8>::new();
    for i in 0..10 {
        map.insert(i, i);
    }
    let mut drain: Vec<(i8, i8)> = map.drain().collect();
    drain.sort();
    let correct_v: Vec<(i8, i8)> = vec![
        (0, 0),
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 5),
        (6, 6),
        (7, 7),
        (8, 8),
        (9, 9),
    ];

    assert_eq!(correct_v, drain);
}

#[test]
fn entry_hashmap() {
    use std::collections::hash_map::Entry;

    let mut map: DefaultHashMap<i8, i8> = defaulthashmap!(
        (0, 0),
        (1, 1),
        (2, 2),
        // (3, 3), missing entry
        (4, 4),
        (5, 5),
        (6, 6),
        (7, 7),
        (8, 8),
        (9, 9),
    );

    assert_eq!(&0, map.get(&3));

    if let Entry::Vacant(entry) = map.entry(3) {
        entry.insert(3);
    }

    assert_eq!(&3, map.get(&3));
}

#[test]
fn get_key_value_hashmap() {
    let map: DefaultHashMap<i8, String> = defaulthashmap!(
        (0, String::from("0")),
        (1, String::from("1")),
        (2, String::from("2")),
        (3, String::from("3")),
        (4, String::from("4")),
        (5, String::from("5")),
        (6, String::from("6")),
        (7, String::from("7")),
        (8, String::from("8")),
        (9, String::from("9")),
    );

    for i in 0..10 {
        let key_value = map.get_key_value(&i);
        let check = (&i, &format!("{}", i));

        assert_eq!(check, key_value);
    }
}

#[test]
fn into_keys_hashmap() {
    let map: DefaultHashMap<i8, String> = defaulthashmap!(
        (0, String::from("0")),
        (1, String::from("1")),
        (2, String::from("2")),
        (3, String::from("3")),
        (4, String::from("4")),
        (5, String::from("5")),
        (6, String::from("6")),
        (7, String::from("7")),
        (8, String::from("8")),
        (9, String::from("9")),
    );

    let correct_v: Vec<i8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut vec_map: Vec<i8> = map.into_keys().collect();
    vec_map.sort();

    assert_eq!(correct_v, vec_map);
}

#[test]
fn into_values_hashmap() {
    let map: DefaultHashMap<i8, String> = defaulthashmap!(
        (0, String::from("0")),
        (1, String::from("1")),
        (2, String::from("2")),
        (3, String::from("3")),
        (4, String::from("4")),
        (5, String::from("5")),
        (6, String::from("6")),
        (7, String::from("7")),
        (8, String::from("8")),
        (9, String::from("9")),
    );

    let correct_v: Vec<String> = vec![
        String::from("0"),
        String::from("1"),
        String::from("2"),
        String::from("3"),
        String::from("4"),
        String::from("5"),
        String::from("6"),
        String::from("7"),
        String::from("8"),
        String::from("9"),
    ];

    let mut vec_map: Vec<String> = map.into_values().collect();
    vec_map.sort();

    assert_eq!(correct_v, vec_map);
}

#[test]
fn default_value_i8_hashmap() {
    let map = DefaultHashMap::<i8, i8>::new();
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
    map.insert(1, 2);
    assert_eq!(map.get(&1), &2);
}

#[test]
fn insert_remove_default_hashmap() {
    let mut map = DefaultHashMap::<i8, i8>::new();
    map.insert(1, 123);
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
    map.insert(1, 2);
    map.insert(3, 4);

    let mut keys: Vec<&i8> = map.keys().collect();
    keys.sort();

    assert_eq!(keys, vec!(&1, &3));
}

#[test]
fn collect_values_hashmap() {
    let mut map = DefaultHashMap::<i8, i8>::new();
    map.insert(1, 2);
    map.insert(3, 4);

    let mut values: Vec<&i8> = map.values().collect();
    values.sort();

    assert_eq!(values, vec!(&2, &4));
}

#[test]
fn check_len_hashmap() {
    let mut map = DefaultHashMap::<i8, i8>::new();
    map.insert(1, 2);
    map.insert(3, 4);

    assert_eq!(map.len(), 2);
}

#[test]
fn contains_key_hashmap() {
    let mut map = DefaultHashMap::<i8, i8>::new();
    map.insert(1, 2);
    map.insert(3, 4);

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
    let mut map: DefaultHashMap<i8, u8> = defaulthashmap!((1, 1), (2, 2), (3, 3), (4, 4),);

    map.retain(|key, _| key <= &2);

    let correct_map: DefaultHashMap<i8, u8> = defaulthashmap!((1, 1), (2, 2),);

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
    let map: DefaultHashMap<i8, u8> = defaulthashmap!((1, 1), (2, 2), (3, 3), (4, 4),);

    let mut v: Vec<(i8, u8)> = map.into_iter().collect();
    v.sort();

    let correct_v: Vec<(i8, u8)> = vec![(1, 1), (2, 2), (3, 3), (4, 4)];

    assert_eq!(v, correct_v);
}

#[test]
fn into_iter_default_hashmap() {
    let mut map: DefaultHashMap<i8, u8> = DefaultHashMap::new();

    for i in 0..10 {
        let _ = map.get_mut(&i);
    }

    let mut v: Vec<(i8, u8)> = map.into_iter().collect();
    let mut correct_v: Vec<(i8, u8)> = vec![];
    v.sort();

    for i in 0..10 {
        correct_v.push((i as i8, 0_u8));
    }

    assert_eq!(v, correct_v);
}

#[test]
fn from_iter_hashmap() {
    let data = [(1, 1), (2, 2), (3, 3), (4, 4)];
    let map: DefaultHashMap<i8, u8> = data.iter().cloned().collect();

    let correct_map: DefaultHashMap<i8, u8> = defaulthashmap!((1, 1), (2, 2), (3, 3), (4, 4));

    assert_eq!(map, correct_map);
}

#[test]
fn borrow_loop_over_default_hashmap() {
    let mut map: DefaultHashMap<i8, u8> = DefaultHashMap::new();

    for i in 0..10 {
        let _ = map.get_mut(&i);
    }

    let mut v: Vec<(&i8, &u8)> = vec![];
    let correct_v: Vec<(&'static i8, &'static u8)> = vec![
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
    ];

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

    let mut v: Vec<(&i8, &u8)> = vec![];
    let correct_v: Vec<(&'static i8, &'static u8)> = vec![
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
    ];

    for (key, value) in &mut map {
        *value += 1;
        v.push((key, value));
    }
    v.sort();

    assert_eq!(correct_v, v);
}

#[test]
fn index_hashmap() {
    let map: DefaultHashMap<i8, i8> = defaulthashmap!(1, 2, 3,);
    let val = map[&1];

    assert_eq!(0, val);
}

#[test]
fn index_hashmap_no_panic() {
    let map: DefaultHashMap<i8, i8> = defaulthashmap!(1, 2, 3,);

    // this should panic since there is no 0 key
    let default_value = map[&0];

    assert_eq!(0, default_value);
}

#[test]
fn hashmap_into_defaulthashmap() {
    let mut hashmap: HashMap<u8, i8> = HashMap::new();

    for i in 0..10 {
        hashmap.insert(i as u8, i as i8);
    }

    let map: DefaultHashMap<u8, i8> = hashmap.into();

    let correct_map: DefaultHashMap<u8, i8> = defaulthashmap!(
        (0, 0),
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 5),
        (6, 6),
        (7, 7),
        (8, 8),
        (9, 9),
    );

    assert_eq!(correct_map, map);
}

#[test]
fn defaulthashmap_into_hashmap() {
    let map: DefaultHashMap<u8, i8> = defaulthashmap!(
        (0, 0),
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 5),
        (6, 6),
        (7, 7),
        (8, 8),
        (9, 9),
    );
    let hashmap: HashMap<u8, i8> = map.into();
    let mut correct_hashmap: HashMap<u8, i8> = HashMap::new();

    for i in 0..10 {
        correct_hashmap.insert(i as u8, i as i8);
    }

    assert_eq!(correct_hashmap, hashmap);
}

#[test]
fn macro_test_hashmap() {
    let map: DefaultHashMap<i8, i8> = defaulthashmap!(1, 2, 3,);

    let map1: DefaultHashMap<i8, i8> = defaulthashmap!((1, 1), (2, 2), (3, 3), (4, 4),);

    let mut _map: DefaultHashMap<i8, i8> = DefaultHashMap::new();
    for i in 1..4 {
        let _ = _map.get_mut(&i);
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

    let map1: DefaultHashMap<i8, i8> = defaulthashmap!((1, 1), (2, 2), (3, 3), (4, 4));

    let mut _map: DefaultHashMap<i8, i8> = DefaultHashMap::new();
    for i in 1..4 {
        let _ = _map.get_mut(&i);
    }

    let mut _map1: DefaultHashMap<i8, i8> = DefaultHashMap::new();
    for i in 1..5 {
        let _ = _map1.insert(i, i);
    }

    assert_eq!(map, _map);
    assert_eq!(map1, _map1);
}
