use std::collections::HashMap;

fn main(){
    let mut users: HashMap<String, u32> = HashMap::new();
    
    users.insert(String::from("Sudhanshu"), 21);
    users.insert(String::from("Messi"), 37);
    users.insert(String::from("Pedri"), 22);

    let first_user_age = users.get("Sudhanshu");

    match first_user_age {
        Some(age)=> println!("Age of first user is : {}", age),
        None => println!("User not found in db")
    }

}