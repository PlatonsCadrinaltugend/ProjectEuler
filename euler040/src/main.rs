use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let vec = vec![1, 10,100,1_000, 10_000, 100_000, 1_000_000];
    let mut product = 1;
    let mut counter = 0;
    let mut newstart = 1;
    for elem in vec{
        for x in newstart..=elem{
            if x.to_string().len() + counter>=elem{
                product *= get_xth_digit(x.clone().try_into().unwrap(), (elem.clone()-counter.clone()).try_into().unwrap());
            }
            counter += x.to_string().len();
            if counter>=elem{
                newstart = x+1;
                break
            }
        }
    }
    println!("{}", product);
    println!("{:?}", start.elapsed());
}

fn get_xth_digit(n:i64, k:i64) -> i64{
    let mut string = n.clone().to_string();
    let mut counter = k.clone();
    if counter>string.len().try_into().unwrap(){
        return 1
    }
    while counter > 1{
        string = (&string[1..]).to_string();
        counter-=1;
    }
    string[0..1].parse::<i64>().unwrap()
}