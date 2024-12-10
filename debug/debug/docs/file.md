- The debug areana 
   What is Debugging in Rust?
    Debugging in Rust refers to the process of finding and resolving issues (such as logic errors, crashes, or performance bottlenecks) in your code. The log crate and env_logger crate are commonly used tools to help you log debug messages in your program.

  - Key Concepts:
    - log::debug! Macro:

  - The debug! macro is used to log debug-level messages. These messages are generally used for troubleshooting and development purposes, as they provide detailed information about the program's state.
  - The log message is formatted just like with println!, and it can     contain placeholders for variables.
  - Log Level:

  -  The debug! macro will only display messages if the log level is set to debug or lower (e.g., info, warn, error).
  -By default, the log level might be set to info, which means debug m   - messages won't be displayed unless you specifically configure the log - level.
  -Setting the Log Level:

  - To enable debug-level logging, you need to set the environment variable RUST_LOG=debug before running your program. This allows env_logger (a common logging library) to capture and display the debug messages.
```Rust
   use log::debug;
use env_logger;
fn main() {
    env_logger::init();
    let numbers = vec![1, 2, 3];
    println!("Element length is : {}", numbers.len());  
    debug!("Element at index 5: {}", numbers[5]); 


}
```
Error handling is 