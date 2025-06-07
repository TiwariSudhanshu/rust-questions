fn main(){
    let mut vec = Vec::new();
    for i in 1..20{
        vec.push(i);
    }
    println!("Even filtered array is : {:?}", even_filter(&vec));
    println!("Vector : {:?}", vec);
}

fn even_filter (vec: &Vec<i32>) -> Vec<i32>{
    let mut new_vec = Vec::new();
    for i in vec{
        if i%2 == 0 {
            new_vec.push(*i);
        }
    }
    return new_vec;
}