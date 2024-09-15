use std::io::{self,Write};
fn main(){
    let numbers = [5,10,15,20,25,30];
    let res = slice_sum(&numbers[2..4]);
    println!("\nThe sum of the slice is {res}");
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