# RUST is statically typed language
 - Rustup show this will show the current version detaiking of the rust ,
 - Method means the function associated with it.
-  Cargo 
 - Cargo run compile your code and result in binary 
 - Cargo test test your code and result in test  binary .
 - Toolchain is use to automate the process of building,testing,and deploying software.
 # Extern 
Link to or import external code.
 # Crate 
A Rust binary or library.
The primary use of the crate keyword is as a part of extern crate declarations, which are used to specify a dependency on a crate external to the one it's declared in. Crates are the fundamental compilation unit of Rust code, and can be seen as libraries or projects
 # Macros 
 Fundamentally  macros are a way of writing code that writes oher code .which is known as metaprogramming. 
# Shadow 
Rust allow the shadow of varible which simply means we can use same name of varible twice.
```Rust
let v :i64 =456;n
println!("The value is {}",v);

let v :i64 =456789;
println!("The value is {}",v);
 Here we are doing the varible shadowing which allow us to use same name for the multiple value 

```
# Type 
 A type define what type of data a varible can hold and the opeartion that can be performed on it .It essentially acts as blueprint for the data ,specifying its sizw,behaviour  and allowed values . 
 There are different types of types of types are there 
  - Primitive type which means it is fundamental types provided by the language it self .
     [Boolean,Integer,Floats,char,str,!never]
  - Compound types 
    These are the user defined data types .
     [Array,Slices,Enums,Structs,Union,References]

# Heap 
Is a region in computer that is used for dynamic  type of memory location  .In stack we have stactic memeory allocation where we  do memory allocation and deallocation .Which follow the concepet of Fifo. Dynamic memory refers to memory that is allocated at runtime, typically on the heap, rather than being statically allocated at compile time on the stack. Dynamic memory allocation allows programs to request memory as needed during execution, enabling flexibility in managing memory usage.
- Heap doesn't grow from the other end line atack .
- Heap is implemented at the cost of the complexity .
In contrast to static memory allocation, where the size and lifetime of memory are determined at compile time, dynamic memory allocation enables the creation of data structures whose size or lifetime cannot be known in advance or may vary during program execution. 
# Pointer 
In rust pointer must always points to the some valid location there are no null references,Rust has optional pointer ,like option owned box, `Option<Box<T>>` `

# <Box<T>>
T is a type parameter representing the type of data stored in the reference.
It is a type of pointer that uniquely owns a heap if allocation of type T.Boxes provide the ownership of the allocation .It drop tehir content when they go out of the scope .
In Rust, isize::MAX represents the maximum value that can be stored in the isize data type.

isize is a signed integer type whose size is dependent on the underlying architecture of the system. On a 64-bit architecture, isize is a 64-bit integer, and on a 32-bit architecture, it is a 32-bit integer.
```Rust
fn main (){
  let bi : u8 = 45; // This is in stack 
  let hi :Box<u8>= Box::new(bi); // This is in heap 
  println!("THe value1 is {:?}",hi);
  println!("THe value2 is {:?}",bi);
 
 let hi:Box<u8>=Box::new(45);
 let hi: u8 = *hi;

 println!("THe value3 is {:?}",hi);

}
```
`isize::MAX` therefore represents the maximum value that can be stored in the isize type on the current system architecture. It's often used when you need a variable that can hold any integer value within the range of the isize type.

# Naming Object 
 The concept of `value`,`object` and `variable`

 - The word “value” indicates an abstract, mathematical concept. For example,
when you say “the value 12” you mean the mathematical concept of the number 12. In
mathematics, there is just one number 12 in the world. Even true or "Hello" are values
that conceptually exist in one single instance in the universe, because they are concepts.
But values may be stored in the memory of a computer. You can store the number
12 or the string “Hello” in several locations of the memory. So, you can have two distinct
memory locations that both contain the 12 value.

- The portion of memory that contains a value is named “object”. Two distinct objects,
located in different positions of memory, are said to be “equal” if they contain the same
value. Instead, two values are said to be “equal” if and only if they are not distinct, that is,
they are actually the same value.
- Name od the object is called `identifiar`.
- Name of the   `Identifier-object` is called `variable`
- The operation of reserving a memory area for an object is named “allocation” of
that object.
- The removal of an object, causing its memory area to become
available for allocating other objects, is named “deallocation” of that object.

- In mutablity it doesnt allocate the object ,it just modifies the value of an already allocated object.
- The first assignment value to the varible is initilazation of the variable.
# Panic 
In rust it simply means that we are going to get the error during the runtime.
Runtime error are the more complex to solve .
# Overhead 
In computing, "overhead" refers to the additional resources, time, or processing required by a system or operation beyond what is strictly necessary to perform the desired task. Overhead can manifest in various forms, including:
# Unsafe 
"unsafe code" refers to code that is allowed to perform certain operations that are considered unsafe by the Rust language's safety guarantees. This includes:
# Primitve 
 It stand for the fundamental or basic components or operations provided by the programming language .
# Method 
It is function defination that is associted with with particular struct type of any traits .In rust it encapsulate the data that is associated with its specifif behaviour.  
# Small optimazations 
Means small/mionor  changes in the program made to code that result in the improvement of the performance .
# Push Insert 
The main difference between push and insert is that. Push put the elements at the end of the vector on the other side insert puts the element at the specified place by shifitng the element.

Dereferencing raw pointers.
Calling unsafe functions or methods.
Accessing or modifying mutable static variables.
Implementing unsafe traits.
Accessing or modifying mutable data without synchronization.
Invoking undefined behavior.

# TUPLE
A tuple is a collection of values of different types.
Tuple are stored in contagenious memory location.
Tuple are immutable by default.
Tuples are simple data structures, offering efficient memory usage for fixed-size collections of different types.
Their immutability ensures memory safety and simplifies reasoning about data access
```Rust 
fn main(){
    let i=('l','k','f',1,8.2);
    println!(" THE value at the index 5 is {}",i.4);
}
```  

# Control flow
It is the flow of program in which it check the condition and give the result on the basis of that .
In rust we can use ```if```it allow us to branch the code depending upon the condition .
```Rust
 fn main(){
    let x=4;
    ifx ==4{
        println!("The value is correct ");
    }
 }
```
We can use ```else``` with ```if ```
```Rust
fn main(){
    let x=48;
    if x<48{
        println!("x is less then 48");
}else {
    println!("The vslue is more then the require one");
}
}
```
# Loop
In Rust we use Loop keyword to repeat the itteration again and again 
We have to explictily stop the loop by using the  ```Ctrl+c```
```Rust
fn main(){
    loop{
        println!("The loop is looop")
    }
}
To stop this loop we use ctrl+c keyword
```

# RUST Ownership and Borrowing 
Ownership is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.
## Rules of ownership
 [1] There is only owner at a time. 
 [2] Each value in rust has owner.
 [3]  When the owner goes out of the scope, the value will be dropped.
 # Here is the simple demostrtion
 ```Rust
 fn main(){
 {let k: i32=5; //In this we have defined scope under {} and this is type of varible scope
 println!("s= {}",k);
 }
 println!("s= {}",k); //When we will call this it will give error in program As it is out of scope 
 
}

```
# Returning value from loops


Here is reason why string can be mutaed but string litrals can't.
As we know that string litral content is known at the compile time and its text is hardcoded directly into the final executable .
In case of string we need to allocate an amount of memory on the heap, unknown at the compile time .
# Memoery allocation in rust 
Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.Then rust called ```drop``` it is function which is automactically called when there is closing curly brakets. 

# Variables and data Interacting 
# Move
Move it is concept in which rust validate the first varible instead of being called a shallow copy it is known as move.
```Rust 
fn main(){
let s1 =String::from("Kaushik rah ");
let s2=s1;
print!("This  is {}",s1);
}
//This code wont work as the value of s1 as been moved to s2 
fn main(){
let s1 =String::from("Kaushik raj ");
let s2=s1;

print!("This  is {}",s2);
}
//On the other this code will run as we call the varible s2 beacuse the ownership of s1 has been passed to s2.
```
# Clone 
 This is concept in rust where it clone the data of data of one variable to another .
 ```Rust
 fn main(){
    let c=String::from("Hey kaushik raj ");
    let d=c.clone();
    println!("This is the value of x = {},This is value of y={}"c ,d);
 }
 
 ```



 This line of code will give error as it we have complex data of string and we cant assign both of them.
 ```Rust
 fn main(){
let x=String::from ("HI");
let y=x;
print!("y is {}",x);
}
```
But on the other side this line of code wont give you error.
 ```RUST
 fn main(){
let x= "HI";
let y=x;
print!("y is {}",x);
}
 ```

 Ownership and Borrowing are why Rust is Rust .
 Owner ship simpliy means owning some amount if memory .
 Initially frustrates both the new and senior developers
 Memory Management 
 No garbage collection 
 Developer manages memoery in code 
 Stacks VS Heap
 Benefits 
 Runtime speed
 Parallel and Concurrent processing 
 Safety 
 In rust memory time is known at the compile time.
 In rust every piece of data in the memory has an memory and there can be only a single owner of memory at a time.
 String is just a collection of u8.
 # Stack
 It store the in the order it gets them and remove the values in the opposite order .It work in the concept of LIFO
 Stack in rust is having known,fixed size.
 Addig data is known is push.
 And removing of data is pop.
 # Heap
 It is less organized,In this when we some amount of data it request for the certain space in  memory .Memory allocator find the space and return a pointer which is adrres of the location.

# Slice 
It is similiar to array but its size is not known at the compile time .
Its two word objet, the first is known as pointer of the data ,the secod is known as the length ogf the slice .

 ```Rust
 #[allow(unused_variables)] //This is so i dont get warning throughout the coding .
 `#[allow(dead_code)]` //is an attribute that disables the `dead_code` lint.
 ```
# Reference and borrowing 
A reference is way to acces or refer to a value stored in memory.It is used to pass the data between program.
Dereference is used to access the value of that a reference point is .
```Rust
fn main(){
    let m =45;
    let d=&m;
    println!("{}",d);
    let d1=d;
     println!("{}",m);

}
```
A reference is like a pointer it that its address we can follow to acces the data stored at that address, and that data is owned by some other varible.It guranteed that it will point to that particular data. It should be under scope of the varible. The reference is most common pointer .
It boorow the value of which it has benn pointed .
```Rust
fn main(){
  { let s=5;
   println!("The vsl is ={}",s);};
   let j= &s;
   println!("The vsl is ={}",j);

    }
    //This will generate the error as it going varible is out of the scope 
    fn main(){
     let s=5;
   println!("The vsl is ={}",s);;
   let j= &s;
   println!("The vsl is ={}",j);

    }
    //This will not generate the error it  will properly work on the cocept of refence and borrowing 
``` 
In refernce it can just borrow the data .
# Dereference 
This is use to borrow the data from the pointer/ refernce which haas been borrowed from the original one.
```Rust 
fn main(){
    let c=45;
    let d=&c;
    let x=*d;
    println!("The value of x is {}",x);
}

```
# Pointer
In rust we have smart pointer which is like data structure and work similiarly like pointer .
There are many smart pointer in rust std lib and all work is to allow you to have multiple owners by keepinf the tracks of the number of owner and when no owner remain,cleaing up the data .
Apart from borrowing ,the rust smart pointer have capabilities to own the data .
Smart pointer are usually implemented with with the struct .
It usually do  implementation on the ```DREF ``` AND ```Drop```
# Using BOX
A ```BOX``` is a pointer type that uniquely owns a heap allocation of type T.
Box have unknown size .
```New``` is keyword that Allocates memory on the heap and then places x into it.
```Rust
fn main(){
    let x =Box::new (5);
    println!("The value of x is {}",x);
}
```
# Function
 It is declared using the fn keyword.
 Unlike other language,here function defination have no restriction on the function order.
 n Rust, ``` && ```is the logical ```AND operator```. It's used to combine two boolean expressions and returns true only if both expressions are true.
 ```RUST
 #[allow(unused_variables )]
#[allow(unused_allocation)]
#[allow(dead_code)]  //This is rust lnt 
fn  add (a:i32, b:i32){
    a+b;
    println!("the value of add is {}",a+b)

} fn main ()
{
    add(45, 25);
    add(4578,56423)
}
```
Here I  have explain few example for the the function.
```Rust
#[allow(dead_code)]
fn main(){
sun(456, 552);
square(5);
maxium(50, 5, 23)
}
fn sun(a:i128,b:i128){
   let c= a-b;
    println!("The value is {} ",c)

}
 fn square (x:i8){
    let p= x*x;
    println!("The square  of x is {} ",p)

 }
 
    fn maxium (h:i8,k:i8,p:i8){
     if h>p &&p>k{
      println!("h is greter ");
     } 
        
     }
```
# Generic
 Generics refer to a feature that allows you to write code that can operate on different data types while maintaining type safety. Generics enable you to create reusable components (such as functions, structs, or enums) that can work with any data type.
There is no runtime cost in rust. Which means that we 
We can use as many as generic we want 
We just need to define it on Placeholder < Anything in case try upper case >.
In simple terms we can say that we are defing the multiple data types in one go from int to bool and so on.
In therory Generics are called ‘parametric polymorphism’ in type theory, which means that they are types or functions that have multiple forms (‘poly’ is multiple, ‘morph’ is form) over a given parameter (‘parametric’).
Note it is always advisble not to use more genric always.

```rust
struct Point<A,B>{
    x:A,
    y:B,
}

fn main() {
    let a=Point{x:78,y:85.47};
    let v=Point{x:-7,y:956};
     println!("x ={} y={}",a.x,a.y);
     println!("x ={} y={}",v.x,v.y);
     
    
}

```
# Traits
Definig shared behaviour(means method) using traits.
Traits allow us define set of method that are shared across different types 

This is for the concurency control.
The main building block for createing the concurent in rust is a "thread".
There are two main two types of thread in programing in thread .
First type is OS thread.It is offered by OS iteslef.
Second types is Green thread.Its an abstraction thats sit ontop of the operating system thread.
Rust uses the opeerating system thread directly and the reason why is for the the sake of hai=ving a lowet runtike by the lower runtime means the lower amount of code included in each binay after its compiled .
While creating a thread we need to bring the thread namespace first .
Rust doesen't make the gurantee  of the thread order .
# MOVE 
Move(KEYWORD) allow the clouser to move data from one thread to another THREAD and we take the ownership of the main thread.
Move key word force the clousre to refernce data by the value rather than by the reference 
# SECOND ABSTRACTION
Rust uses second abstraction which is called channels.
Channels are use to pass the messsage around and channel is made up of a transmitter which is TX ans a reciver which is RX .
Transmitter is the part that sits upstream where we actually push the message in and then the reciver is where msg come out.
We destruct the tuple that is returned by the channel method and the TX HAS SEND method that takes the value we want to send and return  a result and thats why we use unwrap .
 # ARRAY 
 In array we have some built in functionality with the rust 
 In rust array list is preety much vector.
 Its is knowns at the complie time and its slice is not  known at the compile time.
 Array list in not linked list 
 Array list is wrapper that uses array under the hood.
 Keep track of length,index of head/tail.
 push/pop/acces has 0(1).
 Constructor specifies initial size.
 # Struct 
 This let you to group the multiple data type into the similiar group .
 

 # ENUMS 
 Enums are great alternative for the pre-processor constant .
 First we define the value then it automatically take the value from there. 
 Enumerate means om general to count or list items systematically ,in programming it refers to the method or function.
 
 Pre-Processor constant ->

  


# Macro 
```assert_eq!```
This is macro which is used to match the value of varible .
```Rust
fn main(){
    let x=5;
    assert_eq!(x,5);
    println!("The value is matched ");
}
```

  
# Fearless concurrency 
This need to be done after completing the rust by example.
# The Rust Standard Library 
 The Rust Standard Library, often referred to simply as "std," is a collection of essential APIs (Application Programming Interfaces) and utilities provided by the Rust programming language. It's included with every Rust distribution and forms a core part of the language ecosystem.
   # Array 
   A fixed-size array,denoted[ELEMENT-type;non negative compile-time conatant size].
   Before sstarting this lets get into ```Derive code```
   In this we have used clone and copy to use the borrowed varable .
   ```Copy```
   It behaviour is  not overloadable.It always do a simple bit wise copy.It do implict copying means that it automatically copy the value instead of moving the data.

   This trait is not object safe.
 ```Rust
 fn main(){
#[derive(Copy,Clone,Debug)]
 struct Apple;
 let c=Apple;
 let d=c;
 print!(" {c:?}",);
 print!(" {d:?}");
 
     }
 ```
 # std::vec::Vec
  A contiguous growable  array type,written as Vec<T>,in short it is known as vector .
 `New` Vec::new() It is keyword which construct an empty vector and it will not allocate  until a new elemets are pushed inti it.
 `Push` Appends an element to the back of a collection.

 `POP` Removes the last element from a vector and returns it, or None if it is empty.
 `Len`Returns the number of elements in the vector, also referred to as its 'length.

  ```RUST
  // A simple demonstration vector expalaing how we can use it 
  fn main(){
    let  numbers :Vec<i32> = vec![1,2,3,4,5,6,7,8,9];
    println!("{:?}",numbers);
}
 // A vector which show the concept of the ownership 
 
fn main(){
    let x:Vec<i32>=vec![4,5,6];
    let y=x; // The value have been borowwed here .
    println!("THE VALUE OF Y IS {:?}",y);

}

// Simple operation on the vector  hav3 been demonstrated.
fn main(){
   
    let mut v:Vec<i8>  = vec![4,5,6];
    println!("origninal vector is {:?}",v);
    v.push(5); // This is for the insertion at the end of the vector.
    println!("origninal vector is {:?}",v);
    v.insert (1,7); //This for the insertion at the specific location .
    println!("{:?}",v);
    let remove_element=v.remove(2);
    println!("THE VALUE AFTER REMOVAL IS {:?}",remove_element);
    v.sort();
    print!("THe sort value is {:?}",v);

    
    
}
fn main(){
    let  mut c:Vec<i128>=vec![ 1,2,3,7];
    println!("The value of c is {:?}",c);
    let    d: &mut Vec<i128>=   &mut c; // Here we have passed the rference of c to the d instead of passsing the the ownership we have pssed the reference .
    println!("The value of d is {:?}",d);
    d.push(5);
    d.insert(1,700);
    d.remove(2);
    println!("The value of d after insertion  is {:?}",d);
    println!("The value of c is {:?}",c);
}
  ```
 The xtended version pv vector with hign end demonstration 
 ```Rust 
 fn main(){
    let mut vec: Vec<i32> =Vec::new();
    vec.push(8);
    vec.push(785);
    assert_eq!(vec.len(),2);
    assert_eq!(vec[0..2],[8,785]); // For the checking of the multiple value in given vector.
    print!("The value is {:?}",vec);

}
fn main(){
  let mut vec :Vec<i128>=Vec::new(); //This vector will not allocate until the data is pushed into it 
  vec.push(5);
  vec.push(6);
  assert_eq!(vec.len(),2);
  assert_eq!(vec[0..2],[5,6]);
  assert_eq!(vec.pop(),Some(5)); //This line will give eeror as acc to pop we can remove the value from end of the array.
  assert_eq!(vec.pop(),Some(6)); // This will work perfectely.
  println!("The value is {:?}",vec);
}
 let f=vec![4,5,6,7];
 println!("THe {:?}",f[3]); // Concept of indexing has been explained .
```
 `Slicing` it means to access the certain part of the vector without modifying the range of the vector.
 ```Rust
 fn main(){
    let d:vec<i8>=Vec[1,2,3,4,5,6];
    let d1=&d[0..3];
    assert_eq!(d.len(),5);
     assert_eq!(d[0..2],[1,2,3,4,5,6]);
    println!("The value is {:?}",d1);
    println!("The value is {:?}",d);
     }
     // Full fledged slice concept 
       
  fn main(){
    let d:Vec<i8>=vec![1,2,3,4,5,6];
    let d1=&d[0..3];
    assert_eq!(d.len(),6);
     assert_eq!(d[0..6],[1,2,3,4,5,6]);
    println!("The value is {:?}",d1);
    println!("The value is {:?}",d);
     }
  ```
  # Capacity 
    Capacity  simple means that the amount of space allocated for any future elemets that will be added onto the vector.
    On the other hand length is the amount of the element into the vector we alredy  have .

# Gurantees
`Vec` makes a lot of gurantees about its  design.This ensure its a low-overhead as possible in the general case,and can be correctly manipulated in primitive ways to unsafe code.
`Vec` never make gurantee about the memoery layout (Memory layout refers to the organization and arrangement of data in computer memory.)including the order of the fileds.
`ABI` is not stable here which means that Application Binary Interface. It refers to the low-level interface between two binary program modules, typically between a library and the applications or other libraries that use it. The ABI defines how functions, data structures, and system calls are represente 


`Uninit` 
It represent memory that is not initilizwd yet.

If we use iterator then there will no reallocation of memmoery .

```Rust
//Printing the vector array in loop as it avoid the reallocation of memory 
fn main(){
    let mut v: Vec<i32> =Vec::new();
 for i in 0..12{
     v.push(i*2-7)
 }
print!("{:?}",v)


}
```
# Raw pointer
It is low level construct that directly represents the memory addresses. It provide the fine grained control over the memory management but come with significant safety and complexity consideration.Even rust highly discourage the use of raw pointer due to strong safety enforced by the language.Raw pointer doesnt track the ownership and lifetime.
# Alignment types in memeory 
Memory alignment refers to the arrangement of data in memory relative to certain boundaries based on its size and the architecture of the system. It plays a crucial role in performance and efficiency, especially on modern processors with caches.
For low-level language like Rust its much more effective to use the Alignment in memory .By understandin this we can write more efficiant code and performat the code while dealing with lower level aspects .
# Overwrite in memeory 
  To change the pre wrtiteen value in memeory .It should be always done with the safe code . Rust always avoid to use the raw pointer .
  # Wrapper 

In vector memory allocation concept is quite hectic and for easy and impactfull creation of memeory  allocation we use APIs famously known as `alloc`
 As we have our default memory allocator provided by the operaating system which is known as `System`
 
 Creating A `Vec<T>` dirctly form the pointer,capacity and a length.
 Although  when it comes to safety we will find that it highly unsafe because there many thingss which arent checked in it
  - `use std::ptr`. 
     Manually manage the memeory through raw pointer 
 - `use std::mem` This module contains functions for querying the size and alignment of types, initializing and manipulating memory.
 - `as_mut_ptr`Returns an unsafe mutable pointer to the vector's buffer, or a dangling raw pointer valid for zero sized reads if the vector didn't allocate.
 - `unsafe`Code or interfaces whose memory safety cannot be verified by the type system.
 - `write`Overwrites a memory location with the given value without reading or dropping the old value.It behaviour is undefined if the its condition is violeted
 - `from_raw_parts`
 Creates a Vec<T> directly from a pointer, a capacity, and a length.
 This is highly unsafe, due to the number of invariants that aren't checked:
 ## Using memory that was allocated else where 
 Here is the code snippet which need to be revised again 
 ```Rust
 use std::alloc::{alloc,Layout};

fn main(){
    let layout = Layout::array::<u32>(16).expect("overflow cannnot happen");
    let vec = unsafe{
        let mem = alloc (layout).cast::<u32>();
        if mem.is_null(){

            return;
        }
        mem.write(4294967250);
        Vec::from_raw_parts(mem, 1, 16);
     
    };
    /*assert_eq!(vec,&[1_000_000]);
    assert_eq!(vec.capacity(),16); */
 //println!("The value is {:?}",layout)
 }
 ```
 # Module std::sync
 Sync" is short for "synchronization," which refers to the coordination of multiple processes or threads to ensure that they behave correctly and consistently in a concurrent system.
- A thread is the smallest unit of execution within a process. In simple terms, it's a sequence of instructions that can be scheduled and executed independently by the operating system's scheduler. Threads enable concurrent execution of tasks within a program, allowing different parts of the program to execute simultaneously or interleaved.
- Concurrently refers to the execution of multiple tasks or processes at the same time. In a concurrent system, multiple operations can progress simultaneously, either through true parallelism on multiple CPU cores or through interleaved execution on a single CPU core via time-sharing.
 - Note that thanks to Rust’s safety guarantees, accessing global (static) variables requires unsafe code, assuming we don’t use any of the synchronization primitives in this module.
 - Single-threaded: Executes instructions sequentially within a single thread of execution.
-  Multi-threaded: Executes instructions concurrently across multiple threads of execution, allowing for parallelism and potentially improved performance in tasks that can be divided into smaller units of work.
 Due to various reason order of execution of instruction may execute in different order.
 ### Below is the example of the single threaded exection 
 By coceptually ,a RUST program is the a series of the opearation which will execute with the order of opration in the code .
 ```Rust
 static mut A:i8=0;
static mut S:i8=45;
static mut F:i8=45;
//static mut e:i8=45;

fn main(){
  
   unsafe {
    A=8;
    S=5;
    A=A+S;
  F=S;
  println!("The value is {A}{S}{F}");
    
  F=A;
  println!("The value is {A}{S}{F}");
   }
   
}
 ```
 ## Module std::sync::atomic
  Atomic types provide  the primitiv shared memeory communication between threads and are the building block of oother concurent types.
  Atomic variable are safe to share thread and they follow the threading model of the Rust. Atomic model can be stored into static varbiles, and initilized using constant initializers.`AtomicBool::new`.
# Ordering 
 Memory ordering specify the way atmoic operation synchronize memeory.In its weakest Ordeing::Relaxed, only the memeory directely touched by the operation is synchronized.
 - The orderingg enum has the three variants:
   - `SecCst(Sequentially Consistent)`: This ordering provide the strongest gurantee.It ensure that the memeory opeartion appears to occur instantaneously at the some point between the threads presvious and subsequent opeartions.All threads observe the same total order of swquentially consistent opearations.
    - `Acquire`: This ordering ensure that the memeory read that occur after the operation canot be reordered before the operstion.
    - `Release`: This ordering ensures that memory writes that occur before the operation cannot be reordered after the operation. It establishes a happens-before relationship with preceding reads and writes in the same thread.
    `Store` Stores a value into the atomic integer.
    ```Rust
    use std::sync::atomic::{AtomicU64,Ordering};
    fn main(){
    let flag= AtomicU64::new(1000);
    flag.store(5000 , Ordering::Release);
    let value = flag.load(Ordering::Acquire);
    println!("Flag value : {}",value )
    }
    ```
# Zero Sized Types 
For rust in pratically these are the useless stuff .
# In Rust, DYNAMIC /Static means
-  Dynamic types refers to the data type whose size is known at the runtime .It type   safety is lower and performance is lower due to the runtime type.
-  Static type -> Refers to data types whose size be determined at compile time. This means the compiler know how much memory to allocate for a variable of that type.
- Default location for small, fixed-size data: Rust prioritizes stack allocation for data whose size is known at compile time. This includes:
Primitive types: Integers, floats, booleans, characters, etc.
Local variables: Variables declared within a function's scope are typically allocated on the stack.
 - Used for dynamic data or data exceeding stack size: When data has a size unknown at compile time or needs to persist beyond the scope of a function, it is allocated on the heap using specific mechanisms like:
Dynamically sized collections: Vec<T> (vector), String, HashMap<K, V>, etc., whose size can grow or shrink at runtime


# Stack 
A basic fixed-capacity stack stored statically.
 

# Queues and linked Lists 
A queue is a fundamental data structure in computer science that follows the “First In, First Out” (FIFO) principle. It stores and manages data in a specific order, where the first element added to the queue is the first one to be removed. This data structure is commonly used for tasks that require elements to be processed in the order they were added.
A Queue is linear data strucutre that defined three methods.
 - add :This is also knowm as `push` or `queue`, this add elements to the queue.
 - remove : Also known as `deque` or `pop`,this removes the oldest element from the queue.
 - peek : This show the top eleemnet in the queue scheduled for the removal .
```Rust
use queues::{IsQueue, Queue};

#[macro_use]
extern crate queues;
fn main (){
 let mut p : Queue<i64> = queue![];
  p.add(5457);
  p.add(50);
  p.add(500);
  p.add(5000);
  p.add(500);
  p.add(50000);
  p.add(5);
  p.add(531);

  p.remove();
  p.size();
  println!("The value is {:?}",p);


}
``` 
# Linked List 
 It is linear data structure in which the element are not stored in contigous memeory location rather they are linked using pointers. Linked list for the series of the connected nodes =where each nodes store the data and the addres of the next node.
 `Node` It is individual part of larger data strucuture .
  A node in a linked list contain the two componenets.
  - `Data` It holda the actual value or the data associated with the node.
  - `Next Pointer ` It store the memeory address of the next node in the sequence.
  - `Head and tail` The linked list is accessed through the head node, which points to the first node in the list.The last node in the list points to the NUll OR nullptr,indicating the end of the list.This node is known as the tail node .
 - It doesn't allow accesing of the element by using the indexing 
 - It require the extra memory for the storing the pointers,compared to the array.
 - It can grow dynamically,as memory allocation is done at the runtime.
 - It can be easily reorganized and modified without requireing a contigous block of   the memory .
  - `Insertion` at any index is possible .
  - `Deletion` at any index is possible .
  - `Searching` at any index is possible .
  - `Double-linked list` In this each node contain the reference of the both the previous and next node .
  - `Single list ` In this each node contains the refernce to the next node in the sequence .
  - `Circular linked list ` In this last node points back to the head of the node.
```Rust
use std::collections::LinkedList;
fn main () {
    let mut kiss = LinkedList::from([1,2,3]);
    println!("The value is {:?}",kiss);
    kiss.push_back(7);
    println!("The value is {:?}",kiss);
    kiss.push_front(9);
    println!("The value is {:?}",kiss);
    kiss.pop_back();
    println!("The value is {:?}",kiss);
    let mut k1:LinkedList<i8>= LinkedList::new();

    k1.push_back(  8);
    println!("The value is {:?}",k1);

}
```
# Dictionaries 
In rust we say crate implementing associative array known as dictionaries.
The dictionary in data structure is used to store the data in the form of key-value pair.
The key is the attribute that helps us locate the data or value in the memory.
The keys are always unique within a dictionary. The values of the dictionary in data structure may or may not be unique.
Dictionary in data structures are referred to by various names such as maps, symbol tables, etc.
No duplicate keys are allowed in the dictionary data structure, so whenever a duplicate key is found, the last assigned value is treated as the final key-value pair.
The key attribute is case sensitive hence "KEY" is not the same as "key".
The key attribute is immutable hence, we cannot have an array or list as keys we can only have numbers, strings, etc, as the key.
The value attribute can contain a single value or a collection of values stored in any sequential data type like lists, tuples, sets, etc.
The dictionary in data structure internally uses hashing which makes the data retrieval faster.
The dictionary data structure can store a large set of values for a specific key so it is widely used to store a heterogeneous set of data for a key.
```RUST
The code generation is being left here .

```
# Hashing 
 Hashing is the process of converting a given key into another value .
 it henerate a fixed-size output from an input of variable soze using hash function.
 A hash function creates a mapping between key and the value .
  A hash function is use to generate the new value according to the mathematical algorithm.The result of a function is known as hash value.
 It involves mapping data to a specific index in a hash table using hash function .
  A good hash function uses a one- way hashing algorithm or in other words, the hash canot be converted back into the original key .
  Some time two keys can generste the same hash. This is known as collision
# Trees
A tree data structure is defined as a collection of objects or entities known as nodes that are linked together to represent a hierarchy.
It's a non linear data structure as it does not store data in a sequential manner, but stores in a hierarchical fashion.
In the Tree data structure, the first node is known as a root node i.e. from which the tree originates. Each node contains some data and also contains references to child nodes. A root node can never have a parent node.
 A tree is recursive data strucuter that means it can call itself.
#  Dictionaries 
A dictionary is a collection of key-value pairs, where each key is unique. In Rust, dictionaries are implemented using the `HashMap` data structure. To use a `HashMap`, you first need to import it from the standard library

# Deletion 
# Quick sort
 It is a very  efficieant sortng algorithm . It works out where to divide the work.The sort phase simply sorts the two smaller problems that are generated during the partioning phase.n quicksort, we divide the array of items to be sorted into two partitions and then call the quicksort procedure recursively to sort the two partitions, ie we divide the problem into two smaller ones and conquer by solving the smaller ones. To do this, we choose a pivot element and arrange that all the items in the lower part are less than the pivot and all those in the upper part greater than it. In the most general case, we don't know anything about the items to be sorted, so that any choice of the pivot element will do - the first element is a convenient one

# Tress 
 A tress is composed of one or more nodes. A tree is always non-empty.
 # Tree root 
  Tree can be constructed from root value of type T,which can be accesed later via `root ()` and `root_mut ()`.
   ## Tree children
    Tree children can insert/delete child Nodes at front/back of its list,which is conceptual forest.
## Tree Degree
The amount of child nodes of trees is called tree;s `degree()`.
The amount of all nodes of a tree is returned bu `node_count()`.

# CRUD api of Tree
 Create, Read,Update, Delete. These function are the four pillars of complete CRUD API.
 
# Method 
It is the function that is associated with the data types.

 A binary heaps is a complete Binary Tree which is used to store data efficiently to get the max or min element based on its structure .
 A binart heap is either Min or max heap . In a Min Binary Heao,the key is at the root must be minium among all keys present in binary heap.The same property must be truee for alll nodes in binary tree.Max binary tree is simliar to MinHeap.
 # Representation Binary Heap 
 A binary heap is complete  binary tree. A binary heap is typicslly represented as an array.
 The root of the element is at the Arr[0].
 And the other element will be at the Arr[i].
 Arr[(i-1)/2] = It will return the parent node.
 Arr[(2*i)+1] = It will return the left chilf node.
 Arr[(2*i)+1] = It will return the right child of the  node.

 
# Graphs 
- Non-Linear means where data is not organised in sequential order.In this elements are connected to multiple other elements.
- In Linear means where data is organised in sequential order.
 Graph is non linear data structure where it is consist of vertices and edges.
 The vertices are the sometime refered to as nodes and the edges are lines that connects any two nodes in the graph .
 The graph is denoted by G(V,E).
 - Vertices are the fundamental units of the graph ,
 - Edges are the connection line two different node.
 - Every tree is a graph but every graph is not a tree.
 - Linked List ,Tress and Heaps are all special cases of the graphs. 
 -  There are two ways to represent the graphs .
    - Adjacency Matrix  
    - Adjacency List 
- Some basix operations of the graphs are :-
   - Insertion of Nodes/Edges in the graph - Insert node into the graph .
   - Deletion of Nodes/Edges in the graph .
   - Searching on the graphs .
   - Traversal of the graphs .
 Graphs provides graph algorithm which takes graphs created using graphs_builder as input .
 The algorithm implementations are designed to run efficiently on large- scale graphs with billions of nodes and edges .
 Rust Graphs crates are created using graphs_builder crate,which is used as cutom building block of the graphs algorithm.
It use the a Compressed Sparse-Row(CSR) data structure which is tailored for fast and concurrent access to the graph topology.
 There are two types of graphs :-
 Directed Graphs 
 Undirected Graphs 
 



Adjacency" is a term commonly used in graph theory to describe the relationship between nodes or vertices in a graph.

Two sets are called disjoint sets if they don't have any element in the common,the intersection of sets is a null set.
 - Data structure used for the union are :-
  - Array: An array of integers is called Parent[]. If we are dealing with N items, i’th element of the array represents the i’th item
  - Tress It is a Disjoint set. If two elements are in the same tree, then they are in the same Disjoint set. The root node (or the topmost node) of each tree is called the representative of the set. There is always a single unique representative of each set. A simple rule to identify a representative is if ‘i’ is the representative of a set, then Parent[i] = i. If i is not the representative of his set, then it can be found by traveling up the tree until we find the representative.
  - 
# Operation of Disjoints set
## Find

## The union
- We know that when we perform union operation on two sets, then the resultant set will contain all the elements that are present individually in each of the given sets. The union of sets that are disjoint is a little different from the usual union of sets.
-  It takes two elements as inpust and find the representatives of their sets using the `Find` operations and puts either one the tress (representing the set ) under the root node of the other tree.
# Hash Map
 Hash map allows you to store items with identifiers.They are stored in table format with the identifier being hashed using hasing algorithm.
 It is more efficient to retrive items then search tress etc.
 Hashmap key and values are stored in a bucket to specifiy the entry,this entry location is determined using Hashcode function . This hashcode function determine hash where the value is stored.

 # Using Cargo to build Our first program .
  - Cargo is use to declare the metadata of the project and build rust project.
  -  Cargo init is for the starting the project 
  - Cargo build is for the building the project 
  -  Cargo test is for runnig the test and the benchmarks
  -  Cargo search is for the searching the carates,third party-party lib' .
  - Unit testing is the silver bullets of out industry that can make all the differnec in keeping up high software quality .
  
  - In rust we use `TYPE` instead of typedef as the way to defines a new name for an existing type .
  - Alias means alternative name given to the function,data type or other entites .
  - When the varible goes out of the scope the rust call a special function by default called `drop`.
  - String is made of the three parts `ptr length and the capacity`.
# Threding
 - It refers to the ability to create and manage concurrent threads of execution within a program.
 - Threads are the lightweight units of execution that enable parralism and concurrency,allowing differents parts of a program to execute independently and simulataneously . 
 - Concurrency means enabling the multiple threads can run at the same time and share system resource such as CPU time .
 - They share memory space and the other resource with the other threads within the same process .'
 - Parallesim in computing refers to the simulatneous .
  #  TYPES
Signed means positive and negative both .
Unsigned means only positive only .
Usize and isize means number of bits in  your your type of the computer
Cast u8 as char" means "pretend u8 is a char .
We can't cast i32 as a char, but we can cast an i32 as a u8. And then we can do the same from u8 to char .
 Learnt the conversion of one data type into another one .
 
 `Rust
 use std::char;
fn main () {
    let first = "A";
    let seconf = "" ;
    let third  = 909999;
    println!("The value is {}",third as u8 as char);
} 
 `
When using characters as part of a string, the string is encoded to use the least amount of memory needed for each character.
`Rust 
fn main () {
    println!("The sizE is  the : {}",std::mem::size_of::<i128>());
}
`
Some println! need to debug .
 Display means printing with {}, and Debug means printing with {:?} .
   println!("This will print smallest the i8 {} and the biggest will be {}.",i64::MIN , i64::MAX);

- Mutability and shadow is totally different ;
- When you shadow a variable, you don't destroy it. You block it.
- In shadow  we can use the same  name different time .
```Rust
fn main ()  {
    let doesn = ();
    println!("This will not print {:#?}",doesn);
    println!("This will print smallest the i8 {} and the biggest will be {}.",i64::MIN , i64::MAX);
    let mut set = 122;
     set = 233;
     println!("The value is mutable here {} ",set);
    fn times_two (number : i32) -> i32{
        number *2

    }
    let final_number = {
        let y = 10 ;
        let x: i32 = 14;
        let x = times_two(x);
        let x = x+y;
        x
    };
    println!("The data is {:?}",final_number);
        
    }
 ```   
- Stack and the heap ant the pointer 
- Stack need th exact size
- Heap can have any size .
- A pointer is the addered of the location .
- Pointer is know as the reference and means that it point to the memory of another value . A reference means that you borrow the value ,but you dont own it .
- const is for values that don't change, the name is replaced with the value when it's used,
- static is similar to const, but has a fixed memory location and can act as a global variable.
- Refernce can be mutable to 
- Use * to change the mutable  value. This is known as the dereferencing .


# Ownership
Rust is system programming language .
Data with the size unknown at the compile time is stored at the heap .
Managing the heap data is why ownership exit .
- Rules of the owenership
  -  Each values in Rust has a variable  that is called owner .
  - There can be only one owner at a time .
  - Once the varibale go out of the scope then the value dropped .
  - String  can be muted but string literal cant  be muted . The reason is how these two deals with the memory .
  - When we know the contents at the compile time then it is hardcoded into the final executable .
  - When we dont  know the contents at the compile time then the we need to allocate the memory on the heap .
  - When the copy of the string goes out of the scope.then they both try to free the same space , this is known as the double free and it creates the bugs ,which can lead to the security and vulnerabilities .
  - Rust never create the deep copy by own .
  - For integer we have he size at the compile time only .
  - For the sting we doent have the size at the compile time ,we get it durinng the runtime .
```Rust
fn take_kasuhik (string : String) {
  println!("{:?}",string);
}
fn moves_k (value : i32) {
 println!("{:?}",value);
}

fn main () {
    let   s = String::from("Kaushik raj") ;
   
    take_kasuhik (s);

    let c= 13;
    moves_k(c);
   

}
``` 
# Reference 
```Rust

fn calculate (s :&String) -> usize {
    s.len()
}
// We took the &string without taking the string .& will allow  the referencing without taking the ownership . 
fn main () {
 let  s1 = String::from ("Kaushik");
    
 let  x = calculate(&s1);
 // Reference is the type of the data type that doesn't have the ownership .
 // Here we passed the value of the &s1 to the calculate fucntion .
 // The opposite of referencing is deferencing * . 
 // Note We are not allowed to mut the reference value . Mut can be done onlu once in the scope an for that we have to mut the value frst .
 // Rust gurantte that the reference will not be the dangling reference .

 println!("The value is {:?} The calculation is {:?}",s1 ,x);

}



```

```RUST

// Programm to calculate the number of the element it the sentence till it find the space .

fn first_word (s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return  i;
        }
        
    }
    s.len()
}


fn main () {
 let mut s = String::from("Hellworld");
 let word = first_word(&s);
 s.clear();

 println!("The value is {:?}",word);


 let mut k = String::from ("Raju ");
 let work = first_word(&k);
 s.clear();
 println!("The value is {:?}",work);
}

```
# Class
- The main goal of the class is to promote the code organization,maintainability ans resuability . By encapsulating data and behaviour and increases the security by hiding the details of information . And it helps to reuse the data and helps for scalability . 
  - Encapsulation means the bundling of the data into the one .
  - Inheritance  means that it can inherit the methods and data from the parents class .
  - Polymorphism means that  it allow the same opertaion to behave differently on the different class .
  - Modularity means that it can used to for isolating the different aspects of the system . It devide the system into the distinct modules each resposible for the specific aspect or functionality .
  - Reusability refers to the resuse if the code .

- In rust we dont have the concept of the class like other oops language have . But the same similirities can be achived with the same encapusalation .
- The concept of the virtual struct  is orthogonal .
-  Rust objects are bit smaller one as compare to other as they dont have the pointer  .


# Concurrency 
- It is the way in the programming to where multiple task are executed during the overlappping periods of time,rather then sequentially .It is the way to make programm more efficient by allowing multiple operstions to process in the parallel or in an  interleaved manner . Concurrency doesnt means that task are running at the exact same time which would be the parallelism but rather that the task make the progress indepently .
- 



