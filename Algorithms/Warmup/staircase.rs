//https://www.hackerrank.com/challenges/staircase

use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let count: i32 = stdin.lock().lines() //iterator over lines in stdin
        .next().unwrap().unwrap() //finally it's a string
        .trim().parse().unwrap(); //and then parsing count value...But we don't need it :) lol
    for i in 0..count {
        for _ in 0..count - i - 1 {
            print!(" ");
        }
        for _ in 0..i+1 {
            print!("#");
        }
        println!("");
    }
}