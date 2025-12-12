use std::u32;


pub static mut LOWEST: u32 = u32::MAX;

pub fn run(input: &str) -> u32{
   let mut buttons: Vec<Button_schematics> = parse_input(input);

    println!("{:?}",buttons);
    let mut count = 0;
    for pannel in buttons {
        
        let pannel_size = pannel.indicator_lights.len();

        unsafe {
            for i in 1..100{
                recursive_search(vec![false;pannel_size], &pannel.indicator_lights, &pannel.button_wiring, 0,i);
                if LOWEST != u32::MAX {break;}
            }
            let c = LOWEST;

            println!("{}",c);

            assert!(LOWEST != u32::MAX);
            count+= LOWEST;
            LOWEST = u32::MAX;
        }
    }

    count
}


pub unsafe fn recursive_search(mut lights: Vec<bool>, light_goal: &[bool], wiring: &[Vec<u8>], depth: u32, max_depth: u32){
    if depth >= LOWEST { return }
    if depth > max_depth { return }
    if &lights == light_goal {
        LOWEST =depth;
        return;
    }
    
    // search recursively
    for options in wiring {
        // apply wiring
        let mut cloned_lights  = lights.clone();
        apply_wiring(&mut cloned_lights, options);
        recursive_search(cloned_lights, light_goal, wiring, depth+1, max_depth);
    }

    
}


pub fn apply_wiring(lights: &mut Vec<bool>, wiring: &[u8]){
    for wire in wiring {
        let index = *wire as usize;
        let new_state = !lights[index];
        lights[index] = new_state;
    }
}


#[derive(Clone, Debug)]
pub struct Button_schematics{
    pub indicator_lights: Vec<bool>,
    pub button_wiring: Vec<Vec<u8>>,
}


pub fn parse_input(input: &str)-> Vec<Button_schematics>{
    input.lines().map(|line|{
        let mut split = line.split_ascii_whitespace();

        let indicators: Vec<bool> = split.next().unwrap().chars()
            .filter(|c|{
                *c != '[' && *c != ']'
            })

            .map(|c|{
                    if c == '.' {false} else {true}

        }).collect();

        let wiring: Vec<Vec<u8>>  = split.into_iter()
            .filter(|entry|{
                !((*entry).contains('{'))
            }
                
            ).map(|split|{


            let parsed: Vec<u8>= split.chars().filter(|c|{
                *c != '(' && *c != ')'&& *c != ','
            }).map(|num|{
                num.to_digit(10)
                    .map(|digit| digit as u8).unwrap()
            }).collect();

            parsed
        }).collect();


        Button_schematics { indicator_lights: indicators, button_wiring: wiring }
    }).collect()
}