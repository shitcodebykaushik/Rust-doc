use log::debug;
use env_logger;
fn main () {
    env_logger::init();
    println!("This is  sid ");
    let  sid  = vec! [1,2,3];
    println!("This is a debug message {}",sid[10]);
}