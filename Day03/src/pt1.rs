use std::{cmp::max, time::Instant, vec};

pub fn run(puzzle_input: &str)-> u64{
    let mut inputs = puzzle_input.lines().map(|line|{
        let mut max_first=0;
        let mut count = 0;
        let mut digit_of_first = 0;
        for b in &line.as_bytes()[..line.len() - 1] {
            let num = b - b'0';
            if num > max_first {
                max_first = num;
                digit_of_first = count;
            }
            count+=1;
            
        }

        let mut max_second=0;
        for b in &line.as_bytes()[digit_of_first+1..line.len()] {
            let num = b - b'0';
            if num > max_second {
                max_second = num;
            }
            
        }

        let res = max_first*10+max_second;
        // println!("Line: {} resulted in {}, with first: {} and second: {}",line,res, max_first,max_second);

        res as u32
    });

    let mut sum = 0;
    for i in inputs{
        sum+=i;
    }
    sum as u64
}