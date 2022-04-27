fn main() {
    let mut n=0;
    loop{
        n +=1;
        // To stop this infinate loop use break
        if n>11{
            break;
        }
        if n==6{
            continue;
        }
        println!("{}",n);
    }

    while n<17{
        println!("n is smaller than 5");
        n+=1;
    }
    
}
