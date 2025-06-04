fn main() {
  let user1 = User{
    name: String::from("Sudhanshu"),
    age: 21,
    email: String::from("sudhanshutiwari9836@gmail.com"),
    is_logged_in: true
  };

  println!("Name : {}", user1.name);
}


struct User{
    name: String,
    age: u32,
    email: String,
    is_logged_in: bool
}