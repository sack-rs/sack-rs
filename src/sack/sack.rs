use stable_bst::Bound;
use stable_bst::map::Range;
use std::ops::Fn;
use sack::sackerror::SackError;
use sack::core::{Sack};
use sack::iterator::{SackIterator,IntoSackIterator};
use sack::proto::ProtoSack;

impl<T: Clone + Ord, I: Clone + Ord + SackBack<T, (),>> SackBack<Sack<T, (), I>, ()> for Sack<T, (), I> {}

impl<T: Clone + Ord, I: Clone + Ord + SackBack<T, ()>> IntoSackIterator<Sack<T, (), I>, (), ()> for Sack<T, (), I> {}


impl<T: Ord + Clone + SackBack<T, ()>> SackIterator for SingleSackIterator<T> {
    type Item = T;
}

 impl<T: Clone + Ord, I: SackBack<T, ()>> SackBack<T, ()> for WrapperSack<T, I>
    where WrapperSack<T, I>: IntoSackIterator<T, (), ()>
 {
    fn len(&self) -> isize {
        1
    }
 }



/// Sacks without the mutable trait won't have a set operation, and therefore won't
/// be able to be written to
/// Unless they have contents with inner-mutability (inspired by rust structs)
// pub trait WritableSack<T: Clone + Ord, C: Clone + Ord>: SackLike<T, C> {
//    fn put(self, k: T, v: C) -> Result<Self, SackError> where Self: Sized;
// }
/// Not all sacks support all desirable operations, so they are split out into various
/// capabilities packages
/// Splittable sacks are those who's contents are able to be copied and split,
/// along token boundaries, into two new
/// containers that are then able to be managed separately
pub trait SplittableSack<T: Clone + Copy + Ord, C: Clone + Copy + Ord, I: SackBack<T, C>> {
    /// any sack that wants to support the most important mgmt/orchestration needs
    /// to be able to split itself at an arbitrary token that it contains
    fn split<F>(self, split_fn: F) -> Result<ProxySack<T, C, I>, SackError<T, C, I>>
        where F: for<'a> Fn<(&'a SackBack<T, C, IntoSackIter = Self, Item = (T, C)>)>;
}


// impl<T: Ord + Clone, C: Ord + Clone> SackLike<T, C> for ProxySack<T, C> {}

// impl<T: Ord + Clone, C: Ord + Clone> IntoSackIterator<T, C> for ProxySack<T, C> {
//    type Item=Sack<T,C>;
//    type IntoSackIter=SackIterator<Item=Sack<T,C>>;
// }






//
impl SackBack<(), ()> for ProtoSack {
    fn len(&self) -> isize {
        0
    }
}





