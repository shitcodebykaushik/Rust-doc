use graph::prelude::Idx;

pub struct  Solution;  // This is public struct with the name of the solution.
   // This is method(operation which Solution can perfrom ) for the solution struct ,within this block we can define function and  methods too
impl Solution {


    // Here we have defined a public function with the name of longest_string .( s: String , this part define the parameter with the name of s and data type with the string )
pub fn longest_string (s: String) -> i32 {

    // Vec <char> -> This define that the type of a variable is vector(dynamic array) that holds the characters
       //  Vec is the genric collection which means that its growable array.
         //<char >  This means that the vec will hold the char type of the element in his container.
    // s.chars() -> This means that we have method to acces the character in the string individually through iteration.
   // .collect () -> This will collect the character into vec<char>.
   // seq is the name of the variable here that we have declared in this case.
    let seq : Vec<char> = s.chars().collect();

    // Here we are defining the new variable with the name of len.
    // usize is use to define the indexing and representing the size of collections,
    // seq.len() is a method which is use to define the  number of elements in the vector. 

    let len: usize = seq.len();
    
   // Here we have define the tuple of the variable  and it initilize with the 000
    let (mut start,mut end ,mut max) = (0,0,0);



    while end <len {
        for idx in start..end{
   
       if seq [end] ==seq[idx] {
        start = idx +1;
        break;
       }

     


        }    
    
    
    
    }


}


}





fn main () {

}