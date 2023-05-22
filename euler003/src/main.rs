fn main() {
    let number: f64 = 600851475143.0;
    let mut primes: Vec<i64> = prime_numbers(number.clone());
    primes.reverse();
    let number2: i64 = number as i64;
    for elem in primes {
        if number2 %elem == 0{
            print!("{}", elem);
            break
        }
    }
}

fn prime_numbers(number: f64) -> Vec<i64>{
    let number2 = number.clone().sqrt() as i64;
    let mut prime_factors = vec![];
    let mut amount_factors = 0;
    for x in 1..number2{
        for y in 1..x{
            if x%y == 0{
                amount_factors +=1;
                if amount_factors>1{
                    break
                }
            }
        }
        if amount_factors == 1{
            prime_factors.push(x);
        }
        amount_factors = 0;
    }
    prime_factors
}