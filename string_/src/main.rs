fn main() {
    let s1 = String::new();
    println!("s1:{},s1-len:{}",s1,s1.len());

    let s2 = String::from("面向对象编程");
    println!("s2:{},s2-len:{}",s2,s2.len());

    let mut s3 = String::new();
    s3.push_str("高并发编程");
    println!("s3{}",s3);
    let s4 = String::from("高并发编程");
    let result = s4.replace("高并发编程","替换字符串");
    println!("result{}", result);

    let s5 = String::from("面向对象编程");
    println!("length is {}",s5);

    let s6 = "rust从入门到放弃".to_string();
    println!("s6 is {}",s6);
    let s7 = String::from("方法调用");

    show_str(s7.as_str());

    let s8 = "\t\t\tRUST智能合约编写，\n该\r\n从哪里入手";
    println!("length is {}",s8.len());
    println!("length is {}",s8.trim().len());
    let mut s9 = "\t\t\tRUST智能合约编写，该\r\n从哪里入手";
    for item in s9.split('，') {
        println!("字符串拆分：{}" ,item)
    }
    println!("Hello, world!");

    let s10 = "字符串拆分";
    for c in s10.chars() {
        println!("字符：{}",c);
    }

    let s11 = "字符串合并1".to_string();
    let s12 = "字符串合并2".to_string();
    let result = s11+&s12;
    println!("合并字符串：{}",result);
}

fn show_str(name:&str){
    println!("show word is{}",name)
}