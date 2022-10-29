use std::env::{args, Args};
fn main() {
    let mut args:Args = args();
    let first:String = args.nth(1).unwrap();
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();

    let first_num = first.parse::<f32>().unwrap();
    let second_num = second.parse::<f32>().unwrap();
    let sum: f32 = operate(operator, first_num, second_num);
    println!("{}",sum);
}

fn operate(operator: char, first_number: f32, second_number: f32)-> f32 {
    if operator == '+' {
        return first_number + second_number;
    }else if operator == '-' {
        return first_number - second_number;
    }else if operator == '*' {
        return first_number * second_number;
    }else if operator == '/' {
        return first_number / second_number;
    }else {
        0.0
    }
}
