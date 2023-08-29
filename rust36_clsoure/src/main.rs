use std::panic::resume_unwind;


fn main() {
    let add = |x,y|{x+y};
    let result = add(3,4);
    println!("use add result {}",result);
    println!("Hello, world!");
    receives_closure(add);

    let y = 8;
    receives_closure2(|x| x+y);
    let y = 1;
    receives_closure2(|x| x+y);
    let closure = return_closure();
    println!("return closure value {}",closure(88));

    let result = do1(add,8);
    println!("result is {}",result(1));
     let result2 = do2(add,8);
    println!("result is {}",result2(3));
}

fn do2<F,X,Y,Z>(f:F,x:X)->impl Fn(Y)->Z
    where F:Fn(X,Y)->Z,X:Copy{
    move |y|f(x,y)
}
fn do1<F>(f:F,x:i32)->impl Fn(i32)->i32
    where F:Fn(i32,i32)->i32{
   move |y|f(x,y)
}

fn return_closure()-> impl Fn(i32)->i32{
    |x|x+8
}

fn receives_closure<F>(closure:F)where F:Fn(i32,i32)->i32{
    {
        let result = closure(3,6);
        println!("closure at a insert data {}",result);
    }
}

fn receives_closure2<F>(closure:F)where F:Fn(i32)->i32{
    let result = closure(1);
    println!("closure resutl is {}",result);
}