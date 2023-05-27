fn main() {
    let mut max_solutions = 0;
    let mut max = 0;
    for x in 1..=1000{
        let mut solutions = 0;
        if x%3 != 0{
            continue
        }
        for c in 1..=x/2{
            'outer: for b in 1..=c{
                for a in 1..=b{
                    if a*a + b*b > c*c{
                        continue 'outer
                    }
                    if a*a + b*b != c*c{
                        continue
                    }
                    if a+b+c == x{
                        solutions+=1;
                     
                    }
                }
            }
        }
        if solutions>max_solutions{
            max_solutions = solutions;
            max = x;
        }
    }
    println!("{}", max);
}
