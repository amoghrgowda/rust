use std::io;
fn main(){
    println!("Enter the first operand:");
    let mut na = String::new() ;
    io::stdin().read_line(&mut  na).expect("failed to read n1");
    println!("Enter the operator symbol(+,-,* or /):");
    let mut op0 = String::new();
    io::stdin().read_line(&mut op0).expect("Failed to read op");
    println!("Enter the second operand:");
    let mut nb = String::new();
    io::stdin().read_line(&mut nb).expect("failed to read n2");
    let n1 = na.trim().parse::<i32>().expect("ENTER A NUMBER AS THE OPERAND!");
    let n2 = nb.trim().parse::<i32>().expect("ENTER A NUMBER AS THE OPERAND!");
    let oper = op0.trim().chars().next();
    match oper{
        Some(x)=>{let op = x;
            let res:i32 = calc(n1,n2,op);
            println!("The result is:\n{n1}{op}{n2}={res}");},
        None=>{println!("You did not enter an operator!");},
    }
}

fn calc(n1:i32,n2:i32,op:char)->i32{
    let mut res=0;
    match op{
        '+'=>res = n1+n2,
        '-'=>res = n1-n2,
        '*'=>res = n1*n2,
        '/'=>res = n1/n2,
        _=>println!("Enter a valid operator (Choose from the given options only!")
    }
    res
}