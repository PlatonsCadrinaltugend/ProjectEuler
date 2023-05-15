fn main() {
    prime_numbers(200);
}

fn prime_numbers(number: i32) -> Vec<i32>{
    let number2 = number.clone()/2;
    let mut prime_factors = Vec::new();
    let mut amount_factors = 0;
    for x in 1..number{
        for y in 1..x{
            if x%y == 0{
                amount_factors +=1;
            }
        }
        if amount_factors == 1{
            println!("{}", x);
            prime_factors.push(x);
        }
        amount_factors = 0;
    }
    prime_factors
}