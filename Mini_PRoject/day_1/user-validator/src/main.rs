// This is project for  the  Rust_series 
// Topic a program to validate user 
use std::io::stdin;
#[allow(unused_variables)]
fn main () {
println!("# Welcome to the user validation form ");

// Asking the user to give the input  or we can say that we are processing the input 
let mut a = String::new();
stdin().read_line (&mut a).expect("Please enter the number");
 // Creating the chekcpoint for the validation of the user input  .
 let username2 =String::from("Kaushik Raj");
 let a = a.trim();
  // Storing user information in local storage  location and we have used the co .
 if a==username2 {
    println!("Give your pasword details");
 }else {
    println!("Try to remember your username");
 };
 
let useroasswd1 =String::from("Try");

 let mut b= String::new();
 stdin().read_line(&mut b).expect("Please enter the password");
 let b = b.trim();
if b ==useroasswd1 {
    println!("Your  password is ok !")
}else {
    println!("Your password is wrong ")
}
}


