// https://www.hackerrank.com/challenges/solve-me-first

use std::io;
use std::io::prelude::*;

fn main() {

    let stdin = io::stdin(); // representation of stdin

    let a: i32 = stdin.lock()                                    // locked version of stdin
                      .lines()                                   // iterator over lines in stdin
                      .next()                                    // Next line. Since line is Option<Result<String>> we need to check:
                      .expect("stdin is empty")                  // existance 
                      .expect("error reading line from stdin")   // and errorless
                      .trim()                                    // remove leading and trailing whitespace
                      .parse()                                   // try to parse value to i32
                      .expect("can't parse i32");                // and check if its ok

    let b: i32 = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap(); //same as a, but instead of expect we use unwrap for shortness

    println!("{}", a + b);

}