use std::fs::File;
use std::io::ErrorKind;

fn main(){
    let f = File::open("Hello.txt");
    let f = match f{
        Ok(file)=> file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("Hello.txt"){
                Ok(x)=>x,
                Err(e)=>panic!("There was an error {e} while creating the file."),
            }
            other_error =>panic!("Problem opening the file!{other_error}"),
        }
    };
}