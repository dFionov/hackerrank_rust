use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    //don't need it
    let count = stdin.lock().lines().next().unwrap().unwrap();

    let line = stdin.lock().lines().next().unwrap().unwrap();

    println!("{}", line.trim().split(' ').fold(0, |acc, item| acc ^ item.parse::<i32>().unwrap()));
}