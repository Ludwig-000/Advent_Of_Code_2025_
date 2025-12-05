use core::num;



pub fn run(input: &str) -> u64{
    
    let (ranges, numbers) = {
        let mut split= input.split("\r\n\r\n"); // windows specific escape chars. replace with \n\n if something dont work.
        let r = split.next().unwrap();
        let n =  split.next().unwrap();

        let ranges: Vec<(u64,u64)>  = r.lines().map(|line|{
            let mut s= line.split("-");
            let first: u64 =  s.next().unwrap().parse().unwrap();
            let second: u64 =  s.next().unwrap().parse().unwrap();
            (first,second)
        }).collect();

        let numbers: Vec<u64>  = n.lines().map(|line|{
            let num: u64 =line.parse().unwrap();
            num
        }).collect();
        (ranges,numbers)
    };

    let res: u64 = numbers.into_iter().map(|num|{
        let mut is_in_range = false;
        for range in &ranges {

            if num >= range.0 && num <= range.1{
                is_in_range = true;
                break;
            }

        }
        if is_in_range {1}else {0}
    }).sum();



    res

}