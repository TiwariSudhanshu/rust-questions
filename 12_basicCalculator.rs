use std::io::{self, Read} ;

fn add (a:u32, b:u32) -> u32{
    a+b
}
fn sub (a:u32, b:u32) -> u32{
    a-b
}
fn mul (a:u32, b:u32) -> u32{
    a*b
}
fn div (a:u32, b:u32) -> u32{
    a/b
}



fn main(){
   let mut choice = String::new();
   println!("Enter your operator + - * / ");

    io::stdin()
    .read_line(&mut choice)
    .expect("Failed to input operator");

    println!("Enter the two variables (press enter after typing each)");

    let mut a_input = String::new();
    let mut b_input = String::new();

    io::stdin()
    .read_line(&mut a_input)
    .expect("Failed to input first variable");

    io::stdin()
    .read_line(&mut b_input)
    .expect("Failed to input second variable");

     let a: u32 = a_input.trim().parse().expect("Invalid number");
    let b: u32 = b_input.trim().parse().expect("Invalid number");


    if choice.trim() == "+"{
       println!("Addition is {}", add(a, b));
    }
    else if choice.trim() == "-"{
       println!("Subtraction is {}", sub(a, b));
    } 
    else if choice.trim() == "*"{
       println!("Multiplication is {}", mul(a, b));
    } 
    else if choice.trim() == "/"{
       println!("Division is {}", div(a, b));
    }
    else{
        println!("Invalid Choice")
    }

}