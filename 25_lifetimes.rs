fn main(){
    let ans ;
    let str1 = String::from("small");
    {
        let str2 = String::from("longer");
        ans = longest_word(&str1, &str2);
        println!("Answer is : {}", ans);
    }
}

fn longest_word <'a>(str1: &'a str, str2: &'a str) -> &'a str{
    if str1.len() > str2.len() {
        return str1;
    } else{
        return str2;
    }
}