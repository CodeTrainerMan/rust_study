fn main() {


    let t =("tuple多种数据",1);

    println!("{:?}",t);
    println!("元组1{}",t.0);

    println!("{}",t.1);
    println!("Hello, world!");
    show_tuple(t);
    println!("{:?}",t);
    let (book,num) = t;
    println!("{}",book);
    println!("{}",num)
}
fn show_tuple(t:(&str,i32)){
    println!("元组数据传递{:?}",t);
}
