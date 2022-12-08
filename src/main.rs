use std::io;

fn main() {
    loop {
        println!("Select an option below:");
        println!("1) fahrenheight to celsius converter");
        println!("2) fibonacci number");
        println!("3) twelves days of christmas");

        let mut selection = String::new();

        io::stdin()
            .read_line(&mut selection)
            .expect("Unable to retrieve selection");

        let selection: u8 = match selection.trim().parse() {
            Ok(selection) => selection,
            Err(_) => {
                println!("");
                continue;
            }
        };

        if selection == 1 {
            println!("");
            fahrenheit_prompt();
            break;
        } else if selection == 2 {
            println!("");
            fib_prompt();
            break;
        } else if selection == 3 {
            println!("");
            twelve_days_of_christmas();
            break;
        }

        println!("");
    }
}

fn twelve_days_of_christmas() {
    let days: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for n in 0..12 {
        println!("On day {} of Christmas, my true love sent to me", n + 1);
        for m in (0..=n).rev() {
            println!("{}", days[m]);
        }
        println!("\n");
    }
}

fn fahrenheit_prompt() {
    println!("Input fahrenheight to convert to celsius:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Could not retrieve input");

    let input: f64 = input.trim().parse().expect("Could not parse input");

    println!(
        "{} degrees fahrenheit is equal to {} degrees celsius",
        input,
        fahrenheit_to_celsius(input)
    );
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 0.5556
}

fn fib(n: u32) -> u128 {
    if n <= 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn fib_prompt() {
    println!("Input the index of the fibonacci number you would like to retrieve");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not retrieve input");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Number must be >= 0");
                continue;
            }
        };

        println!("Computing...");
        println!("fib({}) => {}", input, fib(input));
        break;
    }
}
