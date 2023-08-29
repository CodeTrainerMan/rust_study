fn main() {
    let speed =1;
    let cost = speed as f64;

    let x = 1u8;
    let y = 2u64;
    let z = 3u32;
    let i = 21;
    let f = 1.0;

    let mut vec = Vec::new();
    vec.push("aaa");
    println!("{:?}",vec);
    println!("Hello, world!");
    let myU64:MyU64 = 5;
    let other:OtherU64 = 5;
    println!("myu add otherU:{}",myU64+other)
}

type MyU64 = u64;
type OtherU64 = u64;