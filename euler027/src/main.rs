use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let blist = prime_numbers(1000);
    let mut max_a = 0;
    let mut max_b = 0;
    let mut max_primes = 0;
    for a in -999..1000{
        for b in &blist{

            let mut n = 0;
            while is_prime(n*n + a*n + b){
                n+=1;
            }
            if n> max_primes{
                max_primes = n;
                max_a =a;
                max_b = *b;
            }

        }
    }
    println!("a:{} b:{} max:{} product:{}",max_a,max_b, max_primes, max_a*max_b);
    println!("{:?}", start.elapsed());
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

fn is_prime(number:i64) -> bool{
    if number<0{
        return false;
    }
    for x in 2..=((number as f64).sqrt() as i64){
        if number%x == 0 && number !=2{
            return false;
        }
    }
    true
}