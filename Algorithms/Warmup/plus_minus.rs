//https://www.hackerrank.com/challenges/plus-minus

use std::io;
use std::io::prelude::*;


fn main() {
    let stdin = io::stdin();

    let count: f64 = stdin.lock().lines() //iterator over lines in stdin
        .next().unwrap().unwrap() //finally it's a string
        .trim().parse().unwrap(); //and then parsing count value...
    let mut count_positive: f64 = 0.0;
    let mut count_negative: f64 = 0.0;
    let mut count_zeros: f64 = 0.0;
    for line in stdin.lock().lines() {
        let line: String = line.unwrap();
        let v: Vec<f64> = line.trim().split(' ').map(|x| x.parse::<f64>().unwrap()).collect();
        for item in v {
            if item > 0.0  {
                count_positive += 1.0;
            } else if item < 0.0  {
                count_negative += 1.0;
            } else {
                count_zeros += 1.0;
            }
        }
        
    }
    println!("{}", count_positive / count);
    println!("{}", count_negative / count);
    println!("{}", count_zeros / count);
}