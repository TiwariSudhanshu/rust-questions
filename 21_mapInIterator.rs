fn main (){
    let vector = vec![1,2,3];
    let itr = vector.iter();

    let itr2 = itr.map(|x| x+1);
    println!("Vector : {:?}", vector);
    println!("Updated Iteration : ");

    for val in itr2{
        println!("{}", val);
    }

}