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
     io::stdin().read_line (&mut data).unwrap(); // This need to be resolved ?w
     let a :i128 =data.trim().parse().unwrap();

     data.clear();
     io::stdin().read_line (&mut data).unwrap();
     let b :i128 =data.trim().parse().unwrap();

     let c =a+b;
     println!("{}",c);
 }
```
```Rust
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut iter = input.trim().split_whitespace();
    let a: i32 = iter.next().expect("No value found").parse().expect("Failed to parse integer");
    let b: i32 = iter.next().expect("No value found").parse().expect("Failed to parse integer");

    // Your code here
    if a+b+(a*b)==111 {
        println!("Yes");
        }else {
            println!("NO")
        }
}
```

 2. Add two numbers

  # 3. Find the longest substring without repeating the character .
























