use sack::core::{Sack, SackLike};
// use sack::wrapper::SackWrappable;
use sack::proto::ProtoSack;
use stable_bst::Bound;
use stable_bst::map::Range;
use stable_bst::map;
use stable_bst::TreeMap;
use stable_bst::map::Iter as TreeMapIter;
use stable_bst::map::Forward;
use stable_bst::map::IntoIter as IntoTreeMapiter;
// use stable_bst::Direction;

pub struct SackIterator<T: Ord + Clone, C: Ord + Clone> {
    pub internal: IntoTreeMapiter<T, C>
}

pub struct WrappingSackIterator<T: Clone + Ord, C: Clone + Ord> {
    pub internal: map::IntoIter<T, C>,
}

// impl<T:Clone+Ord,C:Clone+Ord,I> SackIterator<T,C,I> for WrappingSackIterator<T,C> {
//
// }

pub trait IntoSackIterator<'a, T: Ord + Clone, C: Ord + Clone, I: SackLike<T, C>> {
    type Item;
    type IntoSackIter;
    fn into_sack_iter(self) -> SackIterator<T, C>;
    //        where Self: Sized;
}

impl<'a> IntoSackIterator<'a, (), (), ()> for () {
    type Item = ();
    type IntoSackIter = NullSackIterator<'a>;
    fn into_sack_iter(self) -> NullSackIterator<'a> {
        unimplemented!()
    }
}

impl<'a> IntoSackIterator<'a, i32, (), ()> for i32 {
    type Item = ();
    type IntoSackIter = SingleSackIterator<'a,i32>;
    fn into_sack_iter(self) -> SackIterator<i32, ()> {
        unimplemented!()
    }
}

impl<'a, K: 'a + Clone + Ord, V: 'a + Clone + Ord, I: SackLike<K, V>> IntoSackIterator<'a, K, V, I> for Sack<K, V, I> {
    default type Item = ();
    default type IntoSackIter = SackIterator<K, V>;
    default fn into_sack_iter(self) -> SackIterator<K, V> {
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

pub type NullSackIterator<'a> = SackIterator<(), ()>;

pub type SingleSackIterator<'a, T> = SackIterator<T, ()>;
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

//impl Iterator for NullSackIterator {
//    default type Item = ProtoSack;
//    default fn next(&mut self) -> Option<ProtoSack> {
//        None
//    }
//}

impl<'a, T: Clone + Ord, C: Clone + Ord> Iterator for SackIterator<T, C> {
    type Item = Sack<T, C, ()>;
    fn next(&mut self) -> Option<<SackIterator<T, C> as Iterator>::Item> {
    	self.internal.next().and_then(|(t,c): (T,C)| Some(Sack{t:t,c:c,i:()}))
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
