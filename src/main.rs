use std::io; // For user input

// Define the Operation enum with variants for basic arithmetic operations
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Function to calculate the result based on the Operation enum variant
fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b != 0.0 {
                a / b
            } else {
                println!("Error: Division by zero!");
                std::f64::NAN // Return NaN (not a number)
            }
        }
    }
}

fn main() {
    // Prompt the user for input
    println!("Welcome to the Rust Calculator!");
    println!("Enter the first number:");

    // Read the first number
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let num1: f64 = input1.trim().parse().expect("Please enter a valid number!");

    // Prompt for the operation
    println!("Enter the operation (+, -, *, /):");

    let mut operation_input = String::new();
    io::stdin().read_line(&mut operation_input).unwrap();
    let operation = operation_input.trim();

    // Read the second number
    println!("Enter the second number:");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let num2: f64 = input2.trim().parse().expect("Please enter a valid number!");

    // Create an Operation enum instance based on the user input
    let operation_enum = match operation {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    // Calculate the result
    let result = calculate(operation_enum);

    // Print the result
    println!("The result is: {}", result);
}
