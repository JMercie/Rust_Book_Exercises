use std::io;

pub fn run() {
    println!("Please enter a number to get your Fibonnaci number!");
    println!("Type \"quit\" to end the program");
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("insert a number");

        if input.trim() == "quit" {
            break;
        }

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", fibonacci(input));
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
