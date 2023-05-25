fn main() {
    let mut amount = 0; //for loop doesn't iterate the 2, but the 1
    println!("{}", circular_prime(1193));
    for x in (1..1_000_000).step_by(2){
        if is_prime(x){
            if circular_prime(x){
                amount+=1;
            }
        }
    }
    println!("{}", amount);
}

fn circular_prime(number:i64)->bool{
    let current = number.clone().to_string();
    if current.len() == 1{
        return true
    }
    let permutations = permutations(current);
    for elem in permutations{
        if !is_prime(elem.parse::<i64>().unwrap()){
            return false;
        }
    }
    true

}

fn permutations(array:String)->Vec<String>{
    let mut permutations = array.clone();
    let mut vec = Vec::new();
    loop{
        permutations = String::from(&permutations[1..permutations.len()]) + &String::from(&permutations[0..1]);
        vec.push(permutations.clone());
        if permutations == array{
            break
        }
    }

    vec
}
fn is_prime(number:i64) -> bool{
    if number<0{
        return false;
    }
    for x in 2..=((number as f64).sqrt() as i64){
        if number%x == 0{
            return false;
        }
    }
    true
}