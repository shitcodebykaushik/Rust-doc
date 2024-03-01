
#[warn(unsafe_code,unused_allocation,unused_imports)]
use stack_stack::{Stack, stack};
fn main (){
    let mut s = Stack::with_capacity::<3>();
 
    assert_eq!(s.push(6), Ok(()));
    assert_eq!(s.push(2), Ok(()));
    assert_eq!(s.push(8), Ok(()));
    assert_eq!(s, [6, 2, 8]);
     
    assert_eq!(s.push(3), Err(3));
    assert_eq!(s.push(1), Err(1));
    assert_eq!(s, [6, 2, 8]);
   
   println!("the  {:?}",s);
   
    

    
} 