use std::collections::HashSet;
use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut sum =0;
    let mut newvec = HashSet::new();
    let mut current = 0;
    while newvec.len()<15{
        if is_prime(current){
            if trunctable(current){
                newvec.insert(current);
            }}
        current+=1;
    }
    for elem in &newvec{
        sum+=elem;
    }
    println!("{:?}", start.elapsed());
    println!("{:?}", newvec);
    println!("{}", sum -(2+3+5+7));
}

fn trunctable(n:i64)->bool{
    let mut s = n.to_string();
    if s.len()==1 && s != 1.to_string() && is_prime(n){
        return true
    }
    let this = s.clone().len();
    let mut z = s.clone();
    let that = z.clone().len();
    
    for _x in 1..=this{
        if is_prime(s.parse::<i64>().unwrap()){
            s = s[1..].to_string();
        }
        else{
            return false
        }
    }
    for x in (1..=that).rev(){
        if is_prime(z.parse::<i64>().unwrap()){
            z = z[0..(x-1)].to_string();
        }
        else{
            return false
        }
        
    }
    true
    
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