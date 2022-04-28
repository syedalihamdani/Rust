fn main() {
    let x=10;     //In rust by default all variable are immutable 
    println!("The value of x is {}",x); 

    let mut y=23;   //To make variable mutable.we have to write the mut key word.
    println!("The value of y is {}",y);


    y=333;

    println!("New value of y is {}",y);


    let a:i8 =23;   //Is a signed 32bit integer with i32 we can change it like this  a:i64 
    let b:u8=55;    // If you are shure that the value is not be negative you can change it unsigned integer.
    let f=9.33;     //floting type number.It is f32 type.
    let c=false; 
    println!("the value of a,b,c and f is {},{},{}{}",a,b,c,f)

}
