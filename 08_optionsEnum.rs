

fn find_first_a (str:String)-> Option<i32>{
    for (index  , character) in str.chars().enumerate(){
        if character == 'a' {
            return Some( index as i32 );
        }
    }
    return None;
}


fn main(){
    let name = String::from("Sudhanshu");
    let index = find_first_a(name);
    match index{
        Some(value)=> println!("First index of a is : {}", value),
        None=> println!("a not found in the string")
    }
}