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
```Rust
fn main ()  {
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
 ```   
- Stack and the heap ant the pointer 
- Stack need th exact size
- Heap can have any size .
- A pointer is the addered of the location .
- Pointer is know as the reference and means that it point to the memory of another value . A reference means that you borrow the value ,but you dont own it .
- const is for values that don't change, the name is replaced with the value when it's used,
- static is similar to const, but has a fixed memory location and can act as a global variable.
- Refernce can be mutable to 
- Use * to change the mutable  value. This is known as the dereferencing .


# Ownership
Rust is system programming language .
Data with the size unknown at the compile time is stored at the heap .
Managing the heap data is why ownership exit .
- Rules of the owenership
  -  Each values in Rust has a variable  that is called owner .
  - There can be only one owner at a time .
  - Once the varibale go out of the scope then the value dropped .
  - String  can be muted but string literal cant  be muted . The reason is how these two deals with the memory .
  - When we know the contents at the compile time then it is hardcoded into the final executable .
  - When we dont  know the contents at the compile time then the we need to allocate the memory on the heap .
  - When the copy of the string goes out of the scope.then they both try to free the same space , this is known as the double free and it creates the bugs ,which can lead to the security and vulnerabilities .
  - Rust never create the deep copy by own .
  - For integer we have he size at the compile time only .
  - For the sting we doent have the size at the compile time ,we get it durinng the runtime .
```Rust
fn take_kasuhik (string : String) {
  println!("{:?}",string);
}
fn moves_k (value : i32) {
 println!("{:?}",value);
}

fn main () {
    let   s = String::from("Kaushik raj") ;
   
    take_kasuhik (s);

    let c= 13;
    moves_k(c);
   

}
``` 
# Reference 
```Rust

fn calculate (s :&String) -> usize {
    s.len()
}
// We took the &string without taking the string .& will allow  the referencing without taking the ownership . 
fn main () {
 let  s1 = String::from ("Kaushik");
    
 let  x = calculate(&s1);
 // Reference is the type of the data type that doesn't have the ownership .
 // Here we passed the value of the &s1 to the calculate fucntion .
 // The opposite of referencing is deferencing * . 
 // Note We are not allowed to mut the reference value . Mut can be done onlu once in the scope an for that we have to mut the value frst .
 // Rust gurantte that the reference will not be the dangling reference .

 println!("The value is {:?} The calculation is {:?}",s1 ,x);

}



```

```RUST

// Programm to calculate the number of the element it the sentence till it find the space .

fn first_word (s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return  i;
        }
        
    }
    s.len()
}


fn main () {
 let mut s = String::from("Hellworld");
 let word = first_word(&s);
 s.clear();

 println!("The value is {:?}",word);


 let mut k = String::from ("Raju ");
 let work = first_word(&k);
 s.clear();
 println!("The value is {:?}",work);
}

```




