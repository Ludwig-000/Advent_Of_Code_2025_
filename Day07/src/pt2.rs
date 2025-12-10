use core::num;



pub fn run(input: &str) -> u128{
    let mut lines: Vec<Vec<char>> =  input.lines().map(|v|v.chars().collect()).collect();
    
    lines[0] = lines[0].iter().map(|c|{ // replace the start chars with emitted lasers
        if *c == 'S'{'|'}else {*c}
    }).collect();

    let mut continuous_split_count = vec![0 as u128; lines[0].len()]; // creates our default vec and adds a 1.
    if let Some(s_index) = lines[0].iter().position(|&c| c == '|') {
        continuous_split_count[s_index] = 1;
        
    }
    //println!("{:?}",continuous_split_count);

    let line_len = lines.len();
    for line_num in 1..line_len{ // skip first line, since we already dealt with it.

        let mut new_split_count  = vec![0 as u128; lines[0].len()];

        let prev_line = &lines[line_num-1].clone();
        let current_line = &mut lines[line_num];

        for char_index in 0..current_line.len(){

            if prev_line[char_index] != '|'{
                continue
            }

            // regular downward move
            if current_line[char_index] != '^'{
                current_line[char_index] = '|';
                new_split_count[char_index] += continuous_split_count[char_index];
            } 
            // split downward move
            else {
                // left split
                if char_index != 0 {
                    current_line[char_index-1] = '|';
                    new_split_count[char_index-1] += continuous_split_count[char_index];
                }
                // right split
                if char_index != current_line.len()-1 {
                    current_line[char_index+1] = '|';
                    new_split_count[char_index+1] += continuous_split_count[char_index];
                }
            }

        }
        continuous_split_count = new_split_count;
        //println!("{:?}",continuous_split_count);
    }



    for line in lines{
        for char in line {
            //print!("{}",char);
        }
        //println!("")
    }

    let mut ret=  0;
    for num in continuous_split_count{
        ret+=num;
    }

    ret
}
