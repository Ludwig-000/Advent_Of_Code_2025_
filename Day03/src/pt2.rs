


pub fn run(puzzle_input: &str)-> u64 {

    let inputs: Vec<Vec<u8>> = puzzle_input.lines().map(|line|{
        line.as_bytes().iter().map(|&byte| byte - b'0').collect()
    }).collect();

    
    let mut all_values = inputs.into_iter().map(|value| {

        let mut resulting_num = Vec::new();

        let mut test: &[u8] = &value;
        //println!("Our test data: {:?}",test);
        for i in 0..=11{


            let res: (u8, &[u8]) = optimal_digit(test, test.len()-(12-i));
            test = res.1;

            //println!("round: {}. our number: {}, new slice: {:?}", i+1,res.0, res.1);
            test = res.1;
            resulting_num.push(res.0);

        }
        resulting_num
    });

    // all the summing.
    let res = all_values.map(|vector|{
        //println!("working on {:?}",vector);
        let mut acc = 0u64;
        let mut exp = vector.len() as u32;
    
        for d in vector {
            exp -= 1;
            acc += (d as u64) * 10u64.pow(exp);
        }
        acc
    }).sum();
    
    
    
    //println!("{r}");

    res
}

fn optimal_digit(input: &[u8], allowed_max_digit: usize) -> (u8, &[u8]) {
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