use std::io;
use std::io::prelude::*;

fn main() {
    let mut stdin = io::stdin();

    //don't need it
    let count = stdin.lock().lines().next().unwrap().unwrap();
    
    for line in stdin.lock().lines() {
        let bytes: Vec<i8> = line.unwrap().as_bytes().iter().map(|x| *x as i8).collect();
        let mut result = 0;
        let mut iter = bytes.iter();
        while let(Some(&a), Some(&b)) = (iter.next(), iter.next_back()) {
            result += (b - a).abs();
        }
        println!("{}", result);
    }
}