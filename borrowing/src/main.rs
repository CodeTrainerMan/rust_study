fn main() {
    let str1 = vec!["数组借用测试"];
    let str2 = str1;
    show(&str2);
    println!("借用后数据的值打印{:?}",str2);

    let mut str3 = vec!["数组借用测试","借用方法里改变借用值"];
    println!("借用前数据：{:?}",str3);
    show_change(&mut str3);
    println!("借用后的数据：{:?}",str3);

    println!("Hello, world!");
}

fn show_change( v:&mut Vec<&str>){
    let l = v.len();
    for i in 0..l {

        v[i] = "借用后数据修改";
    }
}

fn show(v:&Vec<&str>){
    println!("借用方法的值展示：{:?}",v);
}