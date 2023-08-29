fn main() {
    let double = |x|{x*2};
    let add = |a,b|{a+b};
    let x = add(1,2);
    println!("add value :{}",x);

    let y = double(5);
    println!("double vleue {}", y );

    let v =4;
    let add2 = |a|{v+a};
    println!("add2 reference value {}",add2(5));
    println!("Hello, world!");
}
