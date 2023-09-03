use num_bigint::BigInt;
//really inefficient with all those clones
fn main() {
    let mut count = 0;
    let mut nom: BigInt = BigInt::from(1);
    let mut den: BigInt = BigInt::from(1);
    for _x in 1..=1_000{
        let k = nom.clone();
        nom = 2*den.clone()+nom.clone();
        den = k+den.clone();
        if lenn(nom.clone())>lenn(den.clone()){
            count+=1;
        }
        let ggt = ggt(nom.clone(), den.clone());
        nom/=&ggt;
        den/=&ggt;
    }
    println!("{count}");
}

fn ggt(mut a:BigInt, mut b:BigInt)->BigInt{
    while b!=BigInt::from(0){
        let h = a%b.clone();
        a = b.clone();
        b= h;
    }
    a
}

fn lenn(mut num:BigInt)->i128{
    let mut count = 0;
    while num>=BigInt::from(1){
        num/=BigInt::from(10);
        count+=1;
    }
    count
}