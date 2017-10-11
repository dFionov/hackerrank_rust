// https://www.hackerrank.com/challenges/compare-the-triplets

use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let alice_rating: Vec<_> = stdin.lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(' ')
        //https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let bob_rating = stdin.lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut alice_score = 0;
    let mut bob_score = 0;

    //https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip
    for (a, b) in alice_rating.iter().zip(bob_rating.iter()) {
        if a > b {
            alice_score += 1;
        } else if a < b {
            bob_score += 1;
        }
    }

    println!("{} {}", alice_score, bob_score);
}