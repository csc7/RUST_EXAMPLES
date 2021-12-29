// Print integers from 1 to N, printing “Fizz” if the integer is divisible
// by 3, “Buzz” if divisible by 5, and “FizzBuzz” if divisible by both 3 and 5

use std::io;

fn main() {

    
    // Welcome message showing maximum integer value for i32 type
    println!("\n");
    println!("===================");
    println!("FIZZ BUZZ MULTIPLES");
    println!("===================\n");
    println!("Enter an integer number lower than 2,147,483,647:");

    // Credit for Data Input in Rust, with error handling:
    // https://stackoverflow.com/questions/31235359/reading-an-integer-from-input-and-assigning-it-to-a-variable
    // Code modified/edited (all Ok(_) and Err(_) messages),
    // last println! deleted
    let mut number = String::new();
    match io::stdin().read_line(&mut number) {
        Ok(_) => println!("\nData entered."),
        Err(_) => {
            println!("\nFailed to read input.");
            return;
        },
    };
    let number: u32 = match number.trim().parse() {
        Ok(n) => {
            println!("\nYour number is {}\n", number);
            n
        },
        Err(_) => {
            println!("\nNot a number.");
            return;
        },
    };
    // End Data Input in Rust

    // Call Main Function of the Program
    fuzz_bizz_multiples (number);

    // End message
    println!("\n");
    println!("Input number reached ({})", number);
    println!("=========================");
    println!("\n");
}


fn fuzz_bizz_multiples(num: u32) {
    // All possible answers and answer to print
    let all_answers = ["Fizz", "Buzz", "FizzBuzz", ""];
    let mut answer = all_answers[3];

    // Check all values
    for i in 1..num+1 {
        // If divisible by both 3 and 5, assign, print and go to
        // next iteration, reseting answer to "none", as it is
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
