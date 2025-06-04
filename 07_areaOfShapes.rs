enum Shapes{
    Rectangle(f64,f64),
    Circle(f64)
}

fn calculate_area(shape:Shapes) -> f64{
    let area = match shape {
        Shapes::Rectangle(a,b ) => a*b,
        Shapes::Circle(r) => 3.14*r*r
    };
    return area;
}

fn main(){
    let rect = Shapes::Rectangle(1.0, 2.0);
    println!("Area of rectangle with dimensions 1 and 2 is {}", calculate_area(rect));
    let cir = Shapes::Circle(2.0);
    print!("Area of circle of radius 2 is {}", calculate_area(cir));
}