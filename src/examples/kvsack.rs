// use sack::io::StreamWritableSack;
// use sack::error::SackError;
// use std::fmt;
// use sack::core::{Sack, SackLike};
// use sack::core::WrapperSack;
// / StreamWritableSack, BoundedIterable, ProxySack, WrapperSack};
// use sack::iterator::*;
// use stable_bst::TreeMap;
// use stable_bst::map;
// use std::hash::Hash;
// use std::iter::Iterator;
// use std::u64;
// use std::default::Default;
// use std::marker::PhantomData;
// use sack::token::Token;
// use stable_bst::map::IntoIter;
//
// // A KVSack is a concrete implementation of a Sack api that
// // just supports standard K/V operations (get and set for now)
// // While its data persistence and querability is not interesting,
// // it will be an example for all new sack-orchestration capabilities.
// // see kv_example.rs for a runnable example
// / #[derive(Clone,PartialOrd,Ord,PartialEq,Eq)]
// / pub struct KVSack<K: Ord + Clone, V: Ord + Clone>(Sack<K, V>) ;
//
// pub type KVSack<T, K,V> = Sack<T, K, V, TreeMap<K, V>>;
//
// pub type KVPair<K, V> = Sack<K, V, (), ()>;
//
// /FIXME, values shouldn't have to be tokens
// pub struct KVSackIterator<K:Token, V:Token>{iter:Box<Iterator<Item=Sack<K,V,(),()>>>}
//
// / impl<K:Clone+Ord + SackLike<K,V>,V:Clone+Ord> IntoSackIterator<K,V,TreeMap<K,V>> for KVSack<K,V> {
// /
// / }
//
// impl<K: Token, V: Token> SackLike<K, V,(),()> for TreeMap<K, V> {
//    fn len(&self) -> isize {
//        self.len() as isize /*FIXME*/
//    }
// }
//
// const BIG: u64 = 3;
//
// impl<K: Token + SackLike<K, (),(),()>, V: Token + SackLike<K, V,(),()>> KVSack<(),K, V>where TreeMap<K, V>: SackLike<V, (),(),()> {
//    pub fn bisect_if_big(self) -> Result<Self, (Self, SackError<K, V, (),TreeMap<K, V>>)> {
//        match self.i.len() as u64 {
//            u64::MIN...BIG => {
//                unimplemented!()
//                //                Err((self,
//                //                     SackError {
//                //                    sack: self.0,
//                //                    error: ErrorType::SackNotFound,
//                //                }))
//            }
//            BIG...u64::MAX => {
//                //  self.bisect()
//                unimplemented!()
//            }
//            _ => unimplemented!(),
//
//        }
//    }
//
//    fn bisect(self) -> Result<Sack<K, V, (),TreeMap<K, V>>, SackError<K, V, (),TreeMap<K, V>>> {
//        // self.0.children.range_iter
//        //        let mut sack1 = KVSack::<K, V>::default();
//        //        let sack2: KVSack<K, V> = KVSack::default();
//        //
//        //        let len1 = self.children.len() / 2;
//        //        let mut iter: KVSackIterator<K, V> = KVSackIterator { sack_children: Box::new(self.children.into_iter()) };
//        //         for i in 0..len1 {
//        //         	 match iter.next() {
//        //         		Some(next) => {sack1=sack1.put(next.0,next.1).unwrap()}
//        //         		None => unimplemented!()
//        //
//        //         	};
//        // 	sack1.put(item.0,item.1);
//        unimplemented!()
//        //  }
//    }
// }
//
// /impl<K:Token,V:Token> Default for KVSack<(),K,V> where TreeMap<K, V>: SackLike<V, ()>{
// /	    fn default() -> Self {
// /    	Sack {
// /    		t:(),
// /    		c:PhantomData,
// /    		d:PhantomData,
// /    		i:TreeMap::new(),
// /    	}
// /    }
// /
// /}
//
// / type foo<K,V>= KVSackIterator<K,V,Item=KVSack<K,V>>;
//
// /impl<'a, K: 'a + Ord + Clone + SackLike<K, ()>, V: 'a + SackLike<K, ()> + SackLike<V, ()>> IntoSackIterator<'a, K, V, (),()> for KVPair<K, V> {
// /    type IntoSackIter = SingleSackIterator<'a, (K, V)>;
// /    type Item = Sack<(K, V), (), (),()>;
// /    fn into_sack_iter(self) -> SackIterator<K, V,(),()> {
// /        SackIterator { internal: unimplemented!() }
// /    }
// /}
//
// impl<'a, K: Token, V: Token> IntoSackIterator<'a, K, V, (),Sack<K, V, (),()>> for map::Iter<'a, K, V, map::Forward> {
//    type Item = KVPair<K, V>;
//    type IntoSackIter = KVSackIterator<K, V>;
//    fn into_sack_iter(self) -> SackIterator<K, V,(),Sack<K,V,(),()>> {
//        unimplemented!()
//    }
// }
//
// impl<K:Token, V:Token> SackLike<(K, V), (),(),()> for (K, V) {
//    fn len(&self) -> isize {
//        1
//    }
// }
//
// /impl<'a, K: SackLike<K, ()>, V: SackLike<K, ()>> IntoSackIterator<'a, K, V, (),TreeMap<K, V>> for KVSack<K, V> {
// /    type Item = Sack<K, V,(),()>;
// /    type IntoSackIter = KVSackIterator<K, V>;
// /    fn into_sack_iter(self) -> SackIterator<K, V,(),()> {
// /        SackIterator{internal:self.i.into_iter()}
// /    }
// /}
//
// trait IntoKVSackIterator<'a, K: Token + SackLike<K, V,(),()>, V: Token>
//    : IntoSackIterator<'a, K, V, (),()> {
//    // fn into_sack_itter
// }
//
//
// impl<'a> IntoKVSackIterator<'a, i32, ()> for i32 {
//    //    fn into_sack_iter(&self) -> SackIterator<Item=Sack<i32,i32,()>>  {
//    //    unimplemented!()
//    //    }
// }
//
//
//
//
//
// / impl<K:Ord+Clone,V:Ord+Clone> IntoSackIterator<K,V> for KVSack<K,V> {
// / 	type Item=KVSack<K,V>;
// / 	type IntoSackIter = Iterator<Item=String>;
// / }
//
// /impl<K: PartialOrd + Ord + Clone + SackLike<K, ()>, V: Ord + Clone + SackLike<K, ()>> Default for KVSack<K, V> {
// /    fn default() -> Self {
// /        Sack {
// /        	t:T,
// /        	c:(),
// /            i: TreeMap::new(),
// /        }
// /    }
// /}
//
// / impl<K: Ord, V> SackLike<K, V> for TreeMap<K, V> {}
//
// impl<K: Token + SackLike<K, V,(),()>, V: Token> Iterator for KVSackIterator<K, V> {
//    type Item = Sack<K, V, (),()>;
//    fn next(&mut self) -> Option<Sack<K,V,(),()>> {
//        match self.iter.next() {
//            None => None,
//            Some(kv) => {
//                Some(kv)
//            }
//        }
//    }
// }
//
//
// / impl<'a, K: 'a + Ord + Copy, V: 'a + Copy + Ord> BoundedIterable<K, V> for KVSack<K, V> {
// /    fn into_range_iter<F>(&self, min: Bound<&K>, max: Bound<&K>) -> Range<K, V> {
// /        self.children.range(min, max).into_iter()
// /    }
// / }
//
//
//
//
//
// / impl<K: Clone + Ord + Copy, V: Copy + Ord> WrapperSack<K, V> for KVSack<K, V>
// /    where KVSack<K, V>: IntoIterator
// / {
// /    fn len(&self) -> isize {
// /        self.children.len()
// /    }
// / }
//
// impl StreamWritableSack<i32, i32, (),TreeMap<i32, i32>> for KVSack<(),i32, i32> {
//    fn put(self, contents: &SackIterator<i32, i32,(),TreeMap<i32,i32>>) -> Result<KVSack<(),i32, i32>, SackError<i32, i32, (),TreeMap<i32, i32>>> {
//        unimplemented!()
//    }
// }
//
// / impl<K: Hash + Ord + Copy+SackLike<K,()>, V: SackWrappable<K, ()>+SackLike<K,()>> StreamWritableSack<K, V, TreeMap<K, V>> for KVSack<K, V>
// /    where KVSack<K, V>: IntoIterator, V: IntoIterator + Clone + Ord + Copy
// / {
// /    fn put(self, contents: &SackIterator<Item = Sack<K, V, TreeMap<K, V>>>)
// /           -> Result<KVSack<K, V>, SackError<K, V, TreeMap<K, V>>> {
// / /        for item in contents.into() {
// / /            match self.c.insert(item.0, item.1) {
// / /                None => (),
// / /                Some(_) => return Err(SackError { sack:self, error: ErrorType::SackNotFound }),
// / /// FIXME error handling
// / /            }
// / /        }
// / panic!();
// /        Ok(self)
// / / unimplemented!()
// / / FIXME
// / / 		if self.0.children.len() > 2 {result.bisect_if_big()} else {Ok(self)}
// /    }
// / }
//
//
// / impl<K: Ord + Clone + Copy, V: Ord + Clone + Copy> SplittableSack<K, V> for KVSack<K, V> {
// /    fn split<F>(self, split_fn: F) -> Result<ProxySack<K,V>, SackError<K, V>>
// /        where F: for<'a> Fn<(&'a SackLike<K, V, IntoSackIter = SackIterator<Item=KVSack<K, V>>, Item = (K, V)>)>
// /    {
// /        // let K = split_fn(&self);
// /        unimplemented!()
// /    }
// / }
//
// / impl<K: Hash+Ord+Copy, V: Copy + IntoIterator + Ord>StreamWritableSack<K, V>
// / for KVSack<K, V> {
// /    fn put(mut self, contents: &mut SackIterator<K,V>) -> Result<Self, SackError> {
// /        for item in contents.into_iter() {
// /            match self.0.children.insert(item.0, item.1) {
// /                None => (),
// /                Some(_) => return Err(SackError::SackNotFound),
// /            }
// /        }
// /        Ok(self)
// /    }
// / }
//
// impl<'a, K: Token, V: SackLike<V, (),(),()>, I: SackLike<V,(),(),()>> IntoSackIterator<'a, K, V, (),I> for TreeMap<K, V> {
//    type Item = KVPair<K,V>;
//    type IntoSackIter = KVSackIterator<K, V>;
//    fn into_sack_iter(self) -> SackIterator<K, V,(),I> {
//    	let foo:IntoIter<K,V> = self.into_iter();
// /        SackIterator { internal: self.into_iter() }
// unimplemented!()
//    }
// }
//
// / impl<K: Clone + Ord + SackLike<K, ()>, V: Clone + Ord + SackLike<V, ()>> IntoSackIterator<V, (), TreeMap<V, ()>> for TreeMap<K, V>
// /    where (): SackLike<V, ()>
// / {
// /    type Item = WrapperSack<V, ()>;
// /    type IntoSackIter = KVSackIterator<V, ()>;
// / }
