
pub fn run(input: &str) -> i64{
   let coordinates: Vec<Vec2>  = {
        let split: Vec<_> = input.lines().map(|line|{
            let mut split_line  = line.split(',');
            let first: i64  = split_line.next().unwrap().parse().unwrap();
            let second: i64 = split_line.next().unwrap().parse().unwrap();
            Vec2::new(first, second)
        }).collect();
        split
   };


   let mut biggest_area: i64= 0;

   let point_num = coordinates.len();
   for lhs in 0..point_num{
    for rhs in 0..point_num{
        let area1 = area(coordinates[lhs], coordinates[rhs]) ;
        if  area1   > biggest_area{
            biggest_area = area1;
            //println!("New biggest area found: {:?}, {:?}",coordinates[lhs], coordinates[rhs]);
        }
    }
   }

   biggest_area
}



pub fn area(lhs: Vec2, rhs: Vec2)-> i64 {
    let w = (lhs.x-rhs.x).abs()+1;
    let h = (lhs.y-rhs.y).abs()+1;
    w*h
}


#[derive(Clone, Copy, Debug)]
pub struct Vec2{
    x: i64,
    y: i64,
}
impl Vec2{
    pub const fn new(x: i64, y: i64)-> Vec2{
        Vec2{x,y}
    }
    
}