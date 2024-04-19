# Here is the list of question for the codechef question 
1. Number Mirror
Write a program that takes a number 
𝑁
N as the input, and prints it to the output.

Input Format
The only line of input contains a single integer.

Output Format
Output the answer in a single line.
```Rust
use std::io;
fn main (){
    let  mut value = String::new();
    io::stdin().read_line( &mut value).expect("Failed to read");
    println!( "The value is {}",value);
}
```
# 2
```Rust
use std::io;
 fn main (){
    let mut data = String::new();
     io::stdin().read_line (&mut data).unwrap(); // This need to be resolved ?
     let a :i128 =data.trim().parse().unwrap();

     data.clear();
     io::stdin().read_line (&mut data).unwrap();
     let b :i128 =data.trim().parse().unwrap();

     let c =a+b;
     println!("{}",c);
 }
```