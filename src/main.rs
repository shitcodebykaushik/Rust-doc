
use std::io;
fn main (){
    println!("Enter your name ");
    let mut l = String::new();
    io::stdin().read_line(&mut l ).err();   
    println!("Your name is {}",l);

}