//! This library exposes a struct that mimicks the behaviour of the python 
//! [defaultdict](https://docs.python.org/3/library/collections.html#collections.defaultdict).
//! 
//! This behaviour does require that the type of the value does have the [`Default`] implemented.


mod default_dict;
pub use default_dict::DefaultHashMap;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_value_i8() {
        let mut map = DefaultHashMap::<i8, i8>::new();
        assert_eq!(map.get(&1), &0);
    }

    #[test]
    fn non_default_value_i8() {
        let mut map = DefaultHashMap::<i8, i8>::new();
        map.insert(1,2);
        assert_eq!(map.get(&1), &2);
    }

    #[test]
    fn insert_remove_default() {
        let mut map = DefaultHashMap::<i8, i8>::new();
        map.insert(1,123);
        let _ = map.remove(&1);
        assert_eq!(map.get(&1), &0);
    }

    #[test]
    fn remove_default() {
        let mut map = DefaultHashMap::<i8, i8>::new();
        let i = map.remove(&1);
        assert_eq!(i, 0);
    }
    
    #[test]
    fn collect_keys() {
        let mut map = DefaultHashMap::<i8, i8>::new();
        map.insert(1,2);
        map.insert(3,4);

        let mut keys: Vec<&i8> = map.keys().collect();
        keys.sort();

        assert_eq!(keys, vec!(&1, &3));
    }

    #[test]
    fn collect_values() {
        let mut map = DefaultHashMap::<i8, i8>::new();
        map.insert(1,2);
        map.insert(3,4);

        let mut values: Vec<&i8> = map.values().collect();
        values.sort();

        assert_eq!(values, vec!(&2, &4));
    }

    #[test]
    fn check_len() {
        let mut map = DefaultHashMap::<i8, i8>::new();
        map.insert(1,2);
        map.insert(3,4);

        assert_eq!(map.len(), 2);
    }

    #[test]
    fn contains_key() {
        let mut map = DefaultHashMap::<i8, i8>::new();
        map.insert(1,2);
        map.insert(3,4);

        assert_eq!(map.contains_key(&1), true);
        assert_eq!(map.contains_key(&2), false);
    }

    #[test]
    fn get_mut() {
        let mut map = DefaultHashMap::<i8, Vec<i8>>::new();
        
        let v1 = map.get_mut(&1);

        v1.push(10);

        assert_eq!(map.get(&1), &vec!(10));
    }

    #[test]
    fn macro_test() {
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
    fn macro_test_slight_change() {
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
}

