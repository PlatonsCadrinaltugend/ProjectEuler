use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let number: f64 = 600851475143.0;
    let mut primes: Vec<i64> = prime_numbers(&number);
    primes.reverse();
    let number2: i64 = number as i64;
    for elem in primes {
        if number2 %elem == 0{
            println!("{}", elem);
            break
        }
    }
    println!("{:?}", start.elapsed());
}

fn prime_numbers(number: &f64) -> Vec<i64>{
    let number2 = number.sqrt() as i64;
    let mut prime_factors = vec![];
    for x in 1..number2{
        if is_prime(x) && (*number as i64)%x == 0{
            prime_factors.push(x);
        }
    }
    prime_factors
}

fn is_prime(number:i64) -> bool{
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