enum Shapes{
    circle(f32),
    rectangle(f32,f32),
}

fn print_shape(shape:Shapes){
    match shape{
        Shapes::circle(radius)=>println!("The shape is a circle with radius {radius}"),
        Shapes::rectangle(width,height)=>println!("The shape is a rectangle with width {width} and height {height}")
    }
}

fn main(){
    let inp = Shapes::rectangle(10.0, 20.0);
    print_shape(inp);
}