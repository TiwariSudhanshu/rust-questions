fn main() {
    let name = String::from("sudhanshu");
    let len = get_str_len(name);
    println!("{}", len);
}


fn get_str_len(str: String) -> usize{
    str.chars().count()
}