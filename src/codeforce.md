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
