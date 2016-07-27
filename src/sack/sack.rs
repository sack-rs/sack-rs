use sack::sackerror::SackError;
use stable_bst::Bound;
use stable_bst::map::Range;

/// The fundamental nature of a sack is such that you can get the contents of
/// anything by its fully qualified name
pub trait SackLike<T, C>: IntoIterator {}

#[derive(Debug,Clone,PartialOrd,Ord,PartialEq,Eq)]
pub struct Sack<T: Ord + Clone, V: Clone + Ord, C: SackLike<T, V>> {
    pub state: Option<T>,
    pub payload: Option<V>,
    pub children: C,
}

/// // Sacks without the mutable trait won't have a set operation, and therefore won't
/// // be able to be written to
/// // Unless they have contents with inner-mutability (inspired by rust structs)
pub trait StreamWritableSack<T, C: IntoIterator>: SackLike<T, C> {
    fn put(self, contents: &mut Iterator<Item = (T, C)>) -> Result<Self, SackError>
        where Self: Sized;
}

/// Sacks without the mutable trait won't have a set operation, and therefore won't
/// be able to be written to
/// Unless they have contents with inner-mutability (inspired by rust structs)
pub trait WritableSack<T, C>: SackLike<T, C> {
    fn put(self, k: T, v: C) -> Result<Self, SackError> where Self: Sized;
}

/// Not all sacks support all desirable operations, so they are split out into various
/// capabilities packages
/// Splittable sacks are those who's contents are able to be copied and split,
/// along token boundaries, into two new
/// containers that are then able to be managed separately
pub trait SplittableSack<T, C> {
    /// any sack that wants to support the most important mgmt/orchestration needs
    /// to be able to split itself at an arbitrary token that it contains
    fn split<F>(self, split_fn: F) -> (Self, Self)
        where Self: Sized,
              F: for<'a> Fn(&'a T) -> T;
}


pub trait BoundedIterable<T, C> {
    fn into_range_iter<F>(&self, rmin: Bound<&T>, max: Bound<&T>) -> Range<T, C>;
}

pub trait IntoNullIterator: IntoIterator {
    fn into_iter() {}
}

pub struct NullIterator;

impl Iterator for NullIterator {
    type Item = ProtoSack;
    fn next(&mut self) -> Option<ProtoSack> {
        None
    }
}

pub struct ProtoSack {}

impl IntoIterator for ProtoSack {
    type Item = ProtoSack;
    type IntoIter = NullIterator;
    fn into_iter(self) -> NullIterator {
        NullIterator//
    }
}
//
impl SackLike<(), ()> for ProtoSack {}

pub struct SackIterator<'a, T: 'a, C: 'a>(&'a mut Iterator<Item = (T, C)>);

impl<'a, T, C> Iterator for SackIterator<'a, T, C> {
    type Item = (T, C);
    fn next(&mut self) -> Option<(T, C)> {
        self.0.next()
    }
}
