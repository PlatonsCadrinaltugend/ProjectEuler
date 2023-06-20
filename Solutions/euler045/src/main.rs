use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut counter = 144;
    let mut p_counter = 1;
    let mut t_counter = 1;
    loop{
        let hex = hexagonal(counter);
        while pentagonal(p_counter)<hex{
            p_counter+=1;
        }
        while triangle(t_counter)<hex{
            t_counter+=1;
        }
        if hex ==pentagonal(p_counter) && hex == triangle(t_counter){
            break
        }
        counter+=1;
    }
    println!("{}", hexagonal(counter));
    println!("{:?}", start.elapsed());
}

fn triangle(n:i64)->i64{
    n*(n+1)/2
}

fn  pentagonal(n:i64)->i64{
    n*(3*n-1)/2
}
fn hexagonal(n:i64)->i64{
    n*(2*n-1)
}