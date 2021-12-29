// Print integers from 1 to N, printing “Fizz” if the integer is divisible
// by 3, “Buzz” if divisible by 5, and “FizzBuzz” if divisible by both 3 and 5


// LIBRARIES
use std::io;


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


// PRINCIPAL FUNCTION, IT COMPUTES THE MULTIPLES AND
// ASSIGN THE CORRESPONDING WORD
fn fuzz_bizz_multiples(num: isize) {

    // All possible answers and answer to print
    let all_answers = ["Fizz", "Buzz", "FizzBuzz", ""];
    let mut answer = all_answers[3];

    // Check all values
    for i in 1..num+1 {
        // If divisible by both 3 and 5, assign, print and go to
        // next iteration, resetTing answer to "none", as it is
        // not necessary to check 3 and 5 individually
        if i % 3 == 0 && i % 5 == 0 {
                answer = all_answers[2];
                println!("{}: {}", i, answer);
                answer = all_answers[3];
                continue;
            }
        // Check if divisible by 3 and assign answer
        if i % 3 == 0 {
                answer = all_answers[0];
            }
        // If not by 3, check if divisible by 5 and assign answer
        if i % 5 == 0 {
                    answer = all_answers[1];
                }
        // Print answer and reset it to "none"       
        println!("{}: {}", i, answer);
        answer = all_answers[3];        
    }
}


fn main() {

    // Initialize variable that controls the execution of program
    let mut keep_running = true;

    while keep_running {
      
        // Welcome message
        println!("\n");
        println!("===================");
        println!("FIZZ BUZZ MULTIPLES");
        println!("===================\n");
        println!("Enter an integer number from 1 to N,");
        println!("(where N is limited by your system");
        println!("architecutre):");

        // Credit for Data Input in Rust, with error handling:
        // https://stackoverflow.com/questions/31235359/reading-an-integer-from-input-and-assigning-it-to-a-variable
        // Code modified/edited (all Ok(_) and Err(_) messages, bott
        // Err() "return" changed by "continue"),
        // last println! deleted
        // Accessed on December 29th, 2021.
        let mut number = String::new();
        match io::stdin().read_line(&mut number) {
            Ok(_) => println!("\nData entered.\n"),
            Err(_) => {
                println!("\nFailed to read input.\n");
                continue;
            },
        };
        let number: isize = match number.trim().parse() {
            Ok(n) => {
                println!("\nYour number is {}\n", number);
                n
            },
            Err(_) => {
                println!("\nNot a number.\n");
                continue;
            },
        };
        // End credit for Data Input in Rust

        // Using "isize" type to account for negative integers (and so give
        // the user a message to enter values from 1 to N, as requested) and
        // to limit the maximum integer value to the one given by the system
        // running the program (for signed integer).

        if number <= 0 {
            println!("Please enter a positive number smaller than");
            println!("2,147,483,647 (program designed for positive integers");
            println!("from 1 to 2,147,483,647.");
            println!("\n");
            continue;
        }

        // Call Main Function of the Program
        fuzz_bizz_multiples (number);        

        // End message
        println!("\n");
        println!("Input number reached ({})", number);
        println!("=========================");
        println!("\n");

        // Ask to continue of leave
        keep_running = stop_or_continue_program();
    }

    println!("==============");
    println!("Program Closed");
    println!("==============");
    println!("\n");
}
