trait Summary{
    fn sumamrize( &self ) -> String;
}

struct User{
    name: String,
    age:  u32
}

impl Summary for User{
    fn sumamrize( &self ) -> String {
        return format!("The name is {}, and the age is {}", self.name, self.age);
    }
}

fn main(){
    let user = User{
        name: String::from("Sudhanshu"),
        age: 21
    };

    println!("{}", user.sumamrize())
}