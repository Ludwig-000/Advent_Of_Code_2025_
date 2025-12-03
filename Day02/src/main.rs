

const INPUT: &'static str = include_str!("input.txt");

fn main() {

    let split_inputs =  INPUT.rsplit(',');
    let mut ranges: Vec<(u64,u64)> = Vec::new();
    for s in split_inputs.rev(){

        let mut parts = s.splitn(2, '-');
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();
        ranges.push( (left.parse().unwrap(), right.parse().unwrap()));
        
    }
    let mut ret: u64= 0;
    for range in ranges{
        //println!("first: {}, second {}",range.0,range.1);
        let res  = check_range(range.0,range.1);
        ret+=res;

    }
    println!("WE GOT: {}",ret)

    
    
    
}


pub fn check_range(first: u64, second: u64)-> u64{
    let mut res: u64=0;
    for number in first..=second{
        //println!("Checking {}", number);
        let as_string  =number.to_string();


        let res1 = look_at_all_possible_patterns(&as_string);
        if res1{
            //println!("\n!!!!\nWe got a hit on {}\n",number);
            res+=number;
        }
        
    }
    res
    
}


fn look_at_all_possible_patterns(input: &str)-> bool{
    for i in 1..=input.len() {
        let window = &input[..i];

        let r = check_branch(window, input);
        if r {
            return true;
        }
        
    }
    false
}

/// checks if a certain pattern is repeated over a string
fn check_branch(pattern: &str, full_string: &str)-> bool{
    let plen = pattern.len();
    let flen = full_string.len();
    
    if (flen % plen != 0)|| pattern == full_string{
        return false;
    }

    let mut i = 0;
    while i < flen {
        if &full_string[i..i + plen] != pattern {
            return false;
        }
        i += plen;
    }
    //println!("Pattern: {}",pattern);
    true
}