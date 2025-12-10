use core::num;



pub fn run(input: &str) -> u128{
    let mut split_count = 0;
    let mut lines: Vec<Vec<char>> =  input.lines().map(|v|v.chars().collect()).collect();

    lines[0] = lines[0].iter().map(|c|{ // replace the start chars with emitted lasers
        if *c == 'S'{'|'}else {*c}
    }).collect();

    let line_len = lines.len();
    for line_num in 1..line_len{ // skip first line, since we already dealt with it.

        let prev_line = &lines[line_num-1].clone();
        let current_line = &mut lines[line_num];

        for char_index in 0..current_line.len(){

            if prev_line[char_index] != '|'{
                continue
            }

            // regular downward move
            if current_line[char_index] != '^'{
                current_line[char_index] = '|';
            } 
            // split downward move
            else {
                split_count+=1;
                // left split
                if char_index != 0 {
                    current_line[char_index-1] = '|';
                }
                // right split
                if char_index != current_line.len()-1 {
                    current_line[char_index+1] = '|';
                }
            }

        }
    }



    for line in lines{
        for char in line {
            print!("{}",char);
        }
        println!("")
    }

    split_count
}
