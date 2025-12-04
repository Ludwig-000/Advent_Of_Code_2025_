use std::{cmp::max, time::Instant, vec};

mod pt1;
mod pt2;

const INPUT: &'static str = include_str!("input.txt");

fn main() {

    let start = Instant::now();
    
    let mut collecting = [0;1000];

    for i in 0..1000{
        let res = pt2::run(INPUT);
        collecting[i] = res;
    }
    

    let end = Instant::now();

    println!("Time Passed: {}micro seconds", end.duration_since(start).as_micros() / 1000);
    println!("result: {:?}", collecting.first())
}