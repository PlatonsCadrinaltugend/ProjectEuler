fn main() {
    let amount = 100;
    let difference = sumsquare(amount) - squaresums(amount);
    println!("{}", difference);
}

fn squaresums(number: i64) -> i64{
    let mut sum = 0;
    for x in 1..=number{
        sum+= x*x;
    }
    sum
}

fn sumsquare(number:i64) -> i64{
    let mut sum = 0;
    for x in 1..=number{
        sum +=x
    }
    sum*sum
}