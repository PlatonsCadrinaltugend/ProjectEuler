use num_bigint::BigUint;
fn main(){
    let mut current = BigUint::from(1 as u64);
    let mut before = Vec::new();
    before.push(BigUint::from(1 as u64));
    before.push(BigUint::from(1 as u64));
    let mut pos = 2;
    let power = BigUint::from(10 as u64).pow(999); // digits -1 
    while current.clone()/power.clone()< BigUint::from(1 as u64){
        current = &before[pos-1] + &before[pos-2];
        before.push(current.clone());
        pos+=1;
    }
    println!("{}", pos);
}
