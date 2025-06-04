
struct Rect{
    height: u32,
    width: u32
}

impl Rect{
    fn area(&self)-> u32{
        return self.height*self.width;
    }
    fn perimeter(&self)-> u32{
        return 2*(self.height + self.width);
    }
}


fn main(){
    let rect1 = Rect{
        height: 50,
        width: 100
    };

    println!("Area of rectangle is {}", rect1.area());
    println!("Perimeter of rectangle is {}", rect1.perimeter());
}