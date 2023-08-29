use crate::enum_sutdy::{enum3, enum1, enum2};

fn main() {
    //enum
    let enu1 = enum_sutdy::enum1;
    println!("enum data :{:?}",enu1);
    println!("Hello, world!");

    println!("Option return data :{:?}",get_discount(55));

    print_road_map(enum3);
    print_road_map(enum1);
    print_road_map(enum2);

    let level3 = study_road_map::Name(String::from("枚举传参"));

    match level3 {
        study_road_map::Name(val)=>{
            println!("枚举匹配数据：{:?}",val)
        }
    }

}
enum study_road_map{
    Name(String),
}
fn print_road_map(r:enum_sutdy){
    match r {
        enum_sutdy::enum1=>{
            println!("enum data is {:?}",r)
        }
        enum_sutdy::enum2=>{
            println!("enum data is {:?}",r)
        }
        enum_sutdy::enum3=>{
            println!("enum data is {:?}",r)
        }
        _=>{
            println!("未匹配到数据")
        }
    }
}

fn get_discount(price:i32)->Option<bool>{
    if price>100 {
        Some(true)
    }else {
        None
    }
}
#[derive(Debug)]
enum enum_sutdy{
    enum1,
    enum2,
    enum3
}