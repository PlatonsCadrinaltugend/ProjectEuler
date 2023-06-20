use std::fs;
fn main() {
    let contents = fs::read_to_string(r"C:\Users\Domin\ProjectEuler\Required Files\0054_poker.txt").expect("REASON");
    let parts = contents.split(" ");
    let mut collection: Vec<&str> = parts.collect();
    for (x, elem) in collection.clone().iter().enumerate(){
        collection.remove(x);
        let coll:Vec<&str> = elem.split("\n").collect();
        for lem in &coll{
            collection.push(lem);
        }
    }
    let mut won = 0;
    for x in (0..5000).step_by(10){
        if poker(vec![String::from(collection[x]),String::from(collection[x+1]),String::from(collection[x+2]),String::from(collection[x+3]),String::from(collection[x+4]),String::from(collection[x+5]),String::from(collection[x+6]),String::from(collection[x+7]),String::from(collection[x+8]),String::from(collection[x+9])]){
            won+=1;
        }
    }
    println!("{}", won);
}

fn poker(mut vec:Vec<String>)->bool{
    for mut elem in &vec{
        match elem[..1].as_bytes(){
            [b'T'] =>elem = &(String::from("10") + &elem[1..]),
            [b'J'] =>elem = &(String::from("11") + &elem[1..]),
            [b'Q'] =>elem = &(String::from("12") + &elem[1..]),
            [b'K'] =>elem = &(String::from("13") + &elem[1..]),
            [b'A'] =>elem = &(String::from("14") + &elem[1..]),
            _ => elem=elem
        } 
    }
    let vec1 = Vec::new();
    let vec2= Vec::new();
    for x in 0..=5{
        vec1.push(vec.chars().nth(x).unwrap());
        vec2.push(&vec[(x+5)..(x+6)]);
    }
    let p1 = what(vec1);
    let p2 = what(vec2);
    return true;
}

fn flush(vec:Vec<&String>)->bool{
    let var = vec[0].chars().last().unwrap();
    for elem in vec{
        if elem.chars().last().unwrap() != var{
            return false;
        }
    }
    true
}

fn what(vec1:Vec<&String>)->i8{
    let mut min = 10;
    if flush(vec1){
        if royal_flush(vec1){
            return 1;
        }
        if straight(vec1){
            return 2;
        }
        min = 5;
    }
    if straight(vec1){
        if min>6{
            min = 6;
        }
    }
    if pair(vec1){
        if triple(vec1){
            if full_house(vec1){
                return 4;
            }
            if four(vec1){
                return 3;
            }
            if min>7{
                return 7;
            }
        }
        if two_pair(vec1){
            return 8;
        }
        return 9;
    }
    return min;
    
}