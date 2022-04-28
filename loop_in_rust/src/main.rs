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

    for i in 1..11{
        println!("Range of numbers is {}",i);
    }

    let r=102..120;

    for i  in r{
        println!("r range is {}",i);
    }

    let animals=vec!["tiger","lion","rabbit"];

    for a in animals.iter(){        //  when you use the vec! make sure you use iter! key word other wise ownership will transfer.
        println!("The name of the animal is {}",a);
    }

    for (index,a) in animals.iter().enumerate(){        //  when you use the vec! make sure you use iter! key word other wise ownership will transfer.
        println!("The index and name of the animal is {} {}",index,a);
    }
    
    
}
