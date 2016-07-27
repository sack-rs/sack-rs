use sack::sackerror::SackError;
use std::fmt;
use sack::sack::{Sack, WritableSack, StreamWritableSack, BoundedIterable};
use stable_bst::TreeMap;
use std::hash::Hash;
use sack::sack::SackLike;
use std::fmt::Debug;
use stable_bst::Bound;
use stable_bst::map::Range;
use std::iter;

/// A KVSack is a concrete implementation of a Sack api that
/// just supports standard K/V operations (get and set for now)
/// While its data persistence and querability is not interesting,
/// it will be an example for all new sack-orchestration capabilities.
/// see kv_example.rs for a runnable example
#[derive(Clone,PartialOrd,Ord,PartialEq,Eq)]
pub struct KVSack<K: Ord + Clone, V: Ord + Clone>(Sack<K, V, TreeMap<K, V>>);

impl<K: PartialOrd + Ord + Clone, V: Ord + Clone> Default for KVSack<K, V> {
    fn default() -> Self {
        KVSack(Sack {
            state: None,
            payload: None,
            children: TreeMap::new(),
        })
    }
}

impl<K: Ord, V> SackLike<K, V> for TreeMap<K, V> {}

impl<K, V> Iterator for KVSackIterator<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<(K, V)> {
        self.0.next()
    }
}

impl<K: 'static + Ord + Clone, V: 'static + Clone + Ord> iter::IntoIterator for KVSack<K, V> {
    type IntoIter = KVSackIterator<K, V>;
    type Item = (K, V);
    fn into_iter(self) -> KVSackIterator<K, V> {
        KVSackIterator(Box::new(self.0.children.into_iter()))
    }
}

impl<K: Copy + Ord + Debug, V: Clone + Ord + Copy + Debug> Debug for KVSack<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?})", self.0)
    }
}

impl<'a, K: 'a + Ord + Copy, V: 'a + Copy + Ord> BoundedIterable<K, V> for KVSack<K, V> {
    fn into_range_iter<F>(&self, min: Bound<&K>, max: Bound<&K>) -> Range<K, V> {
        self.0.children.range(min, max).into_iter()
    }
}

pub struct KVSackIterator<K, V>(pub Box<Iterator<Item = (K, V)>>);

impl<K: Clone + Ord + Copy, V: Copy + Ord> SackLike<K, V> for KVSack<K, V>
    where KVSack<K, V>: IntoIterator
{
}

impl<K: Hash + Ord + Copy, V: Copy + Ord> WritableSack<K, V> for KVSack<K, V>
    where KVSack<K, V>: IntoIterator
{
    fn put(mut self, k: K, v: V) -> Result<Self, SackError> {
        match self.0.children.insert(k, v) {
            None => Ok(self),
            Some(_) => return Err(SackError::SackNotFound),
        }

    }
}

impl<K: 'static + Hash + Ord + Copy, V: 'static + Copy + IntoIterator + Ord> StreamWritableSack<K, V> for KVSack<K, V> {
    fn put(mut self, contents: &mut Iterator<Item=(K,V)>) -> Result<Self, SackError> {
        for item in contents.into_iter() {
            match self.0.children.insert(item.0, item.1) {
                None => (),
                Some(_) => return Err(SackError::SackNotFound),
            }
        }
        Ok(self)
    }
}
