use std::ops::Deref;

fn main() {

    let a = 6;
    let b = Box::new(a);
    println!("b vlaue is {}", b);

    let price = 158;
    let price2 = Box::new(price);
    println!("price compare resutl {}, price smart pointer compare result {}",price==158, *price2==158);

    let x = 666;
    let y = CustomBox::new(x);
    println!("x compare with 666 {}",x==666);
    println!("x compare with *y {}",666==*y);
    println!("*y compare with 666 {}",x==*y);

    println!("Hello, world!");
}

struct CustomBox<T>{
    value:T
}
impl<T> CustomBox<T>{
    fn new(v:T)->CustomBox<T>{
        CustomBox{value:v}
    }
}

impl<T> Deref for CustomBox<T>{
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> Drop for CustomBox<T>{
    fn drop(&mut self) {
        println!("dorp print")
    }
}