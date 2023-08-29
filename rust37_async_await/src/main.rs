
use std::time::Duration;
use async_std::task::{spawn, sleep};
use std::future::Future;
use async_std::task::block_on;


#[async_std::main]
async fn main() {
    println!("Hello, world!");
    // do3();
    // do4();
    // let do3_span = spawn(do3);
    // let do4_span = spawn(do4);
    // do3_span.join().unwrap();
    // do4_span.join().unwrap();
    // let do5_async = spawn(do5());
    // do6().await;
    // do5_async.await;

    let result = block_on(go_study());
    println!("{:?}",result)
}

async fn lesson()->String{
    String::from("Rust ")
}
fn study()->impl Future<Output=String>{
     async{
        let x = lesson().await;
        x+"study target"
    }
}
fn go_study()->impl Future<Output=String>{
    let r = |x:String| async move{
        let y:String = study().await;
        y+&*x
    };
    r(String::from("can't understand"))
}
// fn do3(){
//     for i in 0..5 {
//         println!("do3 {}",i);
//         sleep(Duration::from_millis(500));
//     }
// }
// fn do4(){
//     for i in 0..5 {
//         println!("do4 {}",i);
//         sleep(Duration::from_millis(500));
//     }
// }

async fn do5(){
    for i in 0..5 {
        println!("do5 {}",i);
        sleep(Duration::from_millis(500)).await;
    }
}
async fn do6(){
    for i in 0..5 {
        println!("do6 {}",i);
        sleep(Duration::from_millis(500)).await;
    }
}