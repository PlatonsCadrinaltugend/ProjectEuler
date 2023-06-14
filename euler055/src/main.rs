use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut amount = 0;
    let border = 10_000;
    'outer: for x in 1..border{
        let mut currentsum:u128 = lychrel(x);
        for _y in 1..=50{
            if is_palindrom(currentsum){
                continue 'outer
            }

            currentsum = lychrel(currentsum);
        }
        amount+=1;
    }
    println!("{}", amount);
    println!("{:?}", start.elapsed());
}

fn lychrel(n:u128)-> u128{
    let s = n.clone().to_string().chars().rev().collect::<String>().parse::<u128>().unwrap();
    s+n
}

fn is_palindrom(n:u128)->bool{
    n.to_string().chars().rev().collect::<String>() == n.to_string()
}