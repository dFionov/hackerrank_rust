//https://www.hackerrank.com/challenges/mini-max-sum

use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let values: Vec<i64> = stdin.lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(' ')
        //https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let sum: i64 = values.iter().sum();
    let min = values.iter().min().unwrap();
    let max = values.iter().max().unwrap();

    println!("{} {}", sum - max, sum - min);
}