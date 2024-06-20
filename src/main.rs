fn main (){
    let bi : u8 = 45; // This is in stack 
    let hi :Box<u8>= Box::new(bi); // This is in heap 
    println!("THe value1 is {:?}",hi);
    println!("THe value2 is {:?}",bi);
   
   let hi:Box<u8>=Box::new(45);
   let hi: u8 = *hi;
  
   println!("THe value3 is {:?}",hi);
  
  
  
  }