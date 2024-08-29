use chrono::prelude::*;

fn main() {
    println!("Hello world, from rust!");

    let now = Utc::now();
    println!("Current date and time: {}",now);
}
