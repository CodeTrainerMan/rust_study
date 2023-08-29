fn main() {
    let arr:[&str;3]=["数组学习1","字符串数组","只填写三个数组会报错吗？"];
    let mut arr2=["数组学习1","字符串数组","只填写三个数组会报错吗？"];
    let mut arr3 = ["";4];
    for i in arr {
        println!("arr 数组循环值：{}",i);
    }

    for i in arr2 {
        println!("arr2 数组2循环值：{}",i);
    }
    arr2[0]="0";
    println!("Hello, world!");
    show_arr(arr2);

    println!("数组3的内部值：{:?}",arr3);
    modify_arr(&mut arr3);
    println!("调整后的数据3的内部值{:?}",arr3);
}

fn modify_arr(arr:&mut[&str;4]){
    let l = arr.len();
    for i in 0..l {
        arr[i] = "数组值变化,modify";
    }


}


fn show_arr( arr:[&str;3]){
    for mut i in arr {
        if i=="0" {
            i="数组值变化";
        }
        println!("方法传值数组打印:{}",i)
    }
}