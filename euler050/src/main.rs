use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let border = 1_000_000;
    let mut amount = 0;
    let mut current;
    let vec = prime_numbers(border);
    let mut max = 0;
    for x in 0..vec.len(){
        let mut sum = 0;
        current = 0;
        for y in x..vec.len(){
            sum +=vec[y];
            current+=1;
            if sum> border{
                break
            }
            if is_prime(sum) && sum >= max && current>amount{
                max = sum;
                amount = current;
            }
        }
        if x as i64>border{
            break
        }
    }
    println!("{} {}", max, amount);
    println!("{:?}", start.elapsed());
}


fn prime_numbers(i:i64)->Vec<i64>{
    let mut vec = Vec::new();
    for x in 2..=i{
        if is_prime(x){
            vec.push(x);
        }
    }
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