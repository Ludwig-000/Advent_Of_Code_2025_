

#[derive(Clone, Debug)]
pub enum Alignment{
    Left,
    Right,
}


pub fn run(input: &str) -> u64{
    

    let (operations, nums, alignment) = extract_collums(input);

    // println!("NUMS: {:?}", nums);
    let mut total: u64 = 0;
    for op in 0..operations.len(){
        if operations[op] == '*'{
            match alignment[op]{
                Alignment::Left => {  
                    total+= calculate_lhs(&nums[op], Op::Mul);
                },
                Alignment::Right => {
                    total+= calculate_rhs(&nums[op], Op::Mul);
                },
            }
            
        }
        else if operations[op] == '+'{
            match alignment[op]{
                Alignment::Left => {  
                    total+= calculate_lhs(&nums[op], Op::Plus);
                },
                Alignment::Right => {
                    total+= calculate_rhs(&nums[op], Op::Plus);
                },
            }
        }
        else {
            panic!("unknown operation {}",operations[op]);
        }
    }
    total
}



pub fn extract_collums(input: &str)-> (Vec<char>, Vec<Vec<Vec<char>>>, Vec<Alignment>){

    let op_vec: Vec<char>  = input.lines().last().unwrap().chars().collect();

    let raw_lines = input.lines().filter(|&line| !line.contains('+'));


    let alignment_vec: Vec<Alignment>=  {
        let mut v = Vec::new();
        let first: Vec<_> = raw_lines.clone().next().unwrap().chars().collect();
        let last: Vec<_> = raw_lines.clone().last().unwrap().chars().collect();
        for i in 0..op_vec.len(){
            if op_vec[i] == ' '{
                continue;
            }
            if last[i] == ' '{
                v.push(Alignment::Right);
            }
            else if first[i] == ' '{
                v.push(Alignment::Right);
            }
            else {
                v.push(Alignment::Left);
            }
        }
        v
    };

    //println!("{:?}", alignment_vec.clone());

    let all_collected_lines: Vec<Vec<Vec<char>>>  = raw_lines.map(|line| {
        let l: Vec<_> = line.split_ascii_whitespace().collect();
        let c: Vec<_> = l.into_iter().map(|word|{
            word.chars().collect()
        }).collect();
        c
    }).collect();


    //println!("Raw_nums: {:?}", all_collected_lines);

    // Transpositions the vec, switching collums and rows
    let mut transpositioned_vec: Vec<Vec<Vec<char>>>= Vec::new();

    for width in 0..all_collected_lines[0].len(){

        let mut reshaped_nums = Vec::new();
        for height in 0..all_collected_lines.len(){

            reshaped_nums.push(all_collected_lines[height][width].clone());

        }
        transpositioned_vec.push(reshaped_nums);
    }
    
    let cleaned_operation_vec: Vec<char> = op_vec.into_iter().filter(|character| !(*character == ' ')).collect();
    ( cleaned_operation_vec, transpositioned_vec, alignment_vec )

}

pub enum Op{
    Mul,
    Plus,
}



pub fn calculate_lhs(input: & Vec<Vec<char>>, op: Op)-> u64{
    //println!("PRE-CALC INPUT : {:?}", &input);

    let max_size=  max_len(input);

    let mut actual_numbers_we_have_to_calc: Vec<u64> = Vec::new();

    for index in 0..max_size{
        let mut temp_char_vec=  Vec::new();
        for number in input{
            if number.len() >= (max_size-index){
                temp_char_vec.push(number[(max_size-index)-1]);
            }
        }


        actual_numbers_we_have_to_calc.push({
            let as_string: String = temp_char_vec.iter().collect();
            as_string.parse().unwrap()
        });
         //println!("TEMP VALUE AFTER PARSING: {:?}", &temp_char_vec);
    }

    
     //println!("Actual numbers : {:?}", actual_numbers_we_have_to_calc);

     return match op {
        Op::Plus => {
            let mut result=  0;
            for num in actual_numbers_we_have_to_calc{
                result+=num;
            }
            result
        },
        Op::Mul => {
            let mut result=  1;
            for num in actual_numbers_we_have_to_calc{
                result*=num;
            }
            result
        },
    };
}




pub fn calculate_rhs(input: & Vec<Vec<char>>, op: Op)-> u64{
     //println!("PRE-CALC INPUT : {:?}", &input);

    let max_size=  max_len(input);

    let mut actual_numbers_we_have_to_calculate: Vec<u64> = Vec::new();

    for col_index in 0..max_size{
        let mut temp_char_vec=  Vec::new();

        for row in input{
            let padding = max_size - row.len();
            if col_index >= padding{

                let char_idx = col_index - padding;
                temp_char_vec.push(row[char_idx]);
            }
        }


        actual_numbers_we_have_to_calculate.push({
            let as_string: String = temp_char_vec.iter().collect();
            as_string.parse().unwrap()
        });
        // println!("TEMP VALUE AFTER PARSING: {:?}", &temp_char_vec);
    }

    
    // println!("Actual numbers : {:?}", actual_numbers_we_have_to_calculate);

    return match op {
        Op::Plus => {
            let mut result=  0;
            for num in actual_numbers_we_have_to_calculate{
                result+=num;
            }
            result
        },
        Op::Mul => {
            let mut result=  1;
            for num in actual_numbers_we_have_to_calculate{
                result*=num;
            }
            result
        },
    };
    
}

fn max_len(input: &Vec<Vec<char>>)-> usize{
    let mut max = 0;
    for i in input{
        if i.len() > max{
            max= i.len();
        }
    }
    max
}


