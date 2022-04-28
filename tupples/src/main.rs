fn main() {
    let tup1=(12,3.234,true,"Computer",(1,2,3));
    println!("{}",tup1.0);
    println!("{}",tup1.1);
    println!("{}",tup1.2);
    println!("{}",tup1.3);
    println!("{}",(tup1.4).2);


    let (a,b,c,d,f)=tup1;
    println!("{}",a);
    println!("{}",b);
    println!("{}",c);
    println!("{}",d);
    println!("{}",f.0);

}
