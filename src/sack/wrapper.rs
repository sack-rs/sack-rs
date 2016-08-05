use sack::core::{Sack, SackLike};
use sack::iterator::{IntoSackIterator, NullSackIterator, SackIterator};
use sack::proto::ProtoSack;

/// A Wrapper Sack is a sack that can only contain a single non-sack value, and therefore can't have recursive children.
/// It has all the normal sack functionality, but by guaranteeing having no children and only a single item,
/// the type system can be
/// simplified.
pub struct WrapperSack<T: Clone + Ord + SackLike<T, ()>, I: SackLike<T, ()>>(// )>, I: SackLike<T, ()>>(// )>, I: SackLike<T, ()>>(// )>, I: SackLike<T, ()>>(
                                                                             pub Sack<T, (), I>);

// // Anything that can provide enough functionality that it can be the backing
// // store for a sack
// pub trait SackWrappable<T: Clone + Ord + SackLike<T, C>, C: Clone + Ord>
//    : IntoSackIterator<T, C, T, Item = T> {
//    fn len(&self) -> isize;
// }

// impl SackWrappable<(), ()> for () {
//    fn len(&self) -> isize {
//        0
//    }
// }

// impl<V:Ord+Clone>  SackWrappable<V, ()> for () {
//
// }

// impl SackWrappable<(), ()> for Sack<(),(),()> {
//    fn len(&self) -> isize {}
// }
//
// impl<T: Clone + Ord, I: SackWrappable<T, ()> + Clone + Ord> IntoSackIterator<T, (), I> for WrapperSack<T, I> {
//    type Item = Sack<T, (), I>;
//    type IntoSackIter = SingleSackIterator<Sack<T, (), I>>;
// }


// impl IntoSackIterator<(),(), ()> for () {
// 	type IntoSackIter=NullSackIterator;
// }

// impl SackIterator<(),(),()> for ProtoSack {
// }

// impl SackBack<(), ()> for ProtoSack {
//    fn len(&self) -> isize {
//        0
//    }
// }


// impl SackWrappable<i32, ()> for i32{
// fn len(&self)->isize{1}
// }
