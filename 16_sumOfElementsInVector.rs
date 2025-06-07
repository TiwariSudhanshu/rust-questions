use std::io::{self, stdin} ;

fn main(){
    let mut vec: Vec<i32> = Vec::new();
    println!("Enter the no of elements in vector");
    let mut choice = String::new();

    io::stdin()
    .read_line(&mut choice)
    .expect("Failed to input choice");

    let len: i32 = choice.trim().parse().expect("Failed to input choice");

    println!("Enter the five elements of vector");

    for _ in 0..len{

        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Error in reading the line");

        let value: i32 = input.trim().parse().expect("Enter a valid integer");

        vec.push(value);
 }

    let mut sum : i32 = 0;
    for i in vec{
        sum = sum+i;
    }
    println!("Sum of all elements in the vector is :  {}", sum);
}