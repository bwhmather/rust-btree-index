use std::collections::btree_set;

use std::borrow::Borrow;

/// A one-to-many index datastructure built around the standard library b-tree.
#[derive(Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BTreeIndex<K, V> {
    set: btree_set::BTreeSet<(K, V)>,
}

/// An iterator over the entries of a [`BTreeIndex`].
///
/// This `struct` is created by the [`iter'] method on [`BTreeIndex`].  See its
/// documentation for more.
///
/// [`iter`]: struct.BTreeIndex.html#method.iter
/// [`BTreeIndex`]: struct.BTreeIndex.html
pub struct Iter<'a, K: 'a, V: 'a> {
    iter: btree_set::Iter<'a, (K, V)>,
}

/// An owning iterator over the entries of a [`BTreeIndex`].
///
/// [`BTreeIndex`]: struct.BTreeIndex.html
pub struct IntoIter<K, V> {
    iter: btree_set::IntoIter<(K, V)>,
}

/// An iterator over the keys of a [`BTreeIndex`].
///
/// [`BTreeIndex`]: struct.BTreeIndex.html
pub struct Keys<'a, K: 'a, V: 'a> {
    iter: btree_set::Iter<'a, (K, V)>,
}

/// An iterator over the values of a [`BTreeIndex`].
///
/// [`BTreeIndex`]: struct.BTreeIndex.html
pub struct Values<'a, K: 'a, V: 'a> {
    iter: btree_set::Iter<'a, (K, V)>,
}

/// A view on the values for a particular key in a [`BTreeIndex`]a.
///
/// [`BTreeIndex`]: struct.BTreeIndex.html
pub struct ValueSet<'a, K: 'a, V: 'a> {
    values: btree_set::Range<'a, (K, V)>,
}

impl<K: Ord, V: Ord> BTreeIndex<K, V> {
    pub fn new() -> BTreeIndex<K, V> {
        return BTreeIndex {
            set: btree_set::BTreeSet::new(),
        }
    }

    pub fn clear(&mut self) {
        self.set.clear();
    }

    pub fn get<'a>(&'a self, key: K) -> ValueSet<'a, K, V> {
        unimplemented!();
    }

    pub fn insert(&mut self, key: K, value: V) -> bool {
        return self.set.insert((key, value));
    }

    pub fn remove<Q: ?Sized, R: ?Sized>(&mut self, key: K, value: V) -> bool
    where K: Borrow<Q>, V: Borrow<R>, Q: Ord, R: Ord {
        return self.set.remove(&(key, value));
    }

    pub fn iter(&self) -> Iter<K, V> {
        return Iter {
            iter: self.set.iter(),
        };
    }

    pub fn keys<'a>(&'a self) -> Keys<'a, K, V> {
        return Keys {
            iter: self.set.iter(),
        };
    }

    pub fn values<'a>(&'a self) -> Values<'a, K, V> {
        return Values {
            iter: self.set.iter(),
        };
    }

    pub fn len(&self) -> usize {
        return self.set.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.set.is_empty();
    }
}

impl<K: Ord, V: Ord> Default for BTreeIndex<K, V> {
    fn default() -> BTreeIndex<K, V> {
        return BTreeIndex::new();
    }
}

impl<K: Ord, V: Ord> Extend<(K, V)> for BTreeIndex<K, V> {
    fn extend<T>(&mut self, iter: T)
    where T: IntoIterator<Item = (K, V)> {
        self.set.extend(iter);
    }
}

impl<'a, K, V> Extend<(&'a K, &'a V)> for BTreeIndex<K, V>
where K: Copy + Ord, V: Copy {
    fn extend<T>(&mut self, iter: T)
    where T: IntoIterator<Item = (&'a K, &'a V)> {
        unimplemented!();
    }
}

impl<'a, K: 'a + Ord, V: 'a + Ord> IntoIterator for &'a BTreeIndex<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Iter<'a, K, V> {
        return self.iter();
    }
}

impl<K, V> IntoIterator for BTreeIndex<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> IntoIter<K, V> {
        return IntoIter {
            iter: self.set.into_iter(),
        };
    }
}

impl<'a, K: 'a, V: 'a> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        unimplemented!();
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unimplemented!();
    }
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        unimplemented!();
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
