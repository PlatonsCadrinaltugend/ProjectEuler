fn main() {
    let mut number = 20;
    let amount = 20;
    let mut bool1 = false;
    loop{
        for x in 1..amount{
            if number%x!=0{
                number +=1;
                bool1 = true;
                break
            }
        }
        if bool1 == false{
            println!("{}", number);
            break
        }
        bool1 = false;
    }
}
