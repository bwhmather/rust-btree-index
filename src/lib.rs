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

impl<K, V> BTreeIndex<K, V> {
    pub fn new() -> BTreeIndex<K, V> {
        unimplemented!();
    }

    pub fn clean(&mut self) {
        unimplemented!();
    }

    pub fn get<'a>(&'a self, key: K) -> ValueSet<'a, K, V> {
        unimplemented!();
    }

    pub fn insert(&mut self, key: K, value: V) {
        unimplemented!();
    }

    pub fn remove(&mut self, key: K, value: V) {
        unimplemented!();
    }
}

impl<K, V> BTreeIndex<K, V> {
    pub fn iter(&self) -> Iter<K, V> {
        unimplemented!();
    }

    pub fn keys<'a>(&'a self) -> Keys<'a, K, V> {
        unimplemented!();
    }

    pub fn values<'a>(&'a self) -> Values<'a, K, V> {
        unimplemented!();
    }

    pub fn len(&self) -> usize {
        unimplemented!();
    }

    pub fn is_empty(&self) -> bool {
        unimplemented!();
    }
}

impl<K, V> Default for BTreeIndex<K, V> {
    fn default() -> BTreeIndex<K, V> {
        unimplemented!();
    }
}

impl<K, V> Extend<(K, V)> for BTreeIndex<K, V>
where K: Ord {
    fn extend<T>(&mut self, iter: T)
    where T: IntoIterator<Item = (K, V)> {
        unimplemented!();
    }
}

impl<'a, K, V> Extend<(&'a K, &'a V)> for BTreeIndex<K, V>
where K: Copy + Ord, V: Copy {
    fn extend<T>(&mut self, iter: T)
    where T: IntoIterator<Item = (&'a K, &'a V)> {
        unimplemented!();
    }
}

impl<'a, K, V> IntoIterator for &'a BTreeIndex<K, V>
where K: 'a, V: 'a {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Iter<'a, K, V> {
        unimplemented!();
    }
}

impl<K, V> IntoIterator for BTreeIndex<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> IntoIter<K, V> {
        unimplemented!();
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
