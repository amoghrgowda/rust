use std::io::{self,Write};
fn main(){
    let mut input = String::new();
    print!("Enter a character a or b: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input_char = input.trim().chars().next();

    match input_char{
        Some(c)=>match c{
        'a'=>println!("You entered the letter: a"),
        'b'=>println!("You entered the letter: b"),
        _=>println!("Enter a valid character")
    },
    None => println!("You entered nothing!"),
    }
}