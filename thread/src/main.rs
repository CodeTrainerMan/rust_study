
use std::thread;
use std::time::Duration;

fn main() {
    let handler = thread::spawn(||{
        for i in 0..10 {
            println!("child thread value :{:?}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..5 {
        println!("main thread value :{:?}",i);
        thread::sleep(Duration::from_millis(1))

    }
    handler.join().unwrap();
    println!("Hello, world!");
}
