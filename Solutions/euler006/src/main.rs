use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let amount = 100;
    let difference = sumsquare(amount) - squaresums(amount);
    println!("{}", difference);
    println!("{:?}", start.elapsed());

}

fn squaresums(number: i64) -> i64{
    let mut sum = 0;
    for x in 1..=number{
        sum+= x*x;
    }
    sum
}

fn sumsquare(number:i64) -> i64{
    let mut sum = 0;
    for x in 1..=number{
        sum +=x
    }
    sum*sum
}