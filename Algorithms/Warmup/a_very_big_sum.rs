//https://www.hackerrank.com/challenges/a-very-big-sum

use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let count: usize = stdin.lock().lines() //iterator over lines in stdin
        .next().unwrap().unwrap() //finally it's a string
        .trim().parse().unwrap(); //and then parsing count value...But we don't need it :) lol

    let line = stdin.lock().lines().next().unwrap().unwrap();
    println!("{}", 
             line.trim().split(' ')
             //https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
             .fold(0, |acc, item| acc + item.parse::<i64>().unwrap() ));
}