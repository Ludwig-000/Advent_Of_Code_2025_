use std::{cmp::max, vec};


const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let mut inputs: Vec<Vec<u8>> = INPUT.lines().map(|line|{
        line.as_bytes().iter().map(|&byte| byte - b'0').collect()
    }).collect();

    let mut all_values=  Vec::new();
    for value in inputs{

        let mut resulting_num = Vec::new();

        let mut test: &[u8] = &value;
        println!("Our test data: {:?}",test);
        for i in 0..=11{


            let res: (u8, &[u8]) = optimal_digit(test, test.len()-(12-i));
            test = res.1;

            println!("round: {}. our number: {}, new slice: {:?}", i+1,res.0, res.1);
            test = res.1;
            resulting_num.push(res.0);

        }

        all_values.push(resulting_num);
    }

    let mut r: u64 = 0;

    for vector in all_values {
        println!("working on {:?}",vector);
        let mut acc = 0u64;
        let mut exp = vector.len() as u32;
    
        for d in vector {
            exp -= 1;
            acc += (d as u64) * 10u64.pow(exp);
        }
    
        r += acc;
    }
    
    println!("{r}")
}



pub fn optimal_digit(input: &[u8], allowed_max_digit: usize) -> (u8, &[u8]) {
    let mut max = 0;
    let mut max_index = 0;

    for (i, &digit) in input[..allowed_max_digit+1].iter().enumerate() {
        if digit > max {
            max = digit;
            max_index = i;
        }
    }

    let rest = &input[max_index+1..];
    (max, rest)
}



/// all of question 1 in one function.
pub fn q1(){
    let mut inputs = INPUT.lines().map(|line|{
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
        println!("Line: {} resulted in {}, with first: {} and second: {}",line,res, max_first,max_second);

        res as u32
    });

    let mut sum = 0;
    for i in inputs{
        sum+=i;
    }
    println!("{}",sum);
}