use std::{env, process::exit};

use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}",err);
        exit(1);
    });
    println!("Searching for: {}",conf.query);
    println!("in: {}",conf.filename);
    
    if let Err(e)=mini_grep::run(conf){
        eprintln!("There was an error reading the file: {e}");
        exit(1);
    };

}

