#![allow(unused)]
#![feature(question_mark)]
extern crate sack;
extern crate stable_bst;

use stable_bst::Bound;
use sack::sack::sackerror::SackError;
use sack::kvsack::*;
use sack::sack::sack::*;
use std::ops::Range;

fn main() {
    match other() {
        Ok(()) => println!("No whammies!"),
        Err(err) => panic!("err {}", err),
    }
}

fn other() -> Result<(), SackError> {
    let kvsack: KVSack<i32, i32> = KVSack::default();
    let kvsack = kvsack.put(3, 1)?
        .put(5, 2)?
        .put(4, 8)?
        .put(13, 1)?;

    // We clone because iterating over a sack consumes it, and we don't implicitly copy Sacks
    println!("iterating over all K/Vs: ",);
    for item in kvsack.clone() {
        println!("{:?}", item);
    }
    println!("");

    // Testing debug fmt of kvsacks. It should contain the same info (as well as more structure)
    // as the previous iterator
    println!("debug printing the KVSack: {:?}\n", kvsack);

    // Using a std.ops.Range, but will switch to a more powerful
    // construct for bounded iteration in the future
    // Note that rust Ranges are inclusive start and exclusive end
    // So this should print 3, but not 13
    println!("ranged iteration - items with keys 3 and above, but less than 13: ");
    for (k, v) in kvsack.into_range_iter::<(i32, i32)>(Bound::Included(&3), Bound::Excluded(&13)) {
        println!("{:?}, {:?}", k, v);
    }
    println!("");
    Ok(())
}
