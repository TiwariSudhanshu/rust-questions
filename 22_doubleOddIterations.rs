fn main (){
    let mut vec = Vec::new();
    for i in 1..20{
        vec.push(i);
    }
    println!("Original vector : {:?}", vec);

    let iteration = vec.iter();
    // let odd_iteration = iteration.filter( |x| **x%2 != 0);
    // let doubled_itr = odd_iteration.map(|x| x*2);


    let new_itr = iteration.filter(|x| **x%2 != 0).map(|x| x*2);

    println!("Doubled odd elements in vector");
    for value in new_itr{
        println!("{}", value);
    }

}