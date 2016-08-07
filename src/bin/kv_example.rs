#![allow(unused)]
#![feature(question_mark)]
extern crate sack;
extern crate stable_bst;

use stable_bst::TreeMap;
use stable_bst::Bound;
use sack::sack::error::SackError;
use std::ops::Range;
use sack::examples::kvsack::KVSack;
use sack::sack::io::StreamWritableSack;
use sack::sack::iterator::IntoSackIterator;
use std::marker::PhantomData;
fn main() {
    match other() {
        Ok(()) => println!("No whammies!"),
        Err(err) => panic!("err {}", err),
    }
}

fn other() -> Result<(), SackError<i32, i32, i32, TreeMap<i32, i32>>> {
    let kvsack: KVSack<(), i32, i32> = KVSack {
        t: (),
        c: PhantomData,
        d: PhantomData,
        i: TreeMap::new(),
    };
    //    let kvsack = kvsack.put((3, 1))?
    //        .put(5, 2)?
    //        .put(4, 8)?
    //        .put(13, 1)?;

    // We clone because iterating over a sack consumes it, and we don't implicitly copy Sacks
    println!("iterating over all K/Vs: ");
    for item in kvsack.clone().into_sack_iter() {
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
    println!("ranged iteration - items with keys 4 and above, but less than 13: ");
    //    for (k, v) in kvsack.into_range_iter::<(i32, i32)>(Bound::Included(&4), Bound::Excluded(&13)) {
    //        println!("{:?}, {:?}", k, v);
    //    }
    println!("");
    Ok(())
}
