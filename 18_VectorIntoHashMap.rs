use std::collections::HashMap;


fn group_in_hashmaps(vec: Vec<(String, i32)>) -> HashMap<String, i32>{
    let mut hash = HashMap::new();
    for (key, value) in vec{
        hash.insert(key, value);
    }
    return hash;
}

fn main(){
    let vector = vec![(String::from("Sudhanshu"), 21), (String::from("Messi"), 37), (String::from("Pedri"), 22)];
    let hm = group_in_hashmaps(vector);
    println!("{:?}", hm);
}