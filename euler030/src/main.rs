fn main() {
    let mut sum = 0;
    for x in 10..=354_294{
        if is_fifth(x){
            sum+=x;
        }
    }
    println!("{}", sum);
}
fn is_fifth(number:u64)->bool{
    let mut current = number.clone();
    let mut sum = 0;
    while current>=1{
        sum+=(current%10).pow(5);
        current/=10;
    }
    return sum ==number
}
