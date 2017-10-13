//https://www.hackerrank.com/challenges/game-of-stones-1

use std::io;
use std::io::prelude::*;


fn main() {
    let stdin = io::stdin();
    
    for line in stdin.lock().lines().skip(1) {
        let n = line.unwrap().parse::<i32>().unwrap();
        if n % 7 == 0 || n % 7 == 1 {
            println!("Second");
        } else {
            println!("First");
        }
    }
    
}