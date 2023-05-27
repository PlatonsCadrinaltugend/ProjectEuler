fn main() {
    let mut sum = 0;
    for x in 1..=1_000_000{
        if palindrome(x){
            if binary_palindrome(x){
                sum+=x;
            }
        }
    }
    println!("{}", sum);
}

fn binary_palindrome(n:i64)->bool{
    let s = format!("{:b}", n);
    s == s.chars().rev().collect::<String>()
}

fn palindrome(n:i64)->bool{
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}