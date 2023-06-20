use std::fs;
use std::time::{Instant};
static ASCII_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e','f', 'g', 'h', 'i', 'j','k', 'l', 'm', 'n', 'o','p', 'q', 'r', 's', 't','u', 'v', 'w', 'x', 'y','z'];
fn main() {
    let start = Instant::now();
    let mut vec = Vec::new();
    vec.push(1.0);
    let mut max_n = 1;
    let mut max = 0.0;
    let mut sum = 0;
    
    let contents = fs::read_to_string(r"C:\Users\Domin\ProjectEuler\Required Files\0042_words.txt").expect("REASON").to_lowercase();
    let parts = contents.split("\"");
    let mut collection: Vec<&str> = parts.collect();
    collection.retain(|x| *x != ",");
    collection.retain(|x| *x != " ");
    collection.retain(|x| *x != "");
    for elem in collection{
        if char_sum(elem)<=max{
            if vec.contains(&char_sum(elem)){
                sum +=1;
            }
        }
        else{
            while char_sum(elem)>max {
                max_n +=1;
                vec.push(t(max_n as f64));
                max = t(max_n as f64);
            }
            if vec.contains(&char_sum(elem)){
                sum +=1;
            }
        }
    }
    println!("{}", sum);
    println!("{:?}", start.elapsed());
}

fn t(n:f64)->f64{
    0.5 * n * (n+1.0)
}

fn char_sum(n:&str)-> f64{
    let mut sum = 0.0;
    for c in n.chars(){
        sum += (ASCII_LOWER.iter().position(|&r| r == c).unwrap() +1) as f64;
    }
    sum
}