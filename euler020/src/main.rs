use num_bigint::BigUint;

fn main() {
    let mut factorial: BigUint = BigUint::from(1 as u64);
    let mut sum: BigUint = BigUint::from(0 as u64);
    for x in 1..=100{
        factorial*= BigUint::from(x as u64);
    }
    while factorial>= BigUint::from(1 as u64){
        sum += factorial.clone() % BigUint::from(10 as u64);
        factorial = factorial/BigUint::from(10 as u64);
    }
    println!("{}", sum);
}
