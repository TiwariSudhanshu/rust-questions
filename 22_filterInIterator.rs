
fn main (){
    let v1 = vec! [1,2,3,4,5,6,7,8,9,10];

    let itr = v1.iter();
    let itr2 = itr.filter(|x| *x%2==0);
    for val in itr2{
        println!("{}", val);
    }
}