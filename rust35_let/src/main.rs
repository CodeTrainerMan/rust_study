use std::option::Option::Some;

fn main() {

    let s = Some("Rust study about let");
    let s1:Option<i32> = None;
    let s2:Option<i32> = None;

    if let Some(i) = s{
        println!("have been match {:?}",i)
    }

    if let Some(i)=s1{
        println!("Match {:?}",i)
    }else {
        println!("match fail")
    }

    let flag = false;
    if let Some(i)=s2{
        println!("Match {:?}",i)
    }else if flag {
        println!("flag is true :{}",flag)
    }else {
        println!("match fail")
    }

    let mut num = Some(0);
    while let Some(i)=num{
        if i>9{
            println!("i > 9 {}",i);
            num = None;
        }else {
            println!("i is {:?} try again",i);
            num = Some(i+1)
        }

    }
    println!("Hello, world!");
}
