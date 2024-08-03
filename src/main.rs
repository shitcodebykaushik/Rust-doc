use std::io;
fn main () {
    let mut name = String::new();
    io::stdin().read_line(&mut name ).expect("Failed to read line ");
    let mut a = 56;
    println!("THe value is {:?}",a);
    a= 45;
    println!("THe value is {:?}",a);
}