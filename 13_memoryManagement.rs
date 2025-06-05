
// Errow below because ownership of BMW changed from car 1 to car 2 .. so car1 doesn't refrence to BMW

// fn main(){
//     let car1 = String::from("BMW");
//     let car2 = car1;

//     println!("{}", car1);
//     println!("{}", car2);
// }


// Borrow and Refrencing

fn main(){
    let car1 = String::from("BMW");
    let car2 = &car1;
    println!("Car1 {}", car1);
    println!("Car2 {}", car2);
}