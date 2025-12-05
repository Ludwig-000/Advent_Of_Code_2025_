



pub fn run(input: &str) -> u64{
    
    
    let mut split= input.split("\r\n\r\n"); // windows specific escape chars. replace with \n\n if something dont work.
    let r = split.next().unwrap();

    let mut ranges: Vec<(u64,u64)>  = r.lines().map(|line|{
        let mut s= line.split("-");
        let first: u64 =  s.next().unwrap().parse().unwrap();
        let second: u64 =  s.next().unwrap().parse().unwrap();
        (first,second)
    }).collect();

    
    //println!("Unsorted input ranged: {:?}", &ranges);

    ranges.sort();

    //println!("Sorted input ranged: {:?}", &ranges);

    
    let mut compiled_ranges: Vec<(u64,u64)> = vec![ranges[0]];

    for range in 0..ranges.len(){
        let current  = ranges[range];

        // we do nothing. the last range fully ecplipses our current one. current range gets ignored.
        if !(compiled_ranges.last().unwrap().1 >= current.1) {

            // last range bites into the current range. we extend last range.
            if (compiled_ranges.last().unwrap().1 >= current.0) {
                compiled_ranges.last_mut().unwrap().1 = current.1
            }
            // last range and current range do not overlap. we add a new range.
            else {
                compiled_ranges.push(current);
            }
        }


    }
    //println!("Compiled ranges: {:?}", compiled_ranges);

    let mut res: u64 = 0;
    for range in compiled_ranges{
        res+= range.1 - range.0+1;
    }

    res

}