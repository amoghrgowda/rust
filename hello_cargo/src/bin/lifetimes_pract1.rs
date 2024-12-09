fn main(){
    let some_string = String::from("Hello world");
    let substr1 = some_string.as_str();
    let substr2 = some_string.as_str();
    let result = some_fn(substr1, substr2);
    println!("works fine! result: {result}");
}

fn some_fn<'a>(x:&'a str, y:&'a str) -> &'a str{
    if x.len()>y.len(){
        x
    }
    else{
        y
    }
}