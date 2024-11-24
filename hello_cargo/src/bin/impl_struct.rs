fn main(){
    let rect1 = Rectangle{width:10,height:5};
    let rect2 = Rectangle{width:5, height:2};
    let rect3 = Rectangle{width:20, height:15};

    println!("Can rect1 hold rect2?{}. Area of rect1 is {}", rect1.can_hold(&rect2),rect1.area());
    println!("Can rect3 hold rect1?{}. Area of rect3 is {}", rect3.can_hold(&rect1),rect3.area());
}
struct Rectangle{
    width:i32,
    height:i32,
}

impl Rectangle{
    fn can_hold(&self,other:&Rectangle)->bool{
        self.width>other.width && self.height>other.height
    }
    fn area(&self)->i32{
        self.width * self.height
    }
}
