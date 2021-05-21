use std::io;


fn main() {
    println!("Fibonacci Generator");
    println!("Type \"quit\" to end the program.");

    loop {
        let mut seq = String::new();
        println!("\nEnter sequence: ");
        io::stdin()
            .read_line(&mut seq)
            .expect("Failed to read line.");

        if seq.trim() == "quit" {
            break;
        }
        
        let seq: u32 = match seq.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", fibonacci(seq));
    }
    
}


fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}