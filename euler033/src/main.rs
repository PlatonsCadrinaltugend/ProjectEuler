use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut vec = Vec::new();
    let mut vec2 = Vec::new();
    for a in 11..100{
        for b in 11..a{
            if a%10 != 0 && b%10 != 0 && a%11 != 0 && b%11 != 0{
                let mut a_array = divide_to_string(a);
                let mut b_array = divide_to_string(b);
                for elem in a_array.clone(){
                    if b_array.contains(&elem){
                        let index = a_array.iter().position(|x| *x == elem).unwrap();
                        a_array.remove(index);
                        let index2 = b_array.iter().position(|x| *x == elem).unwrap();
                        b_array.remove(index2);
                        if to_new_string(b_array.clone())/to_new_string(a_array.clone()) == b as f64/a as f64{
                            vec.push(a);
                            vec2.push(b);
                        }
                        break
                    }
                }
            }
        }
    }
    let mut productdenominator = 1;
    let mut productnumerator = 1;
    for elem in &vec{
        productdenominator*=elem;
    }
    for elem in &vec2{
        productnumerator*=*elem;
    }    
    println!("{}", productdenominator/gcd(productdenominator,productnumerator));
    println!("{:?}", start.elapsed());
}

fn divide_to_string(n:i64)->Vec<i64>{
    let mut s = n.to_string();
    let mut array = Vec::new();
    while s.len()>=1{
        array.push(s[0..1].parse::<i64>().unwrap());
        s = s[1..].to_string();
    }
    array
}

fn to_new_string(array:Vec<i64>)->f64{
    let mut s = String::from("");
    for elem in array{
        s+= &elem.to_string();
    }
    s.parse::<f64>().unwrap()
}

fn gcd(a:i64, b:i64)->i64{
    let mut b = b;
    let mut a  = a;
    if a == 0{
        return b.abs()
    }
    if b == 0{
        return a.abs()
    }
    while b!=0{
        let h = a%b;
        a = b;
        b = h;
    }
    return a.abs()
}