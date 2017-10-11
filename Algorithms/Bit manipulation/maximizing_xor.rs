use std::io;
use std::io::prelude::*;

const U32BITS: u32 = 32;

//O(1) complexity algorithm
fn max_xor(l: u32, r: u32) -> u32 {
    assert!(r >= l);
    let k: u32 = r - l;
    let mut i: u32 = 0;
    //We find's number of significant bits in r - l
    while i < U32BITS && ((k >> i) != 0) {
        i += 1;
    }
    //Now we need a number with 1 in i significant bits
    let mut d: u32 = 0;
    for j in 0..i {
        d += 1 << j;
    }
    //Finally we just xor our range values with d
    (r ^ l) | d
}

fn main() {
    let stdin = io::stdin();

    //One thing i hate in rust:
    let l: u32 = stdin.lock().lines()
        .next().unwrap().unwrap()
        .trim().parse().unwrap();
    let r: u32 = stdin.lock().lines()
        .next().unwrap().unwrap()
        .trim().parse().unwrap();
    //is it possible to do it with less unwraps? :) lol

    println!("{}", max_xor(l, r));
}