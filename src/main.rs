fn main ( ) {
let sus:i32 = 34; // This data is in the stack let connvert this into the heap 
println! ("{:?}",sus);
let sus: Box<i32>= Box::new(34); // This is in the heap location where it store the data into the heap location and box is smart pinter here .
println!("{:?}",sus);
}