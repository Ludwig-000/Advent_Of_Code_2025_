use core::num;



pub fn run(input: &str) -> u128{
    
    let operations: Vec<char>  =input.lines().last().unwrap().chars().filter(|&ch| ch == '+' || ch == '*').collect();

    let nums: Vec<Vec<u64>> = input.lines().filter(|&line| !line.contains('+')).map(|line|{

        let split: Vec<u64> = line.split_ascii_whitespace().map(|num|{
            let number: u64 = num.parse().unwrap();
            number
        }).collect();
        split

    }).collect();


    //println!("Ops: {:?}", operations);
    //println!("NUMS: {:?}", nums);

    
    let mut total: u128 =  0;
    for i in 0..operations.len(){

        let mut line_res: u64 =  0;
        if operations[i] == '+'{
            //print!("op is: +");
            for vecc in &nums{
                //print!(" {:?}", vecc[i]);
                line_res += vecc[i];
            }
        }
        else if operations[i] == '*'{
            line_res+=1; // to multiply with
            //print!("op is: *");
            for vecc in &nums{
                //print!(" {:?}", vecc[i]);
                line_res *= vecc[i];

            }
        }
        else {
            panic!("unknown operations: {}",operations[i])
        }
        //println!("   Line Res: {}",line_res);

        total += line_res as u128;
    }
    total
}