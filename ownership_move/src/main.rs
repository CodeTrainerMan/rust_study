fn main() {
    /*
        栈 后进先出，类型大小固定
        堆 编译时大小不固定 容易产生内存益处
    */
    let name = "rust 学习与练习";

    let a = 88;
    let b = a;
    println!("a {},b {}",a,b);

    let str1 = vec!["数组类型转移所有权测试","能否转移成功"];
    let str2 = str1;
    println!("{:?}",str2);

    println!("Hello, world!");
    show(str2);
    let str4 = vec!["数组类型转移所有权测试","能否转移成功"];

    let str5 = show2(str4);
    println!("str5 的显示 {:?}",str5)


}

fn show2(v:Vec<&str>)->Vec<&str>{
    return v;
}

fn show(v:Vec<&str>){
    println!("方法传参 {:?}",v)
}