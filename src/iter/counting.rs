use std::collections::HashMap;
use std::hash::Hash;

pub trait Countable {
    type Item;

    fn each_count(self) -> HashMap<Self::Item, u64>;
}


impl<I, T> Countable for I
    where I: IntoIterator<Item=T>,
          T: Hash + Eq {
    type Item = T;

    fn each_count(self) -> HashMap<Self::Item, u64> {
        let mut hm: HashMap<Self::Item, u64> = HashMap::new();

        for item in self.into_iter() {
            let a = hm.get(&item).unwrap_or(&0_u64);
            let new = a + 1;
            hm.insert(item, new);
        }
        hm
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::iter::counting::Countable;

    #[test]
    fn test_each_count() {
        let v = vec![1, 1, 1, 2, 3, 3, 3, 3];
        let v = v.each_count();

        assert_eq!(v, HashMap::from([(1, 3), (2, 1), (3, 4)]))
    }
}
