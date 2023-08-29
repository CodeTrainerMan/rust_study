use std::mem::transmute;

fn main() {
    println!("Hello, world!");
    hello();
    println!("get_name 返回结果:{}",get_name());
    println!("get_name2 返回结果:{}",get_name2());
    let mut price = 99;
    double_price(price);
    println!("外部price is {}", price);
    double_price2(&mut price);
    println!("引用数据外部值price is {}", price);

    let mut name = String::from("字符串传值");
    show_name(&mut name);
    println!("name 被传入后是否能继续输出{}",name)

}

fn hello(){
    println!("hello function");
}

fn get_name()-> String{
    return String::from("返回值函数")
}

fn get_name2()-> String{
     String::from("返回值函数")
}

fn double_price(mut price:i32){
    price = price * 2;
    println!("price is {}", price)
}
//*简引用 获取内存地址
fn double_price2(price:&mut i32){
    *price = *price *2;
    println!("引用数据内部值 {}",price);
}
fn show_name(mut name:&String){
    println!("传入name值:{}",name)
}