fn main() {
    let mut v = vec![1,2];
    let mut pos = 0;
    while &v[pos] + &v[pos+1] < 4_000_000{
        v.push(&v[pos] + &v[pos+1]);
        pos +=1;
    }
    let mut sum = 0;
    for elem in v {
        if elem%2 == 0{
            sum+=elem;
        }
    }
    println!("{}", sum)
}
