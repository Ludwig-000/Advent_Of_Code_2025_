/// :( couldn't solve this one. the wording is vague and i can't be bothered to guess the author's intent.

pub fn run(input: &str) -> u128{
    let mut current_junction_count = 0;
    let mut points: Vec<Vec3>=  {
        input.lines().map( |line| {
            let mut split = line.split(',');
            let x: u32 = split.next().unwrap().parse().unwrap();
            let y: u32 = split.next().unwrap().parse().unwrap();
            let z: u32 = split.next().unwrap().parse().unwrap();
            Vec3::new(x as f32, y as f32, z as f32)
        }).collect()
    };

    let num_of_points = points.len();

    loop{
        let mut optimal_connection: (Option<(usize,usize)>, f32) = (None, f32::MAX);

        for point_a in 0..num_of_points{
            if points[point_a].junction.is_some(){ // point_b can still link to point_a. this simply prevents loops.
                continue;
            }
            let p_1=  &points[point_a];
            for point_b in 0..num_of_points{
                let p_2 = &points[point_b];
                if *p_2 == *p_1 {continue;} // prevent a point linking to itself.
                let dist = distance(p_1, p_2);
                if dist <= optimal_connection.1{
                    optimal_connection = (Some((point_a,point_b)), dist);
                }
            }
        }
        if optimal_connection.0.is_none() { // no more connections to be made
            break;
        }
        println!("Optimal value found: {:?}",optimal_connection);


        let vals = optimal_connection.0.unwrap();
        let junction = {

            let mut j = if points[vals.0].junction.is_some() {
                points[vals.0].junction.unwrap()
            }
            else if points[vals.1].junction.is_some() {
                points[vals.1].junction.unwrap()
            } else {
                current_junction_count += 1;
                current_junction_count
            };
            j

        };

        points[vals.0].junction = Some(junction);
        points[vals.1].junction = Some(junction);
    }
    
    println!("{:?}",points);

    let mut res_count: Vec<u32> = vec![0; 1000];
    for entry in  points{
        res_count[entry.junction.unwrap() as usize] +=1;
    }
    
    res_count.sort_by(|a, b| b.cmp(a));

    let mut result  =1;
    result *= (res_count[0] as u128*res_count[1]as u128*res_count[2]as u128);
    result
    
}



pub fn distance(lhs: &Vec3, rhs: &Vec3)-> f32{
    ((rhs.x - lhs.x ).powi(2) + (rhs.y - lhs.y ).powi(2) + (rhs.z - lhs.z ).powi(2)).sqrt()
}

#[derive(Debug,Clone,Copy, PartialEq)]
pub struct Vec3{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub junction: Option<u32>,
}
impl Vec3{
    pub const fn new(x: f32, y: f32, z: f32)-> Vec3{
        Vec3{x,y,z,junction: None}
    }
}
