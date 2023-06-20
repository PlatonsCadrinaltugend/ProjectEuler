use itertools::Itertools;
use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let primes = prime_numbers(7_654_321);
    for x in primes.into_iter().rev(){
        if is_n_pandigital(x.to_string(), x.clone().to_string().len()){
            println!("{}", x);
            break
        }
    }
    println!("{:?}", start.elapsed());
}

fn is_n_pandigital(number:String, n:usize)->bool{
    if number.len()!=n{
        return false
    }
    let sorted = number.chars().sorted().collect::<String>();
    let mut string = String::from("");
    for x in 1..=n{
        string = string + &x.to_string();
    }
    sorted == string
}
fn prime_numbers(number: i64) -> Vec<i64>{
    
    let mut prime_factors = vec![];
    let mut amount_factors = 0;
    let mut done = 0;
    let mut x:f64 = 1.0;
    loop{
        let number2 = x.clone().sqrt() as i64;
        for y in 1..=number2{
            if (x as i64)%y == 0{
                amount_factors +=1;
                if amount_factors>1{
                    break
                }
            }
        }
        if amount_factors == 1 && x != 1.0{
            prime_factors.push(x as i64);
        }
        amount_factors = 0;
        x+=1.0;
        done +=1;
        if done == number{
            break
        }
    }
    prime_factors
}