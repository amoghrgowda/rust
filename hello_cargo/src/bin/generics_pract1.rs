struct Point<T,U>{
    x:T,
    y:U,
}

impl<T,U> Point<T,U>{
    fn mixup<V,W>(self, other_point: Point<V,W>) -> Point<T,W>{
        Point{
            x:self.x,
            y:other_point.y,
        }
    }
}

fn main(){
    let p1 = Point{ x:5, y:20.0};
    let p2 = Point{ x:"Hello", y:'a'};
    let p3 = p1.mixup(p2);

    println!("p3 data is x:{} and y:{}", p3.x, p3.y);   // OP: p3 data is x:5 and y:a
}