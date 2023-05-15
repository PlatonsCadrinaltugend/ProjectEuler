fn main() {
    let primes = prime_numbers(10);
    let mut sum = 0;
    for elem in primes{
        sum+=elem;
    }
    println!("{}", sum);
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