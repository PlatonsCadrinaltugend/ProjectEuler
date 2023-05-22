fn main() {
    const MAX:usize = 1001;
    let line = vec![0;MAX];
    let mut array = vec![line.clone();MAX];
    let mut current_number = 1;
    let mut changes = 0;
    let mut a = MAX/2;
    let mut b = MAX/2;
    let mut current = 0 ;
    let mut direction = 0;
    let mut add = false;
    while a<MAX  && b<MAX{
        while changes>=current{
            array[a][b] = current_number;   
            if direction == 0{
                a+=1;
            }
            else if direction ==1{
                b+=1;
            }
            else if direction ==2{
                a-=1;
            }
            else if direction == 3{
                b-=1;
            }
            current_number+=1;
            current +=1;
        }
        current=0;
        if add{
            changes+=1;
        }
        add = !add;
        direction = (direction +1) %4
    }
    let mut sum = -1;
    for x in 0..MAX{
        sum += array[x][x];
        sum += array[x][(MAX-1)-x];
    }
    println!("sum: {}", sum);

}
