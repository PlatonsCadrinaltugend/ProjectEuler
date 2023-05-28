use std::time::{Instant};
fn main() {
    let start = Instant::now();
    let mut array : [Vec<i64>; 21] = core::array::from_fn(|_| Vec::new());
    for y in 0..=20{
        let vecotr: Vec<i64> = Vec::new();
        array[y]=vecotr;
    }
    for z in 0..=20{
        for _a in 0..=20{
            array[z].push(0);
        }
        
    }
    for x in 0..=20{
        for y in 0..=20{
            if x>0{
                if y>0{
                    array[x][y] = array[x-1][y] + array[x][y-1];
                }
                else{
                    array[x][y] = 1;
                }
            }
            else{
                array[x][y] = 1;
            }
        }
    }
    println!("{}", array[20][20]);
    println!("{:?}", start.elapsed());
}
