use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut digit:i64 = 28433;
    for _x in 0..7830457{
        digit = (digit * 2)%10_000_000_000;
    }
    println!("{}", digit +1);
    println!("{:?}", start.elapsed());
}
