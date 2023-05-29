use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut sum = 0;
    for x in 3..=10_000_000{
        if factorial_sums(x){
            sum +=x;
        }
    }
    println!("{}", sum);
    println!("{:?}", start.elapsed());
}

fn factorial_sums(number:u64)->bool{
    let mut sum = 0;
    let mut current = number.clone();
    while current>=1{
        sum += factorial(current%10);
        current/=10;
    }
    sum ==number
}
fn factorial(number:u64)->u64{
    let mut factorial = 1;
    let mut current = number.clone();
    while current>1{
        factorial*=current;
        current-=1;
    }
    factorial
}