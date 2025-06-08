// Methods that call next are consuming adapters because  calling them uses up the iterator

fn main(){
    let v1 = vec![1,2,3,4,5,6];
    let itr = v1.iter();
    let sum: i32 = itr.sum();
    println!("Vector is :{:?}", v1);
    println!("Sum of all elements : {}", sum);

    // for value in itr{

    // }
    // Cannot use itr because it is consumed by sum method
}