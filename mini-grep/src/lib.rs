use std::error;
use std::fs;

pub fn run(conf:Config) -> Result<(),Box<dyn error::Error>>{
    let contents = fs::read_to_string(conf.filename)?;
    //println!("Contents: {contents}");
    for ele in search(&conf.query, &contents){
        println!("output: {ele}");
    }
    Ok(())

}

impl Config{
    pub fn new(args:&Vec<String>)-> Result<Config,&str>{
        if args.len()<4 && args.len()>1{

            let conf = Config {query:args[1].clone(), filename:args[2].clone() }; // try to use references with lifetimes in next refactor 
            Ok(conf)
        }
    
        else {
            Err("Enter the correct number of arguments (first arg: search key, second arg: file name/path)")
        }
}
}

pub struct Config{
    pub query: String,
    pub filename: String,
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    
    let mut res = Vec::new();
    for line in content.lines(){
        if line.contains(query){
            res.push(line);
        }
    }
    res
}

#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let content = "\
Rust:
Safe, fast, productive.
pick three.";

        assert_eq!(vec!["Safe, fast, productive."],search(query,content));

    }
}