use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut max = 0;
    let mut xx = 0;
    for x in 1..=1_000_000{
        let new = collatz(x);
        if new>max{
            max = new;
            xx = x;
        }
    }
    println!("{}", xx);
    println!("{:?}", start.elapsed());
}

fn collatz(mut number:u64) -> u64{
    let mut terms = 1;
    while number != 1{
        if number%2 == 0{
            number /=2;
        }
        else{
            number = number*3 +1;
        }
        terms +=1;
    }
    terms
}