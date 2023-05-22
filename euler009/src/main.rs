fn main() {
    'outer: for x in 1..=998{
        for y in 1..=998{
            for z in 1..=998{
                if x+y+z == 1000{
                    if x*x + y*y == z*z{
                        println!("{}", x*y*z);
                        break 'outer
                    }
                    else if x*x + z*z == y*y{
                        println!("{}", x*y*z);
                        break 'outer
                    }
                    else if z*z + y*y == x*x{
                        println!("{}", x*y*z);
                        break 'outer
                    }
                }
            }
        }
    }
}
