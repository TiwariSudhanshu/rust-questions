enum Shapes{
    Rectangle,
    Circle
}

fn print_area(shape: Shapes){
    println!("hii there ! inside the function");
}


fn main(){
    let shape1 = Shapes::Rectangle;
    print_area(shape1);
}