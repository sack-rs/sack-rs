use stable_bst::TreeMap;


pub trait Token: Clone + Ord {}

impl Token for () {}

impl Token for i32 {}

impl<T: Token, C: Token> Token for TreeMap<T, C> {}

impl<T: Token, C: Token> Token for (T, C) {}
