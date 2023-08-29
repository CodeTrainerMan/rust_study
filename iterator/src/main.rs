fn main() {
    let vector = vec![
        "vector 1","vector 2","vector 3"
    ];
    let mut iterator = vector.iter();
    println!("iter next value is {:?}",iterator.next());
    println!("iter next value is {:?}",iterator.next());
    println!("iter next value is {:?}",iterator.next());
    println!("iter next value is {:?}",iterator.next());

    let iter = vector.iter();
    for i in iter {
        println!(" for cycle data is {}",i)
    }
    println!("Hello, world!");
}
