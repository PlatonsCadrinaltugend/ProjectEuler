fn main() {
    let mut sum = 0;
    let mut abundante = Vec::new();
    for i in 1..=28123{
        if abundant(i) {
            abundante.push(i);
        }
    }
    let mut reachable: [bool; 28123] = [false; 28123];
    for a in &abundante{
        for b in &abundante{
            let ab = a+b;
            if ab>=28123{
                continue
            }
            reachable[ab as usize] = true;
        }
    }
    for x in 0..28123{
        if reachable[x] == false{
            sum +=x;
        }
    }
    println!("{}",sum);
}
fn abundant(number:i64) -> bool{
    let mut sum = 0;
    let div = divisors(number);
    for elem in div{
        sum += elem;
    }
    sum>number
}

fn divisors(number: i64)->Vec<i64>{
    let mut v = Vec::new();
    for x in 1..=(number/2){
        if number%x == 0{
            v.push(x);
        }
    }
    v
}