fn main() {
    let mut sum = 0;
    for x in 1..=1000{
        sum+=word(x).len();
    }
    println!("{}", sum);
}
const dict0_19: [&str; 20] = ["zero", "one", "two","three","four","five","six","seven","eight","nine",  "ten",  "eleven",  "twelve",  "thirteen",  "fourteen",  "fifteen",  "sixteen",  "seventeen",  "eighteen",  "nineteen"];
const dict20_90: [&str; 10] = [ "",  "",  "twenty",  "thirty",  "forty",  "fifty",  "sixty",  "seventy",  "eighty",  "ninety"];
fn word(number:u64)->String{
    if number == 1000{
        return String::from("onethousand")
    }
    if number>=100 && number%100 == 0{
        return String::from(dict0_19[(number/100) as usize].clone()) + &String::from("hundred")
    }
    if number>100{
        let mut returne: String = String::from("");
        returne = returne +  &String::from(dict0_19[(number/100) as usize].clone());
        if number%100>0{
            returne += &String::from("hundredand");
        }
        if number%100>19 && number%100 != 0
       { returne = returne + &String::from(dict20_90[((number%100)/10) as usize].clone())+ &(if number%10!=0{String::from(dict0_19[(number%10) as usize].clone())}else{String::from("")});}
       else if number%100 != 0{
        returne = returne + &String::from(dict0_19[(number%100) as usize].clone());
       }
       return returne
    }
    if number<20{
        return String::from(dict0_19[number as usize].clone())
    }
    if number>=20 && number%10 == 0{
        return String::from(dict20_90[(number/10) as usize].clone())
    }
    if number>20{
        return String::from(dict20_90[(number/10) as usize].clone()) + &String::from(dict0_19[(number%10) as usize].clone())
    }
    String::from(dict20_90[0].clone())
}
