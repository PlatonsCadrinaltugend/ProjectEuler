use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut current = 1;
    let mut last = 1;
    let dividentse = 500;
    loop{
        if dividents(last as f64) > dividentse{
            break
        }
        current +=1;
        last += current;
    }
    println!("{:?}", start.elapsed());
    println!("{}", last);
}

fn dividents(number:f64)-> i64{
    let mut amount = 0;
    for x in 1..=(number.sqrt() as i64){
        if (number as i64)%x==0{
            if x == (number.sqrt() as i64){
                amount +=1;
            }
            else{
                amount+=2;
            }
        }
    }
    amount
}