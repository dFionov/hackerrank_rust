//https://www.hackerrank.com/challenges/birthday-cake-candles

use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let values: Vec<i32> = stdin.lock()
        .lines()
        .skip(1).next()
        .unwrap()
        .unwrap()
        .trim()
        .split(' ')
        //https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let max = values.iter().max().unwrap();
    let count = values.iter().fold(0, |acc, x| acc + if x == max {1} else {0});

    println!("{}", count);
}