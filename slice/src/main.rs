use std::mem::transmute;

fn main() {
    //slice
    let mut v = Vec::new();
    v.push("切片学习");
    v.push("切片第二条数据");
    v.push("切片第三条数据");
    println!("切片数据{:?}",v);
    let  v2 =&v[1..3];
    println!("切片数据提前赋值{:?}",v2);
    show_slice(v2);
    println!("写入切片的所有权是否发生变化{:?}",v2);


     let mut v3 = Vec::new();
    v3.push("切片学习");
    v3.push("切片第二条数据");
    v3.push("切片第三条数据");
    modify_slice( &mut v3);
    println!("方法中修改切片值后 主方法中的值 ：{:?}",v3);

    println!("Hello, world!");
}

fn show_slice(s:&[&str]){
    println!("方法传入值：{:?}",s);
}

fn modify_slice(s:&mut [&str]){
    s[0] = "切片调整";
    println!("切片调整方法中的值:{:?}",s);
}