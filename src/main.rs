
use stack_stack::{Stack, stack};
fn main (){
    let mut d1: Stack<i64, 20> =Stack::with_capacity::<20>();
   d1.push(5);

   
    
    println!("{:?}",d1);
}