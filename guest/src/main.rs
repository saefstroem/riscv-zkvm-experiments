#![no_main]

use risc0_zkvm::guest::env;

risc0_zkvm::entry!(main);

fn main() {
    // Your computation here
    let result = fibonacci(10);
    
    // Commit the result
    env::commit(&result);
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}