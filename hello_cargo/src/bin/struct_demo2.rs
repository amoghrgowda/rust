//problem statement:

/* Problem: Student Grades Management System (Simplified)

You are tasked with managing a single student's grades in three fixed subjects: Math, English, and Science, using structs. */

use std::io;
fn main(){
    println!("Enter the student's name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Could not read name");
    let mut roll_number = String::new();
    println!("Enter the student's roll number:");
    io::stdin().read_line(&mut roll_number).expect("Could not read the roll no");
    let roll_number = roll_number.trim().parse::<u32>().expect("Could not parse roll no");
    println!("Enter the student's scores!");
    println!("Math:");
    let mut math = String::new();
    io::stdin().read_line(&mut math).expect("error reading math score");
    let math:f32 = math.trim().parse().expect("err parsing math score");
    println!("Science:");
    let mut science = String::new();
    io::stdin().read_line(&mut science).expect("error reading Science score");
    let science:f32 = science.trim().parse().expect("err parsing Sci score");
    println!("English:");
    let mut eng = String::new();
    io::stdin().read_line(&mut eng).expect("error reading eng score");
    let eng:f32 = eng.trim().parse().expect("err parsing eng score");

    let student = Student{ name , roll_number, math, science, english:eng }; 
    println!("\nStudent name: {}roll_number: {}\nMath score: {}\nScience score: {}\nEnglish score: {}",student.name,student.roll_number,student.math,student.science,student.english);
}

struct Student{
    name: String,
    roll_number: u32,
    math:f32,
    science:f32,
    english:f32

}