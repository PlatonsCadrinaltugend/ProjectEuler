fn main() {
    'outer: for x in (1..998_001).rev(){
        if palindrome(x){
            for y in(1..999).rev(){
                if x%y == 0{
                    let z = x/y;
                    if z<1000{
                        println!("{}",x);
                        println!("{}", y);
                        break 'outer
                    }

                }
                if y*y >998_001{
                    break
                }
            }
        }
    }
}

fn palindrome(number: i32) -> bool{
    let number3 = reversee(number.clone());
    number3 == number
}

fn reversee(num:i32)->i32{
    let string_number = num.to_string();
    let reverse = string_number.chars().rev().collect::<String>();
    reverse.parse::<i32>().unwrap()
}