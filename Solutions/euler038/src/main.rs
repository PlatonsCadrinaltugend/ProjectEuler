use itertools::Itertools;
use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut vec = Vec::new();
    for x in 1..=100_000{
        if pandigital_multiples(x) != String::from(""){
            if is_pandigital(pandigital_multiples(x)){
                vec.push(pandigital_multiples(x));
            }
        }
    }
    vec.sort();
    println!("{:?}", vec[vec.len()-1].parse::<i64>().unwrap());
    println!("{:?}", start.elapsed());
}
fn pandigital_multiples(n:i64)->String{
    if n>=1_000_000_000{
        return String::from("");
    }
    let mut string = n.clone().to_string();
    let mut len = string.len();
    let mut k = 2;
    while len<9{
        string += &(k*n).to_string();
        len = string.len();
        k+=1;
    }
    if len == 9{
        return string;
    }
    return String::from("");
}

fn is_pandigital(number:String)->bool{
    if number.len()!=9{
        return false
    }
    let sorted = number.chars().sorted().collect::<String>();
    sorted == "123456789"
}