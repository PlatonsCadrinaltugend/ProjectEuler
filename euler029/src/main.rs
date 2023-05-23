use num_bigint::BigUint;
fn main() {
    let mut vector = Vec::new();
    for a in 2..=100{
        for b in 2..=100{
            let res= (BigUint::from(a as u64)).pow(b as u32);
            if !vector.contains(&res){
                vector.push(res);
            }
        }
    }
    println!("{}", vector.len());
}
