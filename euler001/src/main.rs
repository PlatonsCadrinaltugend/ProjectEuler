fn main(){
    let amount = 1000;
    let mut sum = 0;
    for i in 1..amount {
        if i%15==0{
            sum+=i;
        }
        else if i%3==0{
            sum+=i;
        }
        else if i%5==0{
            sum+=i;
        }
    }
    println!("{}",sum)
}
