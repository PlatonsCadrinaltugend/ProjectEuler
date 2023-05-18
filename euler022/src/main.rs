use std::fs;
fn main() {
    static ASCII_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e','f', 'g', 'h', 'i', 'j','k', 'l', 'm', 'n', 'o','p', 'q', 'r', 's', 't','u', 'v', 'w', 'x', 'y','z'];
    let contents = fs::read_to_string(r"C:\Users\Domin\OneDrive\Desktop\Studiumsstuff\Programmierung\Rust\Project Euler\euler022\p022_names.txt").expect("REASON").to_lowercase();
    let parts = contents.split("\"");

    let mut collection: Vec<&str> = parts.collect();
    collection.retain(|x| *x != ",");
    collection.retain(|x| *x != " ");

    collection.retain(|x| *x != "");
    
    collection.sort();
    let mut sum_total = 0;
    for (e, x) in collection.iter().enumerate(){
        if *x == "," || *x == " "{
            continue
        }
        let mut sum = 0;
        for c in x.chars(){
            sum += (ASCII_LOWER.iter().position(|&r| r == c).unwrap() +1) as i64;
        }
        sum_total += ((e+1) as i64)*sum;
        // println!("{} {}",sum, *x);
    }
    println!("{}", sum_total);
}
