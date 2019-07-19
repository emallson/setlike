#[cfg(feature = "bit-set")]
use bit_set::BitSet;
#[cfg(feature = "hashbrown")]
use hashbrown::HashSet as HashBrownSet;
use std::collections::{BTreeSet, HashSet};
use std::hash::{BuildHasher, Hash};
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

/// A Set-like object.
pub trait Setlike<T: Sized> {
    /// `true` if the set contains `el`.
    fn contains(&self, el: &T) -> bool;

    /// Insert the given element.
    ///
    /// Returns `true` if the element was *not* already in the set. If it was, `false` instead.
    fn insert(&mut self, el: T) -> bool;

    /// Remove the the given element.
    ///
    /// Returns `true` if the set contained the element.
    fn remove(&mut self, el: &T) -> bool;

    /// The number of elements in the set.
    fn len(&self) -> usize;

    /// Create an instance of the setlike with a hint that we will need room for `k` elements.
    ///
    /// Not all implementations support this operation; those that do not will simply create an
    /// empty instance.
    fn with_capacity(k: usize) -> Self
    where
        Self: Sized;
}

impl<T: Sized + Eq + Hash, S: BuildHasher + Default> Setlike<T> for HashSet<T, S> {
    fn contains(&self, el: &T) -> bool {
        self.contains(el)
    }

    fn insert(&mut self, el: T) -> bool {
        self.insert(el)
    }

    fn remove(&mut self, el: &T) -> bool {
        self.remove(el)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn with_capacity(k: usize) -> Self {
        HashSet::with_capacity_and_hasher(k, S::default())
    }
}

impl<T: Sized + Ord> Setlike<T> for BTreeSet<T> {
    fn len(&self) -> usize {
        self.len()
    }

    fn contains(&self, el: &T) -> bool {
        self.contains(el)
    }

    fn insert(&mut self, el: T) -> bool {
        self.insert(el)
    }

    fn remove(&mut self, el: &T) -> bool {
        self.remove(el)
    }

    fn with_capacity(_k: usize) -> Self {
        Self::new()
    }
}

#[cfg(feature = "bit-set")]
impl Setlike<usize> for BitSet {
    fn insert(&mut self, i: usize) -> bool {
        self.insert(i)
    }

    fn remove(&mut self, i: &usize) -> bool {
        self.remove(*i)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn contains(&self, i: &usize) -> bool {
        self.contains(*i)
    }

    fn with_capacity(k: usize) -> Self {
        BitSet::with_capacity(k)
    }
}

#[cfg(feature = "hashbrown")]
impl<T: Eq + Hash, S: BuildHasher + Default> Setlike<T> for HashBrownSet<T, S> {
    fn insert(&mut self, i: T) -> bool {
        self.insert(i)
    }

    fn remove(&mut self, i: &T) -> bool {
        self.remove(i)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn contains(&self, i: &T) -> bool {
        self.contains(i)
    }

    fn with_capacity(k: usize) -> Self {
        HashBrownSet::with_capacity_and_hasher(k, S::default())
    }
}

#[cfg(test)]
mod test {

    macro_rules! set_test {
        ($id:ident, $t:ty, $e:ty, $($setup:item),*) => {
            mod $id {
                use super::super::*;
                $($setup)+

                quickcheck! {
                    fn contains_after_insert(set: $t, u: $e) -> bool {
                        let mut set = set;
                        let s: &mut Setlike<$e> = &mut set;
                        s.insert(u);
                        s.contains(&u)
                    }

                    fn not_contains_after_remove(set: $t, u: $e) -> bool {
                        let mut set = set;
                        let s: &mut Setlike<$e> = &mut set;
                        s.insert(u);
                        let contained = s.contains(&u);
                        s.remove(&u);
                        contained && ! s.contains(&u)
                    }

                    fn insert_twice(set: $t, u: $e) -> bool {
                        let mut set = set;
                        let s: &mut Setlike<$e> = &mut set;
                        s.insert(u);
                        !s.insert(u)
                    }

                    fn remove_twice(set: $t, u: $e) -> bool {
                        let mut set = set;
                        let s: &mut Setlike<$e> = &mut set;
                        s.insert(u);
                        s.remove(&u) && !s.remove(&u)
                    }

                    fn len_increments(set: $t, u: $e) -> bool {
                        let mut set = set;
                        let s: &mut Setlike<$e> = &mut set;
                        let l = s.len();
                        // either u is already in s, or s has its length increased
                        !s.insert(u) || s.len() == l + 1
                    }
                }
            }
        }
    }

    set_test!(
        hash_set,
        HashSet<usize>,
        usize,
        use std::collections::HashSet;
    );

    set_test!(
        btree_set,
        BTreeSet<usize>,
        usize,
        use std::collections::BTreeSet;
    );

    // bit-set and hashbrown don't impl Arbitrary and I can't devote time to wrapping right now
}
