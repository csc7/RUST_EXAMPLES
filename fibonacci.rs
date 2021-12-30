// This program returns the Nth fibonacci number with recursion
// and memoization


// Credits for "memoize" and "fibonacci" functions:
// Andrew Pritchard
// https://medium.com/swlh/on-memoization-291fd1dd924
// Accessed on December 29th, 2021, at 19:00.
// I have copied the code for the memoize and fibonacci functions.
// fibonacci function is fib_memo2 in the original code.


// LIBRARIES
use std::io;
use std::collections::HashMap;
use std::hash::Hash;


fn memoize<A, R, F> (cache: &mut HashMap<A, R>, func: F, arg: A) -> R where
    A: Eq + Hash + Clone,
    R: Clone,
    F: Fn(&mut HashMap<A, R>, A) -> R
{
    match cache.get(&arg).map(|x| x.clone()) {
        Some(result) => result,
        None => {
            let result = func(cache, arg.clone());
            cache.insert(arg, result.clone());
            result
        }
    }
}


fn fibonacci (cache: &mut HashMap<u32, u32>, arg: u32) -> u32 {
    match arg {
        0 => 0,
        1 => 1,
        n => memoize(cache, fibonacci, n - 1) + 
             memoize(cache, fibonacci, arg - 2),
    }
}


// THIS FUNCTION CONTROLS THE EXECUTION/REPETITION OF THE PROGRAM
fn stop_or_continue_program() -> bool {
    println!("Keep running?");
    println!("Enter 'exit' to leave or anything else to continue");
    let mut answ = String::new();
    io::stdin().read_line(&mut answ).expect("Not a valid string");
    // The solution using ".trim()" (versus not using it) was found here:
    // http://danielnill.com/rust_tip_compairing_strings
    // © 2018 Daniel Nill.
    // Accessed on December 29th, 2021, at 18:23.
    if answ.trim() == "exit" {
        return false;
    }
    else {
        return true;
    }
}


fn main() {

    // Initialize variable that controls the execution of program
    let mut keep_running = true;

    while keep_running {

        // Welcome message
        println!("\n");
        println!("================================================");
        println!("FIBONACCI NUMBERS WITH RECURSION AND MEMOIZATION");
        println!("================================================\n");
        println!("Enter Nth Fibonacci number you would like to get:");

        // Credit for Data Input in Rust, with error handling:
        // https://stackoverflow.com/questions/31235359/reading-an-integer-from-input-and-assigning-it-to-a-variable
        // Code modified/edited (all Ok(_) and Err(_) messages, both
        // Err() "return" changed by "continue"),
        // last println! deleted
        // Accessed on December 29th, 2021.
        let mut number = String::new();
        match io::stdin().read_line(&mut number) {
            Ok(_) => println!("Data entered.\n"),
            Err(_) => {
                println!("Failed to read input.\n");
                continue;
            },
        };
        let number: u32 = match number.trim().parse() {
            Ok(n) => {
                println!("Your number is {}\n", number);
                n
            },
            Err(_) => {
                println!("\nNot a number.\n");
                continue;
            },
        };
        // End credit for Data Input in Rust
        

        // Call Main Function of the Program, Fibonacci Function
        let fibonacci_number = fibonacci(&mut HashMap::new(), number);    
           
        // End message
        println!("================================================");
        println!("Fibonacci Number: {}", fibonacci_number);
        println!("================================================");
        println!("\n");

        // Ask to continue of leave
        keep_running = stop_or_continue_program();
    }

    println!("==============");
    println!("Program Closed");
    println!("==============");
    println!("\n");
}
