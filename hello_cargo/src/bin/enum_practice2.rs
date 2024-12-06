enum Animals{
    cat,
    dog,
    rabbit,
    unknown,
}

impl Animals{
    fn make_sound(&self){
        match self{
            Animals::cat=>println!("Meow"),
            Animals::dog=>println!("Woof"),
            Animals::rabbit=>println!("..."),
            _=>println!("unknown animal sound")
        }
    }
}

fn main(){
    let kitty = Animals::cat;
    kitty.make_sound();
    let doggy = Animals::dog;
    doggy.make_sound();
    let rabbit1 = Animals::rabbit;
    rabbit1.make_sound();
    let horse = Animals::unknown;
    horse.make_sound();
}