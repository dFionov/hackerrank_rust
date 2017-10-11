// https://www.hackerrank.com/challenges/compare-the-triplets

use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    // don't need it
    let count = stdin.lock().lines().next().unwrap().unwrap();

    let line = stdin.lock().lines().next().unwrap().unwrap();

    println!("{}",
             line.trim().split(' ')
             //https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
             .fold(0, |acc, item| acc + item.parse::<i32>().unwrap()));
}