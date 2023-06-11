use num_bigint::BigUint;
use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut amount = 0;
    for x in 1..=100{
        for y in 1..x{
            if binom_bigger_1m(x,y){
                amount +=1;
            }
        }
    }
    println!("{amount}");
    println!("{:?}", start.elapsed());
}

fn binom_bigger_1m(n:i64, k:i64)->bool{
    let mut denominator = BigUint::from(1 as u64);
    for elem in 1..=(n-k){
        denominator *=BigUint::from(elem as u64);
    }
    let mut numerator = BigUint::from(1 as u64);
    for elem in (k+1)..=n{
        numerator *=BigUint::from(elem as u64);
    }
    numerator/denominator> BigUint::from(1_000_000 as u64)
}