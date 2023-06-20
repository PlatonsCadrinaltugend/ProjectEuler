use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut amount = 0;
    for x in 1..=10_000_000{
        if digit_loop_end(x){
            amount+=1;
        }
    }
    println!("{}", amount);
    println!("{:?}", start.elapsed());
}

fn digit_loop_end(mut i:i64)->bool{
    while i!=1 &&i!=89{
        let mut current = 0;
        while i>=1{
            current+=(i%10)*(i%10);
            i = i/10;
        }
        i = current;
    }
    i==89
}