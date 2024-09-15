use std::io::{self,Write};
fn main(){
    println!("Enter the number of elements in the array:");
    let mut size = String::new();
    io::stdin().read_line(&mut size).expect("Failed to read the size");
    let size: usize = size.trim().parse::<usize>().expect("Failed to parse the size to type int");
    let mut numbers = vec![0;size];

    println!("Enter {size} numbers seperated by spaces :");
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("failed to read the numbers");
    let mut inp = inp.trim().split_whitespace();
    
    if inp.clone().count()<=size {
     
     for i in 0..size{
       match inp.next(){
        Some(x)=>{
            match x.parse::<i32>(){
                Ok(num)=> numbers[i]=num,
                Err(_) => {
                    println!("invalid number: {x}");
                    return;
                }
            }
        },
        None=>{
            println!("not enough input values!!");
            return;
        }
       }
    }}
    else{println!("You entered a lot more numbers than you said you would!");return;}
    println!("Enter the position of element till which you want to find the sum of: ");
    let mut slice_index1 = String::new();
    io::stdin().read_line(&mut slice_index1).expect("Could not read the slice index"); 
    let slice_index1 = slice_index1.trim().parse::<usize>().expect("could not parse slice index");
    
    if slice_index1<=size {
    let result = slice_sum(&numbers[0..slice_index1]);
    println!("The sum of the entered is {result}");
    }
    else{println!("Cannot slice for more than the maximum positions!");return;}
}

fn slice_sum(n:&[i32])->i32{
    let mut sum = 0;
    print!("The candidates for summing are: ");
    io::stdout().flush().expect("failed to flush");
    for &item in n.iter(){
        print!("{item}, ");
        sum = sum + item;
    }
    sum
}