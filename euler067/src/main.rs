use std::time::{Instant};
use std::fs;
fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string(r"C:\Users\Domin\OneDrive\Desktop\Studiumsstuff\Programmierung\Rust\ProjectEuler\euler067\0067_triangle.txt").expect("REASON");
    let parts = contents.lines();
    let mut collection: Vec<&str> = parts.collect();
    collection.retain(|x| *x != "\r");
    collection.retain(|x| *x != "\n");
    let mut data:[Vec<i64>; 100] = core::array::from_fn(|_| Vec::new());
    let mut counter = 0;
    for elem in &collection{
        let data2 = elem.split(" ").collect::<Vec<_>>();
        for x in 0..data2.len(){
            data[counter].push(data2[x].parse::<i64>().unwrap());
        }
        for _x in data2.len()..100{
            data[counter].push(00);
        }
        counter +=1;    
    }
    

    let mut array: [Vec<i64>; 100] = core::array::from_fn(|_| Vec::new());
    for y in 0..100{
        let vecotr: Vec<i64> = Vec::new();
        array[y]=vecotr;
    }
    for z in 0..100{
        for _a in 0..100{
            array[z].push(00);
        }
        
    }
    for a in 0..100{
        for b in 0..100{
            if b+a<=99{array[a][b] = data[a+b][b];}
            // print!("{:4} ", array[a][b]);
        }
        // println!();
    }
    for y in 0..100{
        for x in 0..100{
            if x>0{
                if y>0{
                    if array[y][x-1] > array[y-1][x]{
                        array[y][x] = array[y][x-1] + array[y][x];
                    }
                    else{
                        array[y][x] = array[y-1][x] + array[y][x];
                    }
                }
                else{
                    array[y][x] = array[y][x-1] + array[y][x];
                }
            }
            else{
                if y>0{
                    array[y][x] = array[y-1][x] + array[y][x];
                }
                else{
                    array[y][x] = array[y][x];
                }
            }
            // print!("{:5} ", array[y][x]);
        }
        // println!();
    }
    println!("{}", array[99][99]);
    println!("{:?}", start.elapsed());
}