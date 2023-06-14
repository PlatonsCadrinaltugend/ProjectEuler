use itertools::Itertools;
use std::time::{Instant};
fn main() {
    let start = Instant::now();
    'outer: for x in (1001..=9999).step_by(2){
        'middle: for y in (1001..=x).step_by(2){
            for z in (1001..=y).step_by(2){  
                if x==y || y==z || z==x{
                    continue 'middle
                }
                if x-y != y-z{
                    continue
                }
                if !is_prime(x){
                    continue 'outer
                }
                if !is_prime(y){
                    continue 'middle
                }
                if !is_prime(z){
                    continue
                }
                if permutation(x,y){
                    if permutation(y,z){
                        if z.to_string() + &y.to_string() + &x.to_string() != "148748178147".to_string(){
                            println!("{}", z.to_string() + &y.to_string() + &x.to_string());
                            break 'outer
                        }
                    }
                }
                else{
                    continue 'middle
                }

            }
        }   
    }
    println!("{:?}", start.elapsed());
}

fn permutation(i:i64, n:i64)->bool{
    i.to_string().chars().sorted().collect::<String>() == n.to_string().chars().sorted().collect::<String>()
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