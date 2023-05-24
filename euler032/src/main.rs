use itertools::Itertools;

fn main() {
    let mut vec = Vec::new();
    let mut sum = 0;
    'outer: for a in 1..=1_000_000_000{
        for b in 1..=1_000_000_000{
            if a*b>10_000{
                continue 'outer
            }
            if a>=100 && b>=100{
                break 'outer
            }
            if is_pandigital(a.to_string() + &b.to_string() + &(a*b).to_string()){
                if !vec.contains(&(a*b)){
                    vec.push(a*b);
                    sum+=a*b;
                }
        }
        }
    }
    println!("{}", sum);
}
fn is_pandigital(number:String)->bool{
    if number.len()!=9{
        return false
    }
    let sorted = number.chars().sorted().collect::<String>();
    sorted == "123456789"
}