use std::io;
fn main(){
    println!("Enter a sentence to return the first word of:");
    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence).expect("Failed to read line");
    let result = first_word(&sentence);
    println!("The first word in the entered input is: {result}");
}

fn first_word(s: &String) ->&str {
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]

}