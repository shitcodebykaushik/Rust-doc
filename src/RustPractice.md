# variable 
```rust
fn main () {
    let x: i32 = 5;
    let y: i32;
    assert_eq!(x,5);
    println!("Success");
}
```
```Rust

fn main () {
    let x: i32 = 10;
    let y: i32 = 5;
    println!("The value of the x is {:?} and the value of the y is {:?} ",x,y);
    println!("The value of the x is {:?} and the value of the y is {:?} ",x,y);
  
}



```
```Rust
//Shadowing of the varibale 

fn main () {
    let x: i32= 5;
    {
        let x :i16 = 12;
        assert_eq!(x ,12);
    }
     assert_eq!(x, 5);
     let x= 42;
     println!("{}",x);
}




```