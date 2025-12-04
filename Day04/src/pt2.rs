

pub fn run(input: &str)-> u64{

    let mut full_count = 0;

    let mut full: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    loop {
        let cloned_full: Vec<Vec<char>> = full.clone();
        


        for col in 0..cloned_full.len() {
            for row in 0..cloned_full[0].len() {
                if cloned_full[col][row] == '.' {
                    continue;
                }

                // check all lefties
                let top_left = safe_get(&cloned_full, col as i32 -1, row as i32 -1);
                let left = safe_get(&cloned_full, col as i32, row as i32 -1);
                let bot_left = safe_get(&cloned_full, col as i32+1, row as i32 -1);

                // check middle
                let top = safe_get(&cloned_full, col as i32 -1, row as i32);
                //let middle = safe_get(&full, col as i32, row as i32);
                let bottom = safe_get(&cloned_full, col as i32 +1, row as i32);

                // check right
                let top_right = safe_get(&cloned_full, col as i32 -1, row as i32 +1);
                let right = safe_get(&cloned_full, col as i32, row as i32 +1);
                let bot_right = safe_get(&cloned_full, col as i32+1, row as i32 +1);

                let all= vec![top_left, left, bot_left,top,bottom,top_right,right,bot_right];


                let count: i32   = all.iter().map(|char|{
                    if *char == '@'{1}else{0}
                }).sum();
                
                if count < 4{
                    full_count+=1;
                    full[col][row] = '.';
                }
                
            }
        }
        
        if cloned_full == full{
            break;
        }

    }
    full_count as u64
}



pub fn safe_get(input: &Vec<Vec<char>>, col: i32, row: i32) -> char {
    if col < 0 { return '.'; }
    if row < 0 { return '.'; }

    let col_u = col as usize;
    let row_u = row as usize;

    if col_u >= input.len() { return '.'; }
    if row_u >= input[col_u].len() { return '.'; }
    
    input[col_u][row_u]
}