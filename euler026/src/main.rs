use num_bigint::BigUint;
fn main() {
    let mut largest_cycle = 0;
    let mut length_largest = 0;
    for x in prime_numbers(1000){
        if multiplicative_order(x) > length_largest{
            length_largest = multiplicative_order(x);
            largest_cycle = x;
        }
    }
    println!("{}", largest_cycle);
}

fn prime_numbers(number: i64) -> Vec<i64>{
    
    let mut prime_factors = vec![];
    let mut amount_factors = 0;
    let mut done = 0;
    let mut x:f64 = 1.0;
    loop{
        let number2 = x.clone().sqrt() as i64;
        for y in 1..=number2{
            if (x as i64)%y == 0{
                amount_factors +=1;
                if amount_factors>1{
                    break
                }
            }
        }
        if amount_factors == 1 && x != 1.0{
            prime_factors.push(x as i64);
        }
        amount_factors = 0;
        x+=1.0;
        done +=1;
        if done == number{
            break
        }
    }
    prime_factors
}

fn multiplicative_order(number:i64)->i64{
    let mut len = 1;
    let mut testnumber:BigUint = BigUint::from(10 as u64);
    while testnumber.clone() % BigUint::from(number as u64) != BigUint::from(1 as u64) && testnumber.clone()%BigUint::from(number as  u64) != BigUint::from(0 as u64){
        testnumber*=BigUint::from(10 as u64);
        len +=1;
    }
    len
}