use std::clone;


use geo::{
    Contains, ContainsProperly, Polygon, Rect, coord     // The structure for your axis-aligned Bounding Box
};

pub fn run(input: &str) -> i64{
   let coordinates: Vec<Vec2>  = {
        input.lines().map(|line|{
            let mut split_line  = line.split(',');
            let first: i64  = split_line.next().unwrap().parse().unwrap();
            let second: i64 = split_line.next().unwrap().parse().unwrap();
            Vec2::new(first, second)
        }).collect()
   };
   // debug_map(coordinates.clone());

   
   let exterior_ring_coords = coordinates.iter()
        .map(|v| coord! { x: v.x as f64, y: v.y as f64 })
        .collect::<Vec<_>>();

   let poly = geo::Polygon::new(exterior_ring_coords.into(), vec![]);


    let mut biggest_area: i64= 0;

   let point_num = coordinates.len();

   for lhs in 0..point_num{
    for rhs in 0..point_num{


        let area1 = area(coordinates[lhs], coordinates[rhs]) ;
        if  area1   > biggest_area{

            let rect = Rect::new(
                coord! { x: coordinates[lhs].x as f64, y: coordinates[lhs].y as f64},
                coord! { x: coordinates[rhs].x as f64, y: coordinates[rhs].y as f64},
            );
            if ! poly.contains(&rect){continue}

            biggest_area = area1;
            println!("New biggest area found: {:?}, {:?}",coordinates[lhs], coordinates[rhs]);
        }
    }
   }

   biggest_area
}



pub fn debug_map(map: Vec<Vec2>){
    let width= map.iter().map(|v|v.x).max().unwrap() as usize +1;
    let height = map.iter().map(|v|v.y).max().unwrap() as usize +1;

    let mut grid = vec![vec![0i32; width]; height];
    for vec in map {
        grid[vec.y as usize][vec.x as usize] =1;
        
        for row in &grid{
            println!("{:?}",row);
        }
        println!("\n\n");
    }
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


