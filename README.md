# Rust CLI Calculator 

This is my first Rust code, as I am learning rust from freecodecamp youtube channel, this is the video link [Rust Programming Course for Beginners - Tutorial](https://youtu.be/MsocPEZBd-M)

## What I learned 
### 1st Day
- ##### Accessing array index in Rust is different from other languages.<br>
```rust
    let first_num: String = args.nth(1).unwrap();
    let second_num: String = args.nth(2).unwrap();
 ```
The follwing code will return the first variable values as it is, but the second variable value will be None type. Because in Rust, when we access a index, rust calls the next() method. Every time we call a index, the next index will be 0. The appropriate code of the above example is <br>
```rust
    let first_num: String = args.nth(1).unwrap();
    let second_num: String = args.nth(0).unwrap();
 ```

 - ##### Variable and function Type casting 
 - ##### value is wraped using single quote is ```char``` type and value wraped using double quote is ```String``` type.

 ### 2nd Day
 - ##### About Match Statement, This is like Switch statement.
    ```rust
        match operator {
            '+' => first + second
            _ => panic!("Invalid operator")
        }
    ```
    match implicitly return the result if the case matched. we can provide any number of case, but one default case is mandatory, default case written using the underscore(-). 
 - ##### We can take multiple value in one match case. 
 - ##### About panic! and format! macro. 
 - ##### If we redeclare a variable, Rust will freed the previously defined value..
 - ##### To build the project and run the build project.