use std::fmt::{Display, Formatter};

fn main() {
    println!("Hello, world!");
    let t:data<i32> = data{value:100};
    println!("{}",t.value);

    let d:data<&str> = data{value:"string value"};
    println!("{}",d.value);
    let set_boot  = book{
        name:String::from("trait study"),
        id:1,
        auther:String::from("william")
    };
    set_boot.show();
    show2(set_boot);
}

struct book{
    name:String,
    id:u32,
    auther:String,
}

fn show2<T:Display>(t:T){
    println!("t:{}",t)
}
impl Display for book{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        println!("2book name is {}, book id si {}, book auther is {}",self.name,self.id,self.auther);
        let r = Result::Ok(());
        return r;
        todo!()
    }
}

trait show_book{
    fn show(&self);
}
impl show_book for book{
    fn show(&self) {
        println!("book name is {}, book id si {}, book auther is {}",self.name,self.id,self.auther);
    }
}
struct data<T>{
    value:T
}