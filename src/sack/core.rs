#[derive(Debug,Clone,PartialOrd,Ord,PartialEq,Eq)]
pub struct Sack<T: Ord + Clone, C: Clone + Ord, I: SackLike<T, C>> {
	pub t:T,
	pub c:C,
	pub i:I,
}

impl<K: Clone + Ord, V: Clone + Ord,I:SackLike<K,V>> SackLike<K, V> for Sack<K, V, I> {
	fn len(&self) -> isize {
		self.i.len()
	}
}

pub trait SackLike<T, C> {
    fn len(&self) -> isize;
}

//impl<T, C> SackLike<T, C> for (T, C) {
//    fn len(&self) -> isize {
//        1
//    }
//}

//impl SackLike<i32, i32> for (i32, i32) {}
//impl<T: Ord + Clone, C: Ord + Clone, I: SackLike<T, C>> SackLike<T, C> for Sack<T, C, I> {
//    fn len(&self) -> isize {
//        0
//    }
//}

// impl<T:Ord+Clone> From<T> for Sack<T, (),()> {
//    type from = T;
// }

impl<T: Sized + Clone + Ord> From<T> for Sack<T, (), ()>
    where (): SackLike<T, ()>
{
    fn from(val: T) -> Sack<T,(), ()> {
        Sack {
			t:val,c:(),i:()}
        
    }
}



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

impl<K: Clone + Ord, V: Clone + Ord> SackLike<K, V> for Sack<(K, V), (), (K, V)> {
    fn len(&self) -> isize {
        1
    }
}

impl<T, C> SackLike<T, C> for () {
    default fn len(&self) -> isize {
        unimplemented!()
    }
}

impl<T> SackLike<T, ()> for () {
    fn len(&self) -> isize {
        0
    }
}
impl SackLike<i32, ()> for i32 {
    fn len(&self) -> isize {
        1
    }
}
