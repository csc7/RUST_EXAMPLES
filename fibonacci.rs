// Credits:
// Andrew Pritchard
// https://medium.com/swlh/on-memoization-291fd1dd924
// Accessed on December 29th, 2021, at 19:00.
// I have copied the code for the memoize and fibonacci functions.
// fibonacci function is fib_memo2 in the original code.

// LIBRARIES
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


fn main() {
    println!("{}", fibonacci(&mut HashMap::new(), 40));
}