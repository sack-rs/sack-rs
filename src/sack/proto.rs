use sack::core::Sack;
use sack::iterator::{IntoSackIterator, NullSackIterator, WrappingSackIterator};
use sack::iterator::SackIterator;
use sack::core::SackLike;
use stable_bst::TreeMap;

pub type ProtoSack = Sack<(), (), ()>;

//impl IntoSackIterator<(), (), ()> for ProtoSack {
//    type Item = Sack<(), (), ()>;
//    type IntoSackIter = NullSackIterator;
//    fn into_sack_iter(self) -> SackIterator<(), ()> {
//        unimplemented!()
//    }
//}



//impl IntoIterator for ProtoSack where SackIterator<(),()> : IntoIterator {
//default    type Item = Sack<(), (), ()> ;
//    type IntoIter = NullSackIterator;
//    fn into_iter(self) -> NullSackIterator {
//        NullSackIterator { internal: TreeMap::new() }
//    }
//}
