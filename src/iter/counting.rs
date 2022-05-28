use std::collections::HashMap;
use std::hash::Hash;

pub trait Countable {
    type Item;

    fn each_count(self) -> HashMap<Self::Item, u64>
    where
        Self::Item: Hash + Eq;

    fn each_count_by<K>(self, selector: fn(Self::Item) -> K) -> HashMap<K, u64>
    where
        K: Eq + Hash;
}

impl<I, T> Countable for I
where
    I: IntoIterator<Item = T>,
{
    type Item = T;

    fn each_count(self) -> HashMap<Self::Item, u64>
    where
        Self::Item: Hash + Eq,
    {
        let mut hm: HashMap<Self::Item, u64> = HashMap::new();

        for item in self.into_iter() {
            let a = hm.get(&item).unwrap_or(&0_u64);
            let new = a + 1;
            hm.insert(item, new);
        }
        hm
    }

    fn each_count_by<K>(self, selector: fn(Self::Item) -> K) -> HashMap<K, u64>
    where
        K: Eq + Hash,
    {
        let mut hm: HashMap<K, u64> = HashMap::new();

        for item in self.into_iter().map(|it| selector(it)) {
            let a = hm.get(&item).unwrap_or(&0_u64);
            let new = a + 1;
            hm.insert(item, new);
        }
        hm
    }
}

#[cfg(test)]
mod test {
    use crate::iter::counting::Countable;
    use std::collections::HashMap;

    struct Test {
        a: i64,
    }

    #[test]
    fn test_each_count() {
        let v = vec![1, 1, 1, 2, 3, 3, 3, 3];
        let v = v.each_count();

        assert_eq!(v, HashMap::from([(1, 3), (2, 1), (3, 4)]))
    }

    #[test]
    fn test_each_count_by() {
        let v = vec![
            Test { a: 1 },
            Test { a: 1 },
            Test { a: 1 },
            Test { a: 2 },
            Test { a: 3 },
            Test { a: 3 },
            Test { a: 3 },
            Test { a: 3 },
        ];
        let v = v.each_count_by(|it| it.a);
        assert_eq!(v, HashMap::from([(1, 3), (2, 1), (3, 4)]))
    }
}
