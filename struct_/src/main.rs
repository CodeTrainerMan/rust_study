fn main() {
    println!("Hello, world!");
    let s = Study{
        name:String::from("结构体学习"),
        target:String::from("RUST"),
        spend:100
    };
    println!("结构体数据：{:?}",s);
    show(s);
    let s2 = struct_setval();

    println!("返回结构体数据 name ：{}， target：{}， spend：{}",s2.name,s2.target,s2.spend);

    println!("speed {}",s2.get_speed());


       let s3 = Study::get_instance_another(
        String::from("结构体学习333"),
        String::from("RUST33"),
        1003
    );
    println!("{:?}",s3);


    //元组结构体

    let pair = (String::from("元组结构体"),1);
    println!("元组结构体 数据 ：{:?}",pair);

    //解构元组结构体

    let (stduy,spend) = pair;
    println!("解构元组结构体数据    {}  {}",stduy,spend);

}

fn show(s:Study){
    println!("结构体数据 name ：{}， target：{}， spend：{}",s.name,s.target,s.spend);
}

fn struct_setval()->Study{
    return Study{
          name:String::from("结构体学习22"),
        target:String::from("RUST222"),
        spend:10022
    }
}

#[derive(Debug)]
struct Study{
    name:String,
    target:String,
    spend:i32
}

impl Study {
    fn get_speed(&self)->i32{
        return self.spend;
    }

    fn get_instance_another(name:String, target:String, spend:i32) -> Study {
        return Study{
            name,
            target,
            spend
        };
    }
}