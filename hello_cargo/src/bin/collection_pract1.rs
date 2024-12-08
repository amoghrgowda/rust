// A program to calculate the number of times a word has been repeated in a sentence.

use std::collections::HashMap;

fn main(){
    let text = "Hello world beautiful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count = *count + 1;
    }

    for (k,v) in map{
         println!("{k}:{v}");
    }
    
}