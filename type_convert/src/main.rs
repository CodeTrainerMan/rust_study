fn main() {
    println!("Hello, world!");
    let my_number = MyNumber::from(1);
    println!("{:?}",my_number);

    let speed = 3;
    let my_speed:MyNumber = speed.into();


    println!("{:?}",my_speed);

     let codst:i32 = "5".parse().unwrap();
     println!("{:?}",codst);
}

#[derive(Debug)]
struct MyNumber{
    num:i32,
}

impl From<i32> for MyNumber{
    fn from(value: i32) -> Self {
        MyNumber{num:value}
    }
}
