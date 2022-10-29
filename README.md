# Rust CLI Calculator 

This is my first Rust code, as I am learning rust from freecodecamp youtube channel, this is the video link [Rust Programming Course for Beginners - Tutorial](https://youtu.be/MsocPEZBd-M)

## What I learned 
- Accessing array index in Rust is different from other languages.<br>
```rust
    let first_num: String = args.nth(1).unwrap();
    let second_num: String = args.nth(2).unwrap();
 ```
The follwing code will return the first variable values as it is, but the second variable value will be None type. Because in Rust, when we access a index, rust calls the next() method. Every time we call a index, the next index will be 0. The appropriate code of the above example is <br>
```rust
    let first_num: String = args.nth(1).unwrap();
    let second_num: String = args.nth(0).unwrap();
 ```