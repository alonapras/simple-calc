use std::io;

fn main() {
    loop {
        // Create a new string to hold user input
        let mut input:String = String::new();
    
        println!("Write expression to calculate. Or enter an empty line for exiting");

        // Read one line from STDIN
        let _ = io::stdin().read_line(&mut input);
    
        if input.trim().len() < 3 {
            println!("Bye.");
            break;
        }

        // Find operator in the input string
        let operator_location_in_input = input.find(is_operator);
        if operator_location_in_input.is_none() {
            panic!("Use one of the supported operators [+-*/]")
        }
    
        // Retrieve index of the operator in the input string
        let operator_index = operator_location_in_input.unwrap();
    
        // println!("Input: {}", input);
        // println!("Op Index: {}", operator_index);
        
        // NOTE: Slice the input to first, last parts and operator and remove/trim all the whitespace
        let first: &str = &input[0..operator_index].trim();
        let last: &str = &input[operator_index+1..input.len()-1].trim();
        
        let operator: &str = &input[operator_index..operator_index+1].trim();
    
        // Decide which operation will be used based on the operator character
        let op = match operator {
            "+" => Operation::Add,
            "-" => Operation::Subtract,
            "*" => Operation::Multiply,
            "/" => Operation::Divide,
            _ => panic!("Improper input, please enter the options that have been given")
        };
    
        // Convert operands strings to numeric values
        let a: f64 = first.parse().unwrap();
        let b: f64 = last.parse().unwrap();
    
        // println!("First: {}", first);
        // println!("Op: {}", operator);
        // println!("Last: {}", last);
    
        // Obtain function used for calculation
        let calculate = match op {
            Operation::Add => add,
            Operation::Subtract => subtract,
            Operation::Multiply => multiply,
            Operation::Divide => divide
        };
    
        // Calculate the result
        let result = calculate(a, b);
        
        println!("Result: {}", result);
    
        // Print result (formatting with limiting exponent to 6 digits)
        // println!("Result: {:.6}", result);
    }

}

enum Operation {
    Add,
    Subtract, 
    Multiply, 
    Divide,
}

fn is_operator(c:char) -> bool {
    match c {
        '+' => true,
        '-' => true,
        '/' => true,
        '*' => true,
        _ => false
    }
}

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    a / b
}
