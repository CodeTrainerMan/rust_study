fn main() {
    let total = 2166.00;
    if total>500.00 {
        println!("数据大于500：{}",total);
    }else {
        println!("数据小于等于500")
    }
    if total>200.00&&total <500.00 {
        println!("数据大于200 且 小于500:{} ",total);
    }else if total > 500.00 {
        println!("数据大于500:{} : ",total);
    }else {
        println!("数据小于200:{}",total);
    }

    let code = "10086";
    let choose = match code {
      "10010"=>"联通",
        "10086"=>"移动",
        _=>"不清楚"
    };

    println!("我们的选择是：{}",choose);
    println!("Hello, world!");
}
