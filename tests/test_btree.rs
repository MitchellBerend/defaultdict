use defaultdict::*;

use std::collections::BTreeMap;


#[test]
fn clear_btree() {
    let mut map = DefaultBTreeMap::<i8, i8>::new();
    map.insert(1,123);
    map.clear();

    let correct_map = DefaultBTreeMap::<i8, i8>::new();
    assert_eq!(correct_map, map);

}

#[test]
fn split_of_hashbtree() {
    let mut map1 = DefaultBTreeMap::<i8, i8>::new();

    for i in 0..10 {
        map1.insert(i, i);
    }

    let map2 = map1.split_off(&5);

    let correct_map1: DefaultBTreeMap<i8, i8> = defaultbtreemap!(
        (0, 0),
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4),
    );

    let correct_map2: DefaultBTreeMap<i8, i8> = defaultbtreemap!(
        (5, 5),
        (6, 6),
        (7, 7),
        (8, 8),
        (9, 9),
    );

    assert_eq!(correct_map1, map1);
    assert_eq!(correct_map2, map2);
}
#[test]
fn value_mut_btree() {
    let mut map: DefaultBTreeMap<i8, String> = defaultbtreemap!();

    for i in 0..10 {
        let _ = map.insert(i, format!("{}", i));
    }

    for value in map.values_mut() {
        value.push('a');
    }

    let correct_v: Vec<(i8, String)> = vec!(
        (0, String::from("0a")),
        (1, String::from("1a")),
        (2, String::from("2a")),
        (3, String::from("3a")),
        (4, String::from("4a")),
        (5, String::from("5a")),
        (6, String::from("6a")),
        (7, String::from("7a")),
        (8, String::from("8a")),
        (9, String::from("9a")),
    );

    let map_v: Vec<(i8, String)> = map.into_iter().collect();

    assert_eq!(correct_v, map_v);
}

#[test]
fn remove_entry_btree() {
    let mut map: DefaultBTreeMap<i8, String> = defaultbtreemap!();

    for i in 0..10 {
        let _ = map.insert(i, format!("{}", i));
    }

    let _ = map.remove_entry(&0);
    let _ = map.remove_entry(&5);
    let default_entry: (i8, String) = map.remove_entry(&0);

    let mut map_v: Vec<(i8, String)> = map.into_iter().collect();
    map_v.sort();

    let correct_v: Vec<(i8, String)> = vec!(
        //(0, String::from("0")), this entry is removed
        (1, String::from("1")),
        (2, String::from("2")),
        (3, String::from("3")),
        (4, String::from("4")),
        //(5, String::from("5")), this entry is removed
        (6, String::from("6")),
        (7, String::from("7")),
        (8, String::from("8")),
        (9, String::from("9")),
    );
    let correct_entry: (i8, String) = (0, "".into());

    assert_eq!(correct_v, map_v);
    assert_eq!(correct_entry, default_entry);
}

#[test]
fn append_btree() {
    let mut map1 = DefaultBTreeMap::<i8, i8>::new();
    let mut map2 = DefaultBTreeMap::<i8, i8>::new();

    for i in 0..10 {
        if i % 2 == 0 {
            map1.insert(i, i);
        } else {
            map2.insert(i, i);
        }
    }

    map1.append(&mut map2)
}

#[test]
fn get_key_value_btree() {
    let map: DefaultBTreeMap<i8, String> = defaultbtreemap!(
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
fn entry_btree() {
    use std::collections::btree_map::Entry;

    let mut map: DefaultBTreeMap<i8, i8> = defaultbtreemap!(
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

    if let  Entry::Vacant(entry) = map.entry(3) {
        entry.insert(3);
    }

    assert_eq!(&3, map.get(&3));
}

#[test]
fn into_keys_btree() {
    let map: DefaultBTreeMap<i8, String> = defaultbtreemap!(
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

    let correct_v: Vec<i8> = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);

    let mut vec_map: Vec<i8> = map.into_keys().collect();
    vec_map.sort();

    assert_eq!(correct_v, vec_map);
}

#[test]
fn into_values_btree() {
    let map: DefaultBTreeMap<i8, String> = defaultbtreemap!(
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

    let correct_v: Vec<String> = vec!(
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
    );

    let mut vec_map: Vec<String> = map.into_values().collect();
    vec_map.sort();

    assert_eq!(correct_v, vec_map);
}

#[test]
fn range_btree() {
    let map: DefaultBTreeMap<i8, i8> = defaultbtreemap!(
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

    let mut index: i8 = 0;
    for (&key, &val) in map.range(0..10) {
        assert_eq!(&index, &key);
        assert_eq!(&index, &val);
        index += 1
    }
}

#[test]
fn range_mut_btree() {
    let mut map: DefaultBTreeMap<i8, i8> = defaultbtreemap!(
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

    let mut index: i8 = 0;
    for (&key, &val) in map.range(0..10) {
        assert_eq!(&index, &key);
        assert_eq!(&index, &val);
        index += 1
    }

    index = 0;

    for (&_key, val) in map.range_mut(0..10) {
        *val += 100;
        index += 1;
    }

    index = 0;

    for (&key, &val) in map.range(0..10) {
        let comparison = index + 100;
        assert_eq!(&index, &key);
        assert_eq!(&comparison, &val);
        index += 1
    }
}

#[test]
fn default_value_i8_btree() {
    let map = DefaultBTreeMap::<i8, i8>::new();
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
        let _  = map.get_mut(&i);
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
        let _  = map.get_mut(&i);
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
fn index_btree_no_panic() {
    let map: DefaultBTreeMap<i8, i8> = defaultbtreemap!(1,2,3,);

    let default_value = map[&0];

    assert_eq!(0, default_value);
}

#[test]
fn btree_into_defaultbtree() {
    let mut btree: BTreeMap<u8, i8> = BTreeMap::new();

    for i in 0..10 {
        btree.insert(i as u8, i as i8);
    };

    let map: DefaultBTreeMap<u8, i8> = btree.into();

    let correct_map: DefaultBTreeMap<u8, i8> = defaultbtreemap!(
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
fn defaultbtree_into_btree() {
    let map: DefaultBTreeMap<u8, i8> = defaultbtreemap!(
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
    let btree: BTreeMap<u8, i8> = map.into();
    let mut correct_btree: BTreeMap<u8, i8> = BTreeMap::new();

    for i in 0..10 {
        correct_btree.insert(i as u8, i as i8);
    };

    assert_eq!(correct_btree, btree);
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
        let _ = _map.get_mut(&i);
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
        let _ = _map.get_mut(&i);
    }

    let mut _map1: DefaultBTreeMap<i8, i8> = DefaultBTreeMap::new();
    for i in 1..5 {
        let _ = _map1.insert(i, i);
    }

    assert_eq!(map, _map);
    assert_eq!(map1, _map1);
}
