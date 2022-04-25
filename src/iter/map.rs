use std::collections::HashMap;
use std::hash::Hash;

pub trait MapUtils {
    type Item;

    fn associate_to_map<K, V, F>(self, transform: F) -> HashMap<K, V>
        where
            F: Fn(Self::Item) -> (K, V),
            K: Eq,
            K: Hash;

    fn associate_with_to_map<B, F>(self, transform: F) -> HashMap<Self::Item, B>
        where
            F: Fn(Self::Item) -> B;
}

impl<I, T> MapUtils for I
    where I: IntoIterator<Item=T>,
          T: Hash,
          T: Eq,
          T: Clone {
    type Item = T;

    fn associate_to_map<K, V, F>(self, transform: F) -> HashMap<K, V>
        where
            F: Fn(Self::Item) -> (K, V),
            K: Eq,
            K: Hash {
        self.into_iter()
            .map(|it| transform(it))
            .collect()
    }

    fn associate_with_to_map<B, F>(self, transform: F) -> HashMap<Self::Item, B>
        where
            F: Fn(Self::Item) -> B {
        self.into_iter()
            .map(|it| (it.clone(), transform(it)))
            .collect()
    }
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::iter::map::MapUtils;

    #[test]
    fn test_associate_to_map() {
        let v = vec![1, 2];
        let v = v.associate_to_map(|it| (it, it));
        assert_eq!(v, HashMap::from([(1, 1), (2, 2)]));
    }


    #[test]
    fn test_associate_with_to_map() {
        let v = vec![1, 2];
        let v = v.associate_with_to_map(|it| it + 1);
        assert_eq!(v, HashMap::from([(1, 2), (2, 3)]))
    }
}
