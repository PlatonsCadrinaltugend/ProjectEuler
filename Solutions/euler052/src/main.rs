use std::time::{Instant};
use itertools::Itertools;
fn main() {
    let start = Instant::now();
    let mut counter = 1;
    loop{
        if permuted_multiplies(counter.to_string()){
            println!("{}", counter);
            break
        }
        counter+=1;
    }
    println!("{:?}", start.elapsed());
}

fn permuted_multiplies(string:String)->bool{
    if (&string.parse::<i64>().unwrap() * 2).to_string().chars().sorted().collect::<String>() == string.chars().sorted().collect::<String>(){
        if (&string.parse::<i64>().unwrap() * 3).to_string().chars().sorted().collect::<String>() == string.chars().sorted().collect::<String>(){
            if (&string.parse::<i64>().unwrap() * 4).to_string().chars().sorted().collect::<String>() == string.chars().sorted().collect::<String>(){
                if (&string.parse::<i64>().unwrap() * 5).to_string().chars().sorted().collect::<String>() == string.chars().sorted().collect::<String>(){
                    if (&string.parse::<i64>().unwrap() * 6).to_string().chars().sorted().collect::<String>() == string.chars().sorted().collect::<String>(){
                        return true
                    }
                }
            }
        }
    }
    false
}