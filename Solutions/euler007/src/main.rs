use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let amount = 10_001;
    println!("{}", prime_numbers(amount).last().unwrap());
    println!("{:?}", start.elapsed());
}

fn prime_numbers(number: i64) -> Vec<i64>{
    let mut prime_factors = vec![];
    let mut done = 0;
    let mut x = 1;
    loop{
        if is_prime(x){
            prime_factors.push(x);
            done+=1;
        }
        x+=1;
        if done == number{
            break
        }
    }
    prime_factors
}
fn is_prime(number:i64)->bool{
    if number<2{
        return false;
    }
    for x in 2..=((number as f64).sqrt() as i64){
        if number%x == 0 && number !=2{
            return false;
        }
    }
    true
}