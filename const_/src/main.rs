fn main() {
    const PI:f32 = 3.1415926;
    println!("PI is {}",PI);

    let name = "JAVA 高并发编程";
    let name  = "JAVA 微服务编程";
    println!("name is {}", name);
    const price:f32 = 0.5;
    // const price:f32 = 0.5;
    println!("常量不能被重复定义{}",price);

    static BOOK:&'static str = "666";
    println!("BOOK{}",BOOK);
    // println!("{}",str);
    println!("Hello, world!");
}
