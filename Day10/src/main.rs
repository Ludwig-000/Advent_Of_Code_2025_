use std::{cmp::max, time::Instant, vec};

mod pt1;
mod pt2;

const INPUT: &'static str = include_str!("input.txt");

fn main() {

    let start = Instant::now();
    
    let mut collecting = [0;1];

    for i in 0..=0{
        let res = pt1::run(INPUT);
        collecting[i] = res;
    }

    let end = Instant::now();

    println!("Time Passed: {} seconds", end.duration_since(start).as_secs()/ 1);
    println!("result: {:?}", collecting.first().unwrap())
}


