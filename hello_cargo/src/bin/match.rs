use std::io;
fn main()
{
loop{
    println!("Enter 1,2 or 3: ");
    let mut input = String::new(); 
    io::stdin().read_line(& mut input).expect("Could not read line");
    let some_num: Result<i32, std::num::ParseIntError> = input.trim().parse::<i32>();

    match some_num{
        Ok(x)=>{ match x{
        1 => {println!("it is 1!");break;},
        2 => {println!("it is 2!");break;},
        3 => {println!("it is 3!");break;},
        _ => println!("invalid input!"),
    }},
    Err(_) => println!("You entered nothing!"),

    }
}
}
