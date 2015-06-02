use std::io;
use std::io::prelude::*;

fn main() {
    let mut stdin = io::stdin();

    //don't need it
    let n_t_line = stdin.lock().lines().next().unwrap().unwrap();
    
    let segment_widths: Vec<u32> = 
        stdin.lock().lines()    //Iterator over lines in stdin()
        .next().unwrap().unwrap()   //String represents next line
        .trim().split(' ')  //Splitted string(Vec<&str>)
        .map(|x| x.parse::<u32>().unwrap()).collect();  //Parse each value as u32 and collect in Vec
    
    for line in stdin.lock().lines() {
        let start_stop: Vec<usize> = line.unwrap().trim().split(' ').map(|x| x.parse::<usize>().unwrap()).collect();
        let mut min: u32 = *((segment_widths[start_stop[0]..start_stop[1]].iter()).min().unwrap());
        min = if min > 3 {3} else {min};
        println!("{:?}", min);
    }
}