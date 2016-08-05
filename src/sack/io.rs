use sack::core::{Sack, SackLike};
use sack::iterator::SackIterator;
use sack::error::SackError;

/// // Sacks without the mutable trait won't have a set operation, and therefore won't
/// // be able to be written to
/// // Unless they have contents with inner-mutability (inspired by rust structs)
pub trait StreamWritableSack<T: Clone + Copy + Ord, C: Clone + Copy + Ord, I: SackLike<T, C>>
    where Self: Sized
{
    fn put(self, contents: &SackIterator<T, C>) -> Result<Self, SackError<T, C, I>>;
}
