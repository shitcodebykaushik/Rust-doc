#  TYPES
Signed means positive and negative both .
Unsigned means only positive only .
Usize and isize means number of bits in  your your type of the computer
Cast u8 as char" means "pretend u8 is a char .
We can't cast i32 as a char, but we can cast an i32 as a u8. And then we can do the same from u8 to char .
 Learnt the conversion of one data type into another one .
 
 `Rust
 use std::char;
fn main () {
    let first = "A";
    let seconf = "" ;
    let third  = 909999;
    println!("The value is {}",third as u8 as char);
} 
 `
When using characters as part of a string, the string is encoded to use the least amount of memory needed for each character.
`Rust 
fn main () {
    println!("The sizE is  the : {}",std::mem::size_of::<i128>());
}
`
Some println! need to debug .
 Display means printing with {}, and Debug means printing with {:?} .
   println!("This will print smallest the i8 {} and the biggest will be {}.",i64::MIN , i64::MAX);

- Mutability and shadow is totally different ;
- When you shadow a variable, you don't destroy it. You block it.
- In shadow  we can use the same  name different time .
- fn main ()  {
    let doesn = ();
    println!("This will not print {:#?}",doesn);
    println!("This will print smallest the i8 {} and the biggest will be {}.",i64::MIN , i64::MAX);
    let mut set = 122;
     set = 233;
     println!("The value is mutable here {} ",set);
    fn times_two (number : i32) -> i32{
        number *2

    }
    let final_number = {
        let y = 10 ;
        let x: i32 = 14;
        let x = times_two(x);
        let x = x+y;
        x
    };
    println!("The data is {:?}",final_number);
        
    }
- Stack and the heap ant the pointer 
- Stack need th exact size
- Heap can have any size .
- Pointer is know as the reference and means that it point to the memory of another value . A reference means that you borrow the value ,but you dont own it .
- const is for values that don't change, the name is replaced with the value when it's used,
- static is similar to const, but has a fixed memory location and can act as a global variable.
- Refernce can be mutable to 
- Use * to change the mutable  value. This is known as the dereferencing .
    - 
