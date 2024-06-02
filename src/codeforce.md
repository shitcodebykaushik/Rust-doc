```Rust

use std::io::BufRead;
fn main () {
    let stdin = std::io::stdin();
    let mut line =stdin.lock().lines();

    let input = line.next().unwrap().unwrap();
    let mut input = input.split_ascii_whitespace();

    let (n ,m, a) :(usize , usize,usize) = (

     input.next().unwrap().parse().unwrap(),
     input.next().unwrap().parse().unwrap(),
     input.next().unwrap().parse().unwrap(),




    );

   let x  = (n as f64/ a  as f64).ceil() as usize;
let y = ( m as f64/ a as f64).ceil() as usize;
println!("{}",x*y);

}




```