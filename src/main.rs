use std::io;
fn main () {
    let mut name = String::new();
    io::stdin().read_line(&mut name ).expect("Failed to read line ");
    let mut a = 56;
    println!("THe value is {:?}",a);
    a= 45;
    println!("THe value is {:?}",a);
    println!("Kaushik be ready for the things");
    println! ("This is the hackthon things");
    println!("On this day UI for the hackthon was done ");
}