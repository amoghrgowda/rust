fn main(){
    let number: i32 = 5;
    //IF ELSE BLOCK
    if number<10{       //we can only evaluate a bool, not 0 or 1.
        println!("the number is less than 10!");
    }
    else if number<20{
    println!("number is less than 20");
    }
    else{
    println!("number is greater than or equal to 20!");
    }


    let condition: bool = true;
    let number = if condition {5} else {6};
    println!("{}",number);

    //LOOP
    let mut counter = 0;
    let result = loop{
        counter += 1;

        if counter == 10{
            break counter;          //THIS WILL RETURN COUNTER AFTER BREAKING LOOP
        }
        println!("Again!");
        
    println!("{}",counter);
    };
    println!("result is:{}",result);


    //while loop   
    let mut number = 3;
    while number != 0{
        println!("{}!",number);
        number -= 1;
    }
    println!("LIFTOFF!!!");


    //for loop
    let a = [10,20,30,40,50];

    for element in a.iter(){
        println!("the value is: {}", element);  //prints 10,20,30,40,50
    }

    for num in 1..4{            //last number is exclusive, here, 4 is excl.
        println!("{}", num);    
    }

}