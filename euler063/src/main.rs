use std::time::{Instant};
use num_bigint::BigUint;
fn main() {
    let start = Instant::now();
    let mut number = 0;
    let mut changes = true;
    let mut amount = 0;
    loop{
        for x in 1..=9{
            if (BigUint::from(x as u32)).pow(amount).to_string().len() == amount as usize{
                number +=1;
                changes = true;
            }
        }
        if !changes{
            break
        }
        amount +=1;
        changes = false;

    }
    println!("{}", number);
    println!("{:?}", start.elapsed());
}
