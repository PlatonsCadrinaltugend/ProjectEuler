use std::time::{Instant};
use std::collections::HashSet;
fn main() {
    let start = Instant::now();
    let mut counter = 0;
    let mut current = 2;
    let xth:i64 = 4;
    loop{
        if prime_factors(current).len() == xth as usize{
            counter+=1;
        }
        else{
            counter = 0;
        }
        if counter == xth{
            break
        }
        current +=1;
    }
    println!("{}", current-xth+1);
    println!("{:?}", start.elapsed());
}

fn prime_factors(mut n:i64)->HashSet<i64>{
    let mut vec = HashSet::new();
    while !is_prime(n){
        for x in 2..=((n as f64).sqrt() as i64){
            if is_prime(x){
                if n%x == 0 && n!= x{
                    vec.insert(x);
                    n/=x;
                    break
                }
            }
        }
    }
    vec.insert(n);
    vec
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