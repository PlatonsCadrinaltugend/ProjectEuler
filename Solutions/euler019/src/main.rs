use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut sundays = 0;
    let mut current_monthstart = 2;
    for x in 1901..=2000{
        for y in 1..=12{ 
            if y==9 || y==4 || y==6 || y==11 {
                current_monthstart = (current_monthstart + 2)%7;
            }
            else if y==2{
                if x%4 == 0 && x%100!=0 || x%400 == 0{

                    current_monthstart = (current_monthstart + 1)%7;
                }
            }
            else{
                current_monthstart = (current_monthstart +3)%7;
            }
            if current_monthstart == 0{
                sundays+=1;
            }
        }
    }
    println!("{}", sundays);
    println!("{:?}", start.elapsed());
}
