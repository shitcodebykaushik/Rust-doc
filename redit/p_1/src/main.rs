// Print the 12 times table 

// For means in rust means iterating over collections 
fn main() {
    for o in 1..12 {
        for j in 1..12 {
            print!("{:?}",o*j);
        }
        print!("\n");
    }
}
