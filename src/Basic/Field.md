# This is rust basic practice section .


# Shadow Set

```Rust
fn main () {
    let v:i32= 45;
    println!("The value is  {:?}",v);
    let v:i64 =3456;
    println!("The next value is {:?}",v);
    
    let c= String::from("this is bou ");
    println!("The string value is {:?}",c);

   let r:f32 =34.4;
   println!("The value is  {:?}",r);
}
```
# Heap 
```Rust
// Heap is the dynamic allocation of the memory . It ask for memory during the runtime  .
fn main () { 
    // Here we did the variable shadowing and applied the cocncept of the stack and heap .
let x:i32= 5;  // This is stack the allocation  and dellocation is easy ,predicatable automatically handled by the system .
println!("{:?}",x); 
let x= Box::new(6); //Rust uses the Smart pointer which uses the allocates the heap and store the vaiable value into it. The actual value of the box is structure whicc is pointer to the heap .
println!("{:?}",x);
let f = Box::new(45);
println!("The heap allocation is here {:?}",f);

let c: Box<i32> = Box::new(365);
println!( "The value is {:?}",c);
let d = *c; // Accessing the value through the pointer .
println!( "The value is {:?}",d);
}
```
