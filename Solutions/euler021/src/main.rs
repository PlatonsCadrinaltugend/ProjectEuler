use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut sum = 0;
    let mut vec = Vec::new();
    vec.push(0);
    for a in 1..=10_000{
        vec.push(d(a));
    }
    for x in 0..10_000{
        if vec.contains(&vec[x]){
            let pos = vec.iter().position(|&r| r == vec[x]).unwrap();
            if vec.contains(&vec[pos]) && vec[x] != x.try_into().unwrap(){
                if d(x as u128) == vec[pos] && x == d(vec[pos]).try_into().unwrap(){
                    sum +=vec[x];
                }
            }
        }
    }
    println!("{}", sum);
    println!("{:?}", start.elapsed());
}
fn d(n: u128)-> u128{
    let mut sum = 0;
    for x in 1..=(n/2){
        if n%x == 0{
            sum += x;
        }
    }
    sum
}
