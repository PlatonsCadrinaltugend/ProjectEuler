fn main() {
    let mut amount = 0;
    for h in 0..=1{
        for b in 0..=100{
            for c in 0..=40{
                for d in 0..=20{
                    for e in 0..=10{
                        for f in 0..=4{
                            for g in 0..=2{
                                'outer: for a in 0..=200{
                                    if a + 2*b + 5*c + 10*d + 20*e + 50*f + 100*g + 200*h > 200{
                                        break 'outer
                                    }
                                    if a + 2*b + 5*c + 10*d + 20*e + 50*f + 100*g + 200*h == 200{
                                        amount +=1;
                                    } 
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", amount);
}
