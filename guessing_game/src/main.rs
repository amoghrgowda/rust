use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    print!("Guess the number between 1 and 10:\n");
    io::stdout().flush().expect("Failed to flush input prompt");
    let secret_number = rand::thread_rng().gen_range(1..=10);
   // println!("The secret number is: {secret_number}");

loop {
    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };
    println!("You guessed : {guess}");
    match guess.cmp(&secret_number){
        Ordering::Equal=>{
            println!("You win!");
            break;
        }
        Ordering::Greater=>{
            println!("Too big! Guess again: ");
        }
        Ordering::Less=>{
            println!("Too small! Guess again: ");
        }
    }

}
}
