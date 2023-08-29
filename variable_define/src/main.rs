fn main() {
    let speed = 1;
    {
        println!("speed value : {}",speed);
        let speed = 2;
        println!("speed valueL{}", speed);
        let target = 8;
    }
    println!("outside speed value is：{}",speed);
    // println!("outside target value is：{}",target);
    let speed = String::from("hidden");
    println!("near speed {}",speed);
    println!("Hello, world!");
}
