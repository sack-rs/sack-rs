use sack::core::{Sack, SackLike};
// use sack::wrapper::SackWrappable;
use sack::proto::ProtoSack;
use stable_bst::Bound;
use stable_bst::map::Range;
use stable_bst::map;
use stable_bst::TreeMap;
// use stable_bst::Direction;

pub struct SackIterator<T: Ord + Clone, C: Ord + Clone> {
    pub internal: TreeMap<T, C>,
}

pub struct WrappingSackIterator<T: Clone + Ord, C: Clone + Ord> {
    pub internal: map::IntoIter<T, C>,
}

// impl<T:Clone+Ord,C:Clone+Ord,I> SackIterator<T,C,I> for WrappingSackIterator<T,C> {
//
// }

pub trait IntoSackIterator<T: Ord + Clone, C: Ord + Clone, I: SackLike<T, C>> {
    type Item;
    type IntoSackIter;
    fn into_sack_iter(self) -> SackIterator<T, C>;
    //        where Self: Sized;
}

impl IntoSackIterator<(), (), ()> for () {
    type Item = ();
    type IntoSackIter = NullSackIterator;
    fn into_sack_iter(self) -> NullSackIterator {
        unimplemented!()
    }
}

impl IntoSackIterator<i32, (), ()> for i32 {
    type Item = ();
    type IntoSackIter = SingleSackIterator<i32>;
    fn into_sack_iter(self) -> SackIterator<i32, ()> {
        unimplemented!()
    }
}

impl<K: Clone + Ord, V: Clone + Ord, I: SackLike<K, V>> IntoSackIterator<K, V, I> for Sack<K, V, I> {
    type Item = ();
    type IntoSackIter = SackIterator<K, V>;
    fn into_sack_iter(self) -> SackIterator<K, V> {
        unimplemented!()
    }
}

// impl<K:Clone+Ord,V:Clone+Ord> IntoSackIterator<K, V, TreeMap<K,V>> for Sack<K,V,TreeMap<K,V>> {
//    default type Item = ();
//    default type IntoSackIter = SingleSackIterator<(K,V)>;
//    default fn into_sack_iter(self) -> SackIterator<K,V> {
//        unimplemented!()
//    }
// }

pub type NullSackIterator = SackIterator<(), ()>;

pub type SingleSackIterator<T> = SackIterator<T, ()>;
// impl SackIterator<i32,(),()> for i32 {
// /    type Item = i32;
// }

// impl SackIterator<(),(),()> for NullSackIterator {
// }
//
// impl SackIterator<i32,(),()> for SingleSackIterator<i32> {
//
// }

// impl SackIterator<i32,(),i32> for SingleSackIterator<i32> {
// }

impl Iterator for NullSackIterator {
    type Item = ProtoSack;
    fn next(&mut self) -> Option<ProtoSack> {
        None
    }
}

impl<T: Clone + Ord, C: Clone + Ord> Iterator for SackIterator<T, C> {
    type Item = Sack<(T, C), (), ()>;
    fn next(&mut self) -> Option<<SackIterator<T, C> as Iterator>::Item> {
        unimplemented!()
    }
}
// impl<T:Clone+Ord,C:Clone+Ord> IntoIterator for Sack<T,C,TreeMap<T,C>>{
// 	type Item = Sack<(T,C),(),()>;
// 	type IntoIter=SackIterator<T,C>;
// 	fn into_iter(self) -> i32 {
//
// 	}
// }

// impl<T:Clone+Ord, C:Clone+Ord> Iterator for SackIterator<T, C> {
//    default type Item = Sack<T,C,TreeMap<T,C>>;
//    default fn next(&mut self) -> Option<SackIterator<T,C>> {
//        unimplemented!()
//        //        self.0.next()
//    }
// }

pub trait IntoNullSackIterator: IntoIterator {
    fn into_iter() {}
}

pub trait BoundedIterable<T, C> {
    fn into_range_iter<F>(&self, rmin: Bound<&T>, max: Bound<&T>) -> Range<T, C>;
}