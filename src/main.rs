use std::env::{args, Args};
fn main() {
    // variable in rust are immutable, if we want to change the value of the variable, we need to make sure the variable is defined as mutable using mut keyword.
    let mut args:Args = args();
    let first:String = args.nth(1).unwrap();

    // Option type variable is call the next method, so if we need to access 0 index, if we want to access the 2 index. Weirdo!


    let operator: char = args.nth(0).unwrap().chars().next().unwrap(); // This is actually 2 number index, 
    let second: String = args.nth(0).unwrap(); // And this is the 3rd index 

    // if we redeclare the variable which is already defined, rust will freed the first defined variable.
    let first = first.parse::<f32>().unwrap();
    let second = second.parse::<f32>().unwrap();
    let sum: f32 = operate(operator, first, second);
    println!("{}",output(first, second, sum, operator));
}

fn operate(operator: char, first_number: f32, second_number: f32)-> f32 {
    // Using match statement instead of if-else statement, The match syntax is Like Switch statement.
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        '/' | 'd' => first_number / second_number,
        '%' => first_number % second_number,
        // _ => 0.0 // This is the default case
        _ => panic!("Invalid operator used.")
    }
}

fn output( first: f32, second: f32, result: f32, operator: char )->String {
    return format!("{} {} {} = {}", first, operator, second, result);
}