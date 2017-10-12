//https://www.hackerrank.com/challenges/diagonal-difference

use std::io;
use std::io::prelude::*;


fn main() {
    let stdin = io::stdin();

    let count: usize = stdin.lock().lines()
        .next().unwrap().unwrap() 
        .trim().parse().unwrap();
    
    let mut step = 0;
    let mut sum1 = 0;
    let mut sum2 = 0;
    
    for line in stdin.lock().lines() {
        let line: String = line.unwrap();
        let v: Vec<&str> = line.trim().split(' ').collect();
        sum1 += v[0 + step].parse::<i64>().unwrap();
        sum2 += v[count - 1 - step].parse::<i64>().unwrap();
        step +=1;
    }
    
    println!("{}", (sum1 - sum2).abs());
}