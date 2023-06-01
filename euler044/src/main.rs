use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut vec = Vec::new();
    vec.push(1);
    let mut current = 1;
    let mut amount = 1;
    'outer: loop{
        let currentnumber = pentagonal(current);
        vec.push(currentnumber);
        for elem in vec.clone(){
            if vec.contains(&(currentnumber - elem)){
                while pentagonal(amount)<currentnumber+elem{
                    vec.push(pentagonal(amount));
                    amount+=1;
                }
                if vec.contains(&(currentnumber + elem)){
                    println!("{}", currentnumber - elem);
                    break 'outer
                }
            }
        }
        current+=1;
    }
    println!("{:?}", start.elapsed());
}

fn pentagonal(n:i64)->i64{
    n*(3*n-1)/2
}