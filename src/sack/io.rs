use sack::core::{Sack, SackLike};
use sack::iterator::SackIterator;
use sack::error::SackError;
use sack::token::Token;

/// // Sacks without the mutable trait won't have a set operation, and therefore won't
/// // be able to be written to
/// // Unless they have contents with inner-mutability (inspired by rust structs)
pub trait StreamWritableSack<T: Token, C: Token, D: Token, I: SackLike<C, D, (), ()>>
    where Self: Sized
{
    fn put(self, contents: &SackIterator<T, C, D, I>) -> Result<Self, SackError<T, C, D, I>>;
}
