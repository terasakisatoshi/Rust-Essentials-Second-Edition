use std::io;
use std::error;

fn main() {
    println!("Give a positive secret number: ");
    match input_num() {
        Ok(v) => println!("Input value is: {}", v),
        Err(e) => println!("Error - Please input an integer number!: {}", e)
    }
}

// try! macro
// fn input_num() -> Result<u32, Box<error::Error>> {
//     let mut input = String::new();
//     try!(io::stdin().read_line(&mut input));
//     Ok(try!(input.trim().parse()))
// }

// error propagation operator ?
fn input_num() -> Result<u32, Box<error::Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}

