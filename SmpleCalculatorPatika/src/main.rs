use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering ;


use std::io;




fn main() {
    let mut performed_operation = String::new();
    println!("Please enter which operation you want to perform:");

    io::stdin().read_line(&mut performed_operation)
        .expect("Failed to read input");

    let operation: Operation = match performed_operation.trim().as_ref() {
        "add" => Operation::Add,
        "subtract" => Operation::Subtract,
        "multiply" => Operation::Multiply,
        "divide" => Operation::Divide,
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    println!("Please enter two values:");

    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Failed to read input");

    let x: f64 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    let mut y = String::new();
    io::stdin().read_line(&mut y)
        .expect("Failed to read input");

    let y: f64 = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    match process_calculator(operation, x, y) {
        Some(result) => println!("Result: {}", result),
        None => println!("Error: Division by zero"),
    }
}


enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn process_calculator(operation: Operation, x: f64, y: f64) -> Option<f64> {
    match operation {
        Operation::Add => Some(x + y),
        Operation::Subtract => Some(x - y),
        Operation::Multiply => Some(x * y),
        Operation::Divide => {
            if y != 0.0 {
                Some(x / y)
            } else {
                None

            }
        }
    }
}






















