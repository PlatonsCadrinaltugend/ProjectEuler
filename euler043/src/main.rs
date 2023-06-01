use itertools::Itertools;
use std::time::{Instant};
fn main() {
    let mut sum = 0;
    let start = Instant::now();
    let permutations = vec![0,1,2,3,4,5,6,7,8,9];
    for elem in permutations.iter().permutations(permutations.len()){
        if sub_string_divisibility(combine(elem.clone())){
            sum += combine(elem).parse::<i64>().unwrap();
        }
    }
    println!("{}", sum);
    println!("{:?}", start.elapsed());
}

fn is_n_pandigital(number:String, n:usize)->bool{
    if number.len()!=n+1{
        return false
    }
    let sorted = number.chars().sorted().collect::<String>();
    let mut string = String::from("");
    for x in 0..=n{
        string = string + &x.to_string();
    }
    sorted == string
}

fn sub_string_divisibility(n:String)->bool{
    let s = n.clone();
    if s[1..=3].parse::<i64>().unwrap() %2 != 0{
        return false
    }
    if s[2..=4].parse::<i64>().unwrap() %3 != 0{
        return false
    }
    if s[3..=5].parse::<i64>().unwrap() %5 != 0{
        return false
    }
    if s[4..=6].parse::<i64>().unwrap() %7 != 0{
        return false
    }
    if s[5..=7].parse::<i64>().unwrap() %11 != 0{
        return false
    }
    if s[6..=8].parse::<i64>().unwrap() %13 != 0{
        return false
    }
    if s[7..=9].parse::<i64>().unwrap() %17 != 0{
        return false
    }
    true
}

fn combine(n:Vec<&i32>)->String{
    let mut string = String::from("");
    for elem in n{
        string += &elem.to_string();
    }
    string
}