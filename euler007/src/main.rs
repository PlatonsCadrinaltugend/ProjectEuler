fn main() {
    let amount = 10_001;
    println!("{}", prime_numbers(amount).last().unwrap());
}

fn prime_numbers(number: i64) -> Vec<i64>{
    let mut prime_factors = vec![];
    let mut amount_factors = 0;
    let mut done = 0;
    let mut x = 1;
    loop{
        for y in 1..x{
            if x%y == 0{
                amount_factors +=1;
                if amount_factors>1{
                    break
                }
            }
        }
        if amount_factors == 1{
            done +=1;
            prime_factors.push(x);
        }
        amount_factors = 0;
        x+=1;
        if done == number{
            break
        }
    }
    prime_factors
}