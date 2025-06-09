use std::thread;

fn main(){

    let handle = thread::spawn( || {
        for i in 1..11{
            println!("Hii from spawned core : {}", i);
        }
    });

    

     for i in 1..11{
            println!("Hii from main core : {}", i);
        }
handle.join().unwrap();
}