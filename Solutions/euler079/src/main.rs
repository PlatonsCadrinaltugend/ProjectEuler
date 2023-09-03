use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string(r"C:\Users\Domin\ProjectEuler\Required Files\0079_keylog.txt").expect("REASON").to_lowercase();
    let parts = contents.split("\n");
    let collection: Vec<&str> = parts.collect();
    let mut prev = HashMap::new();
    for elem in collection{
        let mut vec = Vec::new();
        for chars in elem.chars(){
            if !prev.contains_key(&chars){
                prev.insert(chars,HashSet::new());
            }
            for chs in vec.clone(){
                prev.get_mut(&chars).map(|val| val.insert(chs));
            }
            vec.push(chars);   
        }
    }
    let mut passcode = String::from("");
    for x in 0..prev.len(){
        for elem in prev.keys(){
            if prev[elem].len() == x{
                passcode = passcode.to_owned()+ &elem.to_string();
            }
        }
    }
    println!("{}", passcode);
}
