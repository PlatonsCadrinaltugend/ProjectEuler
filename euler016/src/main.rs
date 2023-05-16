use num_bigint::BigUint;
use num_bigint;
fn main() {
    let mut number:BigUint = BigUint::from(2 as u64);
    let mut sum = BigUint::from(0 as u64);
    number = number.pow(1000);
    while number>=BigUint::from(1 as u64){
        sum += number.clone() % BigUint::from(10 as u64);
        number/=BigUint::from(10 as u64);
    }
    println!("{}", sum);    

}
