fn main() {
    for i in 0..5 {
        println!("循环显示：{}",i);
    }
    for i in 0..=5 {
        println!("循环显示等于：{}",i)
    }
    let list = vec!["数组1","数组2"];
    for i in list.iter() {
        println!("iterator循环 ：{}",i)
    }

    for i in list.iter() {
        match i {
            &"数组1"=>println!("匹配的数据数组1{}",i),
            &"数组2"=>println!("匹配到数据2"),
            _=>println!("未匹配到结果"),
        }
        println!("iterator循环 ：{}",i)
    }
     for i in list.into_iter() {
        match i {
            //不需要&
            "数组1"=>println!("匹配的数据数组1"),
            "数组2"=>println!("匹配到数据2"),
            _=>println!("未匹配到结果"),
        }
        println!("iterator循环 ：{}",i)
    }

    //
    let mut list2 = vec!["数组1","数组2"];
    for i in list2.iter_mut(){
        *i = match i {
            &mut "数组1"=>{"匹配的数据数组1"},
            &mut "数组2"=>{"匹配到数据2"},
            _=>*i,
        };
        println!("name is {}", i)
    }
    let mut  num=0;
    while  num < 20{
        println!("num is {}",num);
        num = num +2;
    }
    loop {
        if num > 40 {
            break
        }
        println!("loop num is {}", num);
        num = num +3;
    }
    println!("Hello, world!");
}
