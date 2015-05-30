use std::io;
use std::io::prelude::*;

fn height(num_cycles: u32) -> u32 {
    let mut height: u32 = 1;
    for i in 0..num_cycles {
        if i % 2 == 0 {
            height *= 2;
        } else {
            height += 1;
        }
    }
    height
}

fn main() {
    let stdin = io::stdin();

    let count: usize = stdin.lock().lines() //iterator over lines in stdin
        .next().unwrap().unwrap() //finally it's a string
        .trim().parse().unwrap(); //and then parsing count value...But we don't need it :) lol

    for line in stdin.lock().lines() {
        let num_cycles: u32 = line.unwrap().trim().parse().unwrap();
        
        println!("{}", height(num_cycles));
    }

}