use std::collections::HashSet;
use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut vec =HashSet::new();
    vec.insert(2);
    vec.insert(3);
    let mut current = 3;
    'outer: loop{
        current+=1;
        if is_prime(current){
            vec.insert(current);
            continue
        }
        for elem in &vec{
            if is_double_square(current -*elem){
                continue 'outer
            }
        }
        break
    }
    println!("{}", current);
    println!("{:?}", start.elapsed());
    
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

fn is_double_square(i:i64)->bool{
    ((i/2) as f64).sqrt()%1.0 == 0.0
}