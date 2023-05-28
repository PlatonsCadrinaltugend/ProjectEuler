use itertools::Itertools;
use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let permutations = vec![0,1,2,3,4,5,6,7,8,9];
    if let Some(solution) =permutations.iter().permutations(permutations.len()).nth(999_999){
        println!("{:?}", solution);
}
    println!("{:?}", start.elapsed());
}