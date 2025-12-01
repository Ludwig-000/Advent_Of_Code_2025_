
const INPUT: &'static str = include_str!("full_input.txt");

pub struct Lock {
    value: u8,
    zero_count: u32,
}
impl Lock{
    pub fn new(starting_value: u8)-> Lock{
        let zero_count = if starting_value == 0{1}else {0};
        Lock{
            value: starting_value,
            zero_count,
        }
    }
    pub fn move_by(&mut self, val: i32){
        let mut val = val;

        if val > 0 {
            while val !=0 {
                self.up_one();
                val -=1;
            }
        }
        if val < 0 {
            while val !=0 {
                self.down_one();
                val +=1;
            }
        }
        
    }
    fn down_one(&mut self){
        if self.value == 0 {
            self.value = 99;
        }else {  self.value -= 1; }

        if self.value ==0 {
            self.zero_count +=1;
        }
    }
    fn up_one(&mut self){
        if self.value == 99 {
            self.value = 0;
        }else {  self.value += 1; }

        if self.value ==0 {
            self.zero_count +=1;
        }
    }
}
fn main() {

    let start = std::time::Instant::now();
    let mut my_lock = Lock::new(50);

    for line in INPUT.lines(){

        let (direction,value)=  line.split_at(1);

        let mut val: i32 = value.parse().unwrap();
        match direction{
            "L" => {val*= -1;},
            _ => {},
        }

        my_lock.move_by(val);

    }
    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start);
    println!("Execution time in microseconds: {} micro seconds", duration.as_micros());
    println!("solution: {}",my_lock.zero_count);
}
