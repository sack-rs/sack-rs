use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use sack::core::{Sack, SackLike};
use std::marker::Reflect;
use sack::token::Token;

#[derive(Debug)]
pub struct SackError<T: Token, C: Token, I:Into<Sack<T,C,I>>+SackLike<T, C>> {
    sack: Sack<T, C, I>,
    pub error: ErrorType,
}

#[derive(Debug)]
pub enum ErrorType {
    SackNotFound,
}

impl<T: Token + Reflect + Debug, C: Token + Reflect + Debug, I: SackLike<C, ()>+SackLike<T,C>+Reflect + Debug> fmt::Display for SackError<T,
                                                                                                                                        C,
                                                                                                                                        
                                                                                                                                        I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.description())
    }
}

impl<T: Token + Debug + Reflect, C: Token + Debug + Reflect, I: Debug + Reflect+SackLike<T,C>+SackLike<C,()>> Error for SackError<T, C,I> {
    fn description(&self) -> &str {
        "generic error"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
