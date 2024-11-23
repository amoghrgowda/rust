use std::io;
fn main(){
    let mut user_name = String::new();
    let mut user_age = String::new();
    let mut user_status = String::new();
    
    println!("Enter the user's name:");
    io::stdin().read_line(&mut user_name).expect("Failed to read user name");
    println!("Enter the user's age:");
    io::stdin().read_line(&mut user_age).expect("Failed to read user's age");
    println!("Enter true if the user is online otherwise, enter false:");
    io::stdin().read_line(&mut user_status).expect("Failed to read the user's status");
    let user_age = user_age.trim().parse::<i32>().expect("failed to parse user age");
    let user_status = user_status.trim().parse::<bool>().expect("Failed to parse user status");

    let user_info = User{
        acc_name:user_name.trim().to_owned(),
        age: user_age,
        online: user_status
    };

    if user_info.online{
        println!("User {} of age {} is online",user_info.acc_name,user_info.age);
    } 
    else{
        println!("User {} of age {} is offline",user_info.acc_name,user_info.age);
    }
}

struct User{
    acc_name:String,
    age: i32,
    online: bool,

}
