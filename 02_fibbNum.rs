fn main() {
    println!("Hello, world!");
    println!("{}", fibb_num(10));
}


fn fibb_num(num:i32) -> i32{
    let mut first: i32 = 0;
    let mut second: i32 = 1;

    if num==0 {
        return first;
    } 
    if num == 1 {
        return second;
    }

    for _ in 2..num{
        let temp: i32 = second;
        second = first+second;
        first = temp;
    }

    return second;

}