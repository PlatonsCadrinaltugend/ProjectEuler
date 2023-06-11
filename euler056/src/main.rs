use num_bigint::BigUint;
use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut max = BigUint::from(0 as u64);
    for a in 1..=100{
        for b in 1..=100{
            if digit_sum(BigUint::from(a as u64).pow(b)) > max{
                max = digit_sum(BigUint::from(a as u64).pow(b));
            }
        }
    }
    println!("{max}");
    println!("{:?}", start.elapsed());
}

fn digit_sum(mut n:BigUint)->BigUint{
    let mut sum = BigUint::from(0 as u64);
    while n>=BigUint::from(1 as u64){
        sum += n.clone()%BigUint::from(10 as u64);
        n/=BigUint::from(10 as u64);
    }
    sum
}