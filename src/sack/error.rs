use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use sack::core::{Sack, SackLike};
use std::marker::Reflect;
use sack::token::Token;

#[derive(Debug)]
pub struct SackError<T: Token, C: Token, D: Token, I> {
    sack: Sack<T, C, D, I>,
    pub error: ErrorType,
}

#[derive(Debug)]
pub enum ErrorType {
    SackNotFound,
}

impl<T: Token + Reflect + Debug, C: Token + Reflect + Debug, D: Token + Reflect + Debug, I: SackLike<C, D, (), ()>+SackLike<C, D,  (), ()>+Reflect + Debug> fmt::Display for SackError<T,
                                                                                                                                        C,
                                                                                                                                        D,
                                                                                                                                        I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.description())
    }
}

impl<T: Token + Debug + Reflect, C: Token + Debug + Reflect, D: Token + Debug + Reflect, I: Debug + Reflect+SackLike<C,D,(),()>> Error for SackError<T, C, D,I> {
    fn description(&self) -> &str {
        "generic error"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
