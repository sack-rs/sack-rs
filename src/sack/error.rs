use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use sack::core::{Sack, SackLike};
use std::marker::Reflect;

#[derive(Debug)]
pub struct SackError<T: Ord + Clone + Copy, C: Ord + Clone + Copy, I: SackLike<T, C>> {
    sack: Sack<T, C, I>,
    pub error: ErrorType,
}

#[derive(Debug)]
pub enum ErrorType {
    SackNotFound,
}

impl<T: Clone + Ord + Copy + Reflect + Debug, C: Clone + Ord + Copy + Reflect + Debug, I:Reflect+Debug+SackLike<T,C>> fmt::Display for SackError<T, C, I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.description())
    }
}

impl<T: Clone + Ord + Copy + Debug + Reflect, C: Clone + Ord + Copy + Debug + Reflect, I: Debug + Reflect+SackLike<T,C>> Error for SackError<T, C, I> {
    fn description(&self) -> &str {
        "generic error"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
