//https://www.hackerrank.com/challenges/time-conversion

use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();

    let mut time_str = stdin.lock().lines() //iterator over lines in stdin
        .next().unwrap().unwrap().trim().to_string(); //finally it's a string;
    time_str.pop();
    let is_pm = if time_str.pop().unwrap() == 'P' {
        true
    } else {
        false
    };
    let mut time: Vec<_> = time_str.split(':').map(|x| x.parse::<i32>().unwrap()).collect();
    if is_pm {
        if time[0] != 12 {
            time[0] += 12;
        }
    } else if time[0] == 12 {
        time[0] = 0;
    }

    println!("{0:02}:{1:02}:{2:02}", time[0], time[1], time[2]);
}