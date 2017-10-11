use std::io;
use std::io::prelude::*;

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn main() {
    let stdin = io::stdin();

    let count: usize = stdin.lock().lines() //iterator over lines in stdin
        .next().unwrap().unwrap() //finally it's a string
        .trim().parse().unwrap(); //and then parsing count value...But we don't need it :) lol

    for line in stdin.lock().lines() {
        let line: String = line.unwrap();
        let v: Vec<&str> = line.trim().split(' ').collect();
        let a: u32 = v[0].parse().unwrap();
        let b: u32 = v[1].parse().unwrap();
        println!("{}", add(a, b));
    }
}