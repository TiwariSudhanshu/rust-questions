fn main(){

    // Immutable Iterations
    immutable_itr();

    // Mutable Iterations
    mutable_itr();

    // Next Iteration
    next_itr();

    // Into Iteration (Takes ownership of the collection) simple for loop is same as into iteration
    into_itr();

}

fn immutable_itr(){
  let vec = vec![9,8,6,3,2] ;
    let itr = vec.iter();
    for i in itr{
        println!("{}",i);
    }
}


fn mutable_itr(){
    let mut vec = vec![1,2,3,4,5];
    let mut_itr = vec.iter_mut();
    for i in mut_itr{
        *i = *i + 1;
        println!("{}", i);
    }
}

fn next_itr(){
    let mut vec = vec![1,2,3,4,5];
    let mut mut_itr = vec.iter_mut();

    while let Some(value) = mut_itr.next(){
        println!("{}", value);
    }
}

fn into_itr(){
    let  vec = vec![1,2,3,4,5];
    let itr = vec.into_iter();
    for value in itr{
        println!("{}", value);
    }
}