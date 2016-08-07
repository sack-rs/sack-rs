use std::marker::PhantomData;
use sack::token::Token;
use stable_bst::TreeMap;
use sack::iterator::SackIterator;

// type Token = Clone + Ord;

#[derive(Debug,Clone,PartialOrd,Ord,PartialEq,Eq)]
pub struct Sack<T: Token, C: Token, D: Token, I> {
    pub t: T, // The token by which I know myself, and by which my parents know me
    pub c: PhantomData<C>, // The top level type that this sack contains (everything in my universe that I can see
    pub d: PhantomData<D>, // The fully qualified typesystem of all possible nestings within the child's sacks. the descendants of the children
    pub i: I, // The storage implementation of this (non-nested) sack, which must implement sacklike functionality over C,D
}

// impl Into<Sack<(),(), (), ()>> for () {}

pub trait SackLike<T: Token, C: Token, D: Token, I>: Token + Into<Sack<T, C, D, I>> {
    fn len(&self) -> isize;
    fn into_iter(&self) -> &Iterator<Item = TokenSack<T>>;
}

pub type TokenSack<T> = Sack<T, (), (), ()>;

impl<T: Token, C: Token, D: Token, I: SackLike<T, C, D, I>> Token for Sack<T, C, D, I> {}

impl<'a, T: Token, C: Token> SackLike<T, C, (), ()> for Sack<T, C, (), ()> {
    fn len(&self) -> isize {
        1
    }
    fn into_iter(&self) -> &Iterator<Item = Sack<T, (), (), ()>> {}
}

// impl SackLike<i32,i32,(),()> for TreeMap<i32, i32>{
// fn len(&self) -> isize{1}
// }

impl<'a, T: Token, C: Token> From<&'a TreeMap<T, C>> for Sack<T, C, (), ()> {
    fn from(t: &TreeMap<T, C>) -> Self {}
}
impl<'a, T: Token, C: Token> From<()> for Sack<T, C, (), ()> {
    fn from(t: ()) -> Self {}
}


// impl<T, C> SackLike<T, C> for (T, C) {
//    fn len(&self) -> isize {
//        1
//    }
// }

// impl SackLike<i32, i32> for (i32, i32) {}
// impl<T: Ord + Clone, C: Ord + Clone, I: SackLike<T, C>> SackLike<T, C> for Sack<T, C, I> {
//    fn len(&self) -> isize {
//        0
//    }
// }

// impl<T:Ord+Clone> From<T> for Sack<T, (),()> {
//    type from = T;
// }

// impl<T: Sized + Token> From<T> for Sack<T, T, (), ()>
//    where (): SackLike<T, (),(),()>
// {
//    fn from(val: T) -> Sack<T,T, (), ()> {
//        Sack {
// 			t:val,
// 			c:PhantomData,
// 			d:PhantomData,
// 			i:()
//        }
//
//    }
// }



// impl<T> SackLike<T, ()> for T {
//    fn len(&self) -> isize {
//    	match self {
//    		() => 0,
//    		//String(it) => it.len(),
//
//    	}
//        1
//    }
// }

pub struct WrapperSack<T: Token + SackLike<T, (), (), ()>, C: Token>(// ), (), ()>, C: Token>(// ),(),()>, C: Token>(
                                                                     pub Sack<T, C, (), ()>);





impl<T: Token, C: Token> SackLike<T, C, (), ()> for () {
    fn len(&self) -> isize {
        0
    }

    fn into_iter(&self) -> &Iterator<Item = Sack<T, (), (), ()>> {}
}

// impl SackLike<(), (),(),()> for () {
//    fn len(&self) -> isize {
//        0
//    }
// }
// impl<T:Token> SackLike<T, (),(),()> for T {
//    fn len(&self) -> isize {
//        1
//    }
// }
