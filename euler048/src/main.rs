use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut sum:i64 = 0;
    for x in 1..=1000{
        let mut currentsum:i64 = x;
        for _y in 1..x{
            currentsum = (currentsum * x) % 10_000_000_000 ;
        }
        sum  = (sum + currentsum) %10_000_000_000;
    }
    println!("{}", sum);
    println!("{:?}", start.elapsed());
}
