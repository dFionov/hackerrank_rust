use std::io;
use std::io::prelude::*;

fn main() {
    let mut stdin = io::stdin();

    //don't need it
    let count = stdin.lock().lines().next().unwrap().unwrap();
    
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut result = 0;
        let mut iter = line.bytes().map(|x| x as i8);
        while let(Some(a), Some(b)) = (iter.next(), iter.next_back()) {
            result += (b - a).abs();
        }
        println!("{}", result);
    }
}